use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::io::Error as IOError;
use std::io::{stdin, stdout, Write};
use std::sync::Arc;
use std::{cmp::Eq, hash::Hash};

use clingo::{ClingoError, Control, Literal, Part, ShowType, SolveMode, SolveResult, Symbol};
use itertools::Itertools;
use thiserror::Error;

use crate::cache::CACHE;
use crate::translator::Atom;
use crate::utils::{Facets, Repr, Route, ToHashSet};

pub fn zoom(mode: &impl GoalOrientedNavigation, navigator: &mut Navigator, facet: &str) -> f32 {
    mode.zoom(navigator, facet)
}

pub fn filter(
    mode: &impl GoalOrientedNavigation,
    navigator: &mut Navigator,
    current_facets: &[Symbol],
) -> Vec<String> {
    mode.filter(navigator, current_facets)
}

fn eval_weight(
    weight: &impl Eval,
    navigator: &mut Navigator,
    facet: &str,
) -> (usize, Option<usize>) {
    weight.eval_weight(navigator, facet)
}

fn show_weight(weight: &impl Eval, navigator: &mut Navigator, facet: &str) {
    weight.show_weight(navigator, facet)
}

#[derive(Error, Debug)]
pub enum NavigatorError {
    #[error("ClingoError: ")]
    Clingo(#[from] ClingoError),
    #[error("Unwrapped None.")]
    None,
    #[error("IOError: ")]
    IO(#[from] IOError),
    #[error("Invalid input.")]
    InvalidInput(String),
}

type Result<T> = std::result::Result<T, NavigatorError>;
type Literals = HashMap<Symbol, Literal>;

pub trait Eval {
    fn eval_weight(&self, navigator: &mut Navigator, facet: &str) -> (usize, Option<usize>);
    fn show_weight(&self, navigator: &mut Navigator, facet: &str);
}

#[derive(Debug, Clone)]
pub enum Weight {
    Absolute,
    FacetCounting,
}
impl Eval for Weight {
    fn eval_weight(&self, navigator: &mut Navigator, facet: &str) -> (usize, Option<usize>) {
        let new_route = navigator.route.peek_step(&facet.to_owned()).0;
        let new_assumptions = navigator
            .parse_input_to_literals(&new_route)
            .collect::<Vec<Literal>>();

        match self {
            Weight::Absolute => {
                let current_assumptions = navigator.active_facets.clone();

                let count = navigator.count(&current_assumptions);

                let weight = count - navigator.count(&new_assumptions);
                let inverse_weight = count - weight; // w_#AS is splitting

                (weight, Some(inverse_weight))
            }
            Weight::FacetCounting => {
                let count = navigator.current_facets.len();

                let facets = navigator.inclusive_facets(&new_assumptions);
                let weight = (count - facets.len()) * 2;

                (weight, None)
            }
        }
    }
    fn show_weight(&self, navigator: &mut Navigator, facet: &str) {
        match self {
            Weight::Absolute => {
                let (weight, inverse_weight) = self.eval_weight(navigator, facet);

                let inverse_facet = match facet.starts_with('~') {
                    true => facet[1..].to_owned(),
                    _ => format!("~{}", facet),
                };

                println!("{}: {:?}", facet, weight,);
                println!(
                    "{}: {:?}",
                    inverse_facet,
                    inverse_weight.expect("computing absolue inverse weight failed.")
                );
            }
            Weight::FacetCounting => {
                let (weight, _) = self.eval_weight(navigator, facet);

                println!("{}: {:?}", facet, weight);
            }
        }
    }
}

pub trait GoalOrientedNavigation: Send + Sync {
    fn eval(&self, navigator: &mut Navigator, facet: &str) -> (usize, Option<usize>);
    fn show(&self, navigator: &mut Navigator, facet: &str);
    fn zoom(&self, navigator: &mut Navigator, facet: &str) -> f32;
    fn filter(&self, navigator: &mut Navigator, current_facets: &[Symbol]) -> Vec<String>;
}

#[derive(Debug, Clone)]
pub enum Mode {
    GoalOriented(Weight),
    StrictlyGoalOriented(Weight),
    Explore(Weight),
}
impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::GoalOriented(Weight::Absolute) => write!(f, "absolute goal-oriented mode"),
            Self::GoalOriented(Weight::FacetCounting) => {
                write!(f, "facet-counting goal-oriented mode")
            }
            Self::StrictlyGoalOriented(Weight::Absolute) => {
                write!(f, "absolute strictly-goal-oriented mode")
            }
            Self::StrictlyGoalOriented(Weight::FacetCounting) => {
                write!(f, "facet-counting strictly-goal-oriented mode")
            }
            Self::Explore(Weight::Absolute) => write!(f, "absolute explore mode"),
            Self::Explore(Weight::FacetCounting) => write!(f, "facet-counting explore mode"),
        }
    }
}
impl GoalOrientedNavigation for Mode {
    fn eval(&self, navigator: &mut Navigator, facet: &str) -> (usize, Option<usize>) {
        match self {
            Self::GoalOriented(t) => eval_weight(t, navigator, facet),
            Self::StrictlyGoalOriented(t) => eval_weight(t, navigator, facet),
            Self::Explore(t) => eval_weight(t, navigator, facet),
        }
    }
    fn show(&self, navigator: &mut Navigator, facet: &str) {
        match self {
            Self::GoalOriented(t) => show_weight(t, navigator, facet),
            Self::StrictlyGoalOriented(t) => show_weight(t, navigator, facet),
            Self::Explore(t) => show_weight(t, navigator, facet),
        }
    }
    fn zoom(&self, navigator: &mut Navigator, facet: &str) -> f32 {
        match self {
            Self::GoalOriented(Weight::FacetCounting)
            | Self::StrictlyGoalOriented(Weight::FacetCounting)
            | Self::Explore(Weight::FacetCounting) => {
                let count = (navigator.initial_facets.len() * 2) as f32;
                let weight = eval_weight(&Weight::FacetCounting, navigator, facet).0 as f32;

                weight / count
            }
            _ => {
                let count = navigator.count(&[]) as f32;
                let weight = eval_weight(&Weight::Absolute, navigator, facet).0 as f32;

                weight / count
            }
        }
    }
    fn filter(&self, navigator: &mut Navigator, current_facets: &[Symbol]) -> Vec<String> {
        let mut cache = CACHE.lock().expect("cache lock is poisoned.");

        match self {
            Self::StrictlyGoalOriented(Weight::FacetCounting) => {
                let i = std::sync::atomic::AtomicUsize::new(0);
                let start = std::time::Instant::now();
                let cr_s = navigator.route.iter().cloned().collect::<String>();

                if let Some(v) = cache.max_fc_facets.get(&cr_s) {
                    v.to_vec()
                } else {
                    let count = current_facets.len();
                    let mut data = vec![];

                    current_facets.iter().for_each(|f| {
                        let repr = f.repr();
                        let neg_repr = format!("~{}", repr);

                        let r0 = navigator.route.peek_step(repr.clone()).0;
                        let a0 = navigator
                            .parse_input_to_literals(&r0)
                            .collect::<Vec<Literal>>();
                        let w0 = count - navigator.inclusive_facets(&a0).len();

                        let r1 = navigator.route.peek_step(neg_repr.clone()).0;
                        let a1 = navigator
                            .parse_input_to_literals(&r1)
                            .collect::<Vec<Literal>>();
                        let w1 = count - navigator.inclusive_facets(&a1).len();

                        let j = i.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        println!("{:?}", j);

                        data.push((repr, w0));
                        data.push((neg_repr, w1));
                    });

                    let max = data.iter().map(|(_, w)| w).max().expect("unknown error.");

                    let fs = data
                        .iter()
                        .cloned()
                        .filter(|(_, w)| w == max)
                        .map(|(f_s, _)| f_s)
                        .collect::<Vec<String>>();

                    assert!(cache.max_fc_facets.put(cr_s, fs.clone()).is_none());

                    println!("elapsed: {:?}", start.elapsed());

                    fs
                }
            }
            Self::Explore(Weight::FacetCounting) => {
                let start = std::time::Instant::now();
                let cr_s = navigator.route.clone().iter().cloned().collect::<String>();

                if let Some(v) = cache.min_fc_facets.get(&cr_s) {
                    v.to_vec()
                } else {
                    let count = current_facets.len();
                    let mut data = vec![];

                    current_facets.iter().for_each(|f| {
                        let repr = f.repr();
                        let neg_repr = format!("~{}", repr);

                        let r0 = navigator.route.peek_step(repr.clone()).0;
                        let a0 = navigator
                            .parse_input_to_literals(&r0)
                            .collect::<Vec<Literal>>();
                        let w0 = count - navigator.inclusive_facets(&a0).len();

                        let r1 = navigator.route.peek_step(neg_repr.clone()).0;
                        let a1 = navigator
                            .parse_input_to_literals(&r1)
                            .collect::<Vec<Literal>>();
                        let w1 = count - navigator.inclusive_facets(&a1).len();

                        println!(".");

                        data.push((repr, w0));
                        data.push((neg_repr, w1));
                    });

                    let min = data.iter().map(|(_, w)| w).min().expect("unknown error.");

                    let fs = data
                        .iter()
                        .cloned()
                        .filter(|(_, w)| w == min)
                        .map(|(f_s, _)| f_s)
                        .collect::<Vec<String>>();

                    assert!(cache.min_fc_facets.put(cr_s, fs.clone()).is_none());

                    println!("elapsed: {:?}", start.elapsed());

                    fs
                }
            }
            Self::StrictlyGoalOriented(Weight::Absolute) => {
                let start = std::time::Instant::now();
                let cr_s = navigator.route.iter().cloned().collect::<String>();

                if let Some(v) = cache.max_as_facets.get(&cr_s) {
                    v.to_vec()
                } else {
                    let count = navigator.count(&navigator.active_facets.clone());

                    let mut data = vec![];

                    current_facets.iter().for_each(|f| {
                        let repr = f.repr();
                        let neg_repr = format!("~{}", repr);

                        let r0 = navigator.route.peek_step(repr.clone()).0;
                        let a0 = navigator
                            .parse_input_to_literals(&r0)
                            .collect::<Vec<Literal>>();
                        let w0 = count - navigator.count(&a0);

                        let w1 = count - w0; // w_#AS is splitting

                        data.push((repr, w0));
                        data.push((neg_repr, w1));
                    });

                    let max = data.iter().map(|(_, w)| w).max().expect("unknown error.");

                    let fs = data
                        .iter()
                        .cloned()
                        .filter(|(_, w)| w == max)
                        .map(|(f_s, _)| f_s)
                        .collect::<Vec<String>>();

                    assert!(cache.max_as_facets.put(cr_s, fs.clone()).is_none());

                    println!("elapsed: {:?}", start.elapsed());

                    fs
                }
            }
            Self::Explore(Weight::Absolute) => {
                let start = std::time::Instant::now();
                let cr_s = navigator.route.iter().cloned().collect::<String>();

                if let Some(v) = cache.min_as_facets.get(&cr_s) {
                    v.to_vec()
                } else {
                    let count = navigator.count(&navigator.active_facets.clone());

                    let mut data = vec![];

                    current_facets.iter().for_each(|f| {
                        let repr = f.repr();
                        let neg_repr = format!("~{}", repr);

                        let r0 = navigator.route.peek_step(repr.clone()).0;
                        let a0 = navigator
                            .parse_input_to_literals(&r0)
                            .collect::<Vec<Literal>>();
                        let w0 = count - navigator.count(&a0);

                        let r1 = navigator.route.peek_step(neg_repr.clone()).0;
                        let a1 = navigator
                            .parse_input_to_literals(&r1)
                            .collect::<Vec<Literal>>();
                        let w1 = count - navigator.inclusive_facets(&a1).len();

                        data.push((repr, w0));
                        data.push((neg_repr, w1));
                    });

                    let min = data.iter().map(|(_, w)| w).min().expect("unknown error.");

                    let fs = data
                        .iter()
                        .cloned()
                        .filter(|(_, w)| w == min)
                        .map(|(f_s, _)| f_s)
                        .collect::<Vec<String>>();

                    assert!(cache.min_as_facets.put(cr_s, fs.clone()).is_none());

                    println!("elapsed: {:?}", start.elapsed());

                    fs
                }
            }
            Self::GoalOriented(_) => {
                // NOTE: avoid .map
                println!("{}", navigator.current_facets);

                vec![]
            }
        }
    }
}

#[derive(Debug)]
enum EnumMode {
    Brave,
    Cautious,
}
impl From<EnumMode> for &str {
    fn from(enum_mode: EnumMode) -> Self {
        match enum_mode {
            EnumMode::Brave => "brave",
            EnumMode::Cautious => "cautious",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Navigator {
    pub(crate) logic_program: String,
    control: Arc<Control>,
    literals: Literals,
    n: usize,
    pub current_facets: Facets,
    pub(crate) initial_facets: Facets,
    pub(crate) active_facets: Vec<Literal>,
    pub(crate) route: Route,
    pace: f32,
}
impl Navigator {
    pub fn new(source: impl Into<String>, n: usize) -> Result<Self> {
        let mut ctl = Control::new(vec!["0".to_owned()])?;

        let logic_program = source.into();
        ctl.add("base", &[], &logic_program)?;
        ctl.ground(&[Part::new("base", &[])?])?;

        let n_cpus = num_cpus::get().to_string();
        ctl.configuration_mut() // activates parallel competition based search
            .map(|c| {
                c.root()
                    .and_then(|rk| c.map_at(rk, "solve.parallel_mode"))
                    .and_then(|sk| c.value_set(sk, &n_cpus))
            })??;
        let mut literals: Literals = HashMap::new();
        for atom in ctl.symbolic_atoms()?.iter()? {
            literals.insert(atom.symbol()?, atom.literal()?);
        }

        let mut solve_handle = ctl.solve(SolveMode::YIELD, &[])?;
        let sat = solve_handle.get()? == SolveResult::SATISFIABLE;
        solve_handle.close()?;

        let initial_facets = match sat {
            true => {
                ctl.configuration_mut().map(|c| {
                    c.root()
                        .and_then(|rk| c.map_at(rk, "solve.enum_mode"))
                        .and_then(|sk| c.value_set(sk, "brave"))
                })??;
                let bc = ctl
                    .all_models()?
                    .last()
                    .map(|model| model.symbols)
                    .ok_or(NavigatorError::None)?;

                ctl.configuration_mut().map(|c| {
                    c.root()
                        .and_then(|rk| c.map_at(rk, "solve.enum_mode"))
                        .and_then(|sk| c.value_set(sk, "cautious"))
                })??;
                let cc = ctl
                    .all_models()?
                    .last()
                    .map(|model| model.symbols)
                    .ok_or(NavigatorError::None)?;

                match cc.is_empty() {
                    true => Facets(bc),
                    _ => Facets(bc.difference(&cc)),
                }
            }
            _ => panic!("passed logic program is unsatisfiable."),
        };
        ctl.configuration_mut().map(|c| {
            c.root()
                .and_then(|rk| c.map_at(rk, "solve.enum_mode"))
                .and_then(|sk| c.value_set(sk, "auto"))
        })??;

        let control = Arc::new(ctl);

        Ok(Self {
            logic_program,
            control,
            literals,
            n,
            current_facets: initial_facets.clone(),
            initial_facets,
            active_facets: vec![],
            route: Route(vec![]),
            pace: 0f32,
        })
    }
    fn assume(&mut self, assumptions: &[Literal]) {
        Arc::get_mut(&mut self.control)
            .expect("control error.")
            .backend()
            .and_then(|mut b| b.assume(assumptions))
            .expect("backend assumption failed.")
    }
    fn reset_enum_mode(&mut self) {
        Arc::get_mut(&mut self.control)
            .expect("control error")
            .configuration_mut()
            .map(|c| {
                c.root()
                    .and_then(|rk| c.map_at(rk, "solve.enum_mode"))
                    .and_then(|sk| c.value_set(sk, "auto"))
            })
            .expect("resetting solve.enum-mode failed.")
            .expect("resetting solve.enum-mode failed.");
    }
    pub(crate) fn satisfiable(&mut self, assumptions: &[Literal]) -> bool {
        let ctl = Arc::get_mut(&mut self.control).expect("control error.");

        let mut solve_handle = ctl
            .solve(SolveMode::YIELD, assumptions)
            .expect("getting solve handle failed.");
        let sat = solve_handle
            .get()
            .map(|sr| sr == SolveResult::SATISFIABLE)
            .expect("getting solve result failed.");

        solve_handle.close().expect("closing solve handle failed.");

        sat
    }
    fn consequences(
        &mut self,
        enum_mode: EnumMode,
        assumptions: &[Literal],
    ) -> Option<Vec<Symbol>> {
        if !self.satisfiable(assumptions) {
            return Some(vec![]);
        }

        self.assume(assumptions);

        let ctl = Arc::get_mut(&mut self.control).expect("control error.");

        ctl.configuration_mut()
            .map(|c| {
                c.root()
                    .and_then(|rk| c.map_at(rk, "solve.enum_mode"))
                    .map(|sk| c.value_set(sk, enum_mode.into()))
                    .ok()
            })
            .ok()?;

        let consequences = ctl.all_models().ok()?.last().map(|model| model.symbols);

        self.reset_enum_mode();

        consequences
    }
    // TODO: warn user or remove whitespaces between args
    fn literal(&self, str: impl AsRef<str> + Display) -> Result<Literal> {
        let s = str.as_ref();
        let negative_prefixes = &['~']; //

        match negative_prefixes.iter().any(|p| s.starts_with(*p)) {
            true => Atom(&s[1..])
                .parse(negative_prefixes)
                .map(|s| self.literals.get(&s))
                .flatten()
                .map(|l| l.negate())
                .ok_or_else(|| NavigatorError::InvalidInput(format!("unknown literal: {}", str))),
            _ => Atom(s)
                .parse(negative_prefixes)
                .map(|s| self.literals.get(&s))
                .flatten()
                .cloned()
                .ok_or_else(|| NavigatorError::InvalidInput(format!("unknown literal: {}", str))),
        }
    }
    pub(crate) fn inclusive_facets(&mut self, assumptions: &[Literal]) -> Facets {
        let bc = self
            .consequences(EnumMode::Brave, assumptions)
            .expect("BC computation failed.");
        let cc = self
            .consequences(EnumMode::Cautious, assumptions)
            .expect("CC computation failed.");

        match cc.is_empty() {
            true => Facets(bc),
            _ => Facets(bc.difference(&cc)),
        }
    }
    fn count(&mut self, assumptions: &[Literal]) -> usize {
        self.assume(assumptions);

        let ctl = Arc::get_mut(&mut self.control).expect("control error.");

        let count = ctl
            .all_models()
            .map(|models| models.count())
            .expect("counting solutions failed.");

        count
    }
    pub(crate) fn current_route_is_maximal_safe(&mut self) -> bool {
        let route = self
            .parse_input_to_literals(&self.route.0)
            .collect::<Vec<Literal>>();
        match self.satisfiable(&route) {
            false => false,
            _ => {
                let facets = self.inclusive_facets(&route);
                facets.is_empty() // NOTE: closed world assumption for any, all
                    || facets.to_strings().all(|s| {
                        !self.satisfiable(
                            &self
                                .parse_input_to_literals(&self.route.peek_step(&s).0)
                                .collect::<Vec<Literal>>(),
                        )
                    })
            }
        }
    }
    pub(crate) fn update(&mut self) {
        let initial_count = (self.initial_facets.len() * 2) as f32;

        let assumptions = self.active_facets.clone();
        let new_facets = self.inclusive_facets(&assumptions);
        let new_count = (new_facets.len() * 2) as f32;

        self.current_facets = new_facets;
        self.pace = (initial_count - new_count) / initial_count;
    }
    pub fn navigate(&mut self) {
        dbg!(&self.route);
        self.assume(&self.active_facets.clone());

        let ctl = Arc::get_mut(&mut self.control).expect("control error.");

        let mut iter = ctl
            .all_models()
            .expect("solving failed.")
            .map(|model| model.symbols);

        println!();

        match iter.next() {
            Some(model) => {
                println!("Answer 1: ");
                for atom in model {
                    print!(
                        "{} ",
                        atom.to_string()
                            .expect("Symbol to String conversion failed.")
                    );
                }
                println!();

                for (i, model) in iter.enumerate() {
                    println!("Answer {:?}: ", i + 2);
                    for atom in model {
                        print!(
                            "{} ",
                            atom.to_string()
                                .expect("Symbol to String conversion failed.")
                        );
                    }
                    println!();
                }
                println!("SATISFIABLE\n");
            }
            _ => println!("UNSATISFIABLE\n"),
        }
    }
    pub fn navigate_n(&mut self, n: Option<usize>) {
        match n == Some(0) {
            true => self.navigate(),
            _ => {
                let route = self.active_facets.clone();
                self.assume(&route);

                let ctl = Arc::get_mut(&mut self.control).expect("control error.");

                let mut handle = ctl
                    .solve(SolveMode::YIELD, &route)
                    .expect("solving failed.");

                println!();

                match handle.get().expect("getting first solve result failed.")
                    != SolveResult::SATISFIABLE
                {
                    true => println!("UNSATISFIABLE\n"),
                    _ => {
                        while let Some(model) = handle.model().expect("getting model failed.") {
                            let i = model.number().expect("getting model number failed.");

                            println!("Answer {:?}: ", i);
                            for atom in model
                                .symbols(ShowType::SHOWN)
                                .expect("getting Symbols failed.")
                                .iter()
                            {
                                print!(
                                    "{} ",
                                    atom.to_string()
                                        .expect("Symbol to String conversion failed.")
                                );
                            }
                            println!();

                            if i == n.unwrap_or(self.n) as u64 {
                                println!("SATISFIABLE\n");

                                handle.close().expect("closing solve handle failed.");

                                return;
                            }

                            handle.resume().expect("solve handle failed resuming.");
                        }

                        println!("SATISFIABLE\n");

                        handle.close().expect("closing solve handle failed.");
                    }
                }
            }
        }
    }
    pub(crate) fn parse_input_to_literals<'a, S>(
        &'a self,
        input: &'a [S],
    ) -> impl Iterator<Item = Literal> + 'a
    where
        S: AsRef<str> + Display,
    {
        input
            .iter()
            .map(move |s| self.literal(s))
            .filter_map(Result::ok)
    }
    pub fn activate(&mut self, facets: &[String]) {
        // TODO: suboptimal
        facets.iter().for_each(|s| {
            let r = self.literal(s);
            match r.is_ok() {
                true => self.route.activate(s.to_owned()),
                _ => println!("\n{:?}\n", r),
            }
        });

        let mut new_active_facets = self.active_facets.clone();

        new_active_facets.extend(self.parse_input_to_literals(facets));
        self.active_facets = new_active_facets;

        self.update();
    }
    pub fn deactivate_any<S>(&mut self, facets: &[S])
    where
        S: Repr + Eq + Hash,
    {
        facets.iter().unique().for_each(|f| {
            self.route.deactivate_any(f.repr()).iter().for_each(|pos| {
                self.active_facets.remove(*pos);
            });
        });

        self.update();
    }
    pub fn user_input(&self) -> String {
        let mut user_input = String::new();
        stdout()
            .flush()
            .and_then(|_| stdin().read_line(&mut user_input))
            .expect("IO operation failed.");

        user_input.trim().to_owned()
    }
    pub fn info(&self) {
        print!("{}", self.route);
        print!(" [ {:?}% ] ~> ", (self.pace * 100f32).round() as usize);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const PI_1: &str = "a;b. c;d :- b. e.";
    const QUEENS: &str = "
    #const n = 30.
    {q(I ,1..n)} == 1 :- I = 1..n.
    {q(1..n, J)} == 1 :- J = 1..n.
    :- {q(D-J, J)} >= 2, D = 2..2*n.
    :- {q(D+J, J)} >= 2, D = 1-n..n-1.
    ";
    const GRID: &str = "
    cell(1..U) :- U=9.
    obj(1..9).
    {set_obj_cell(X,C) : obj(X)} = 1 :- cell(C).
    :- set_obj_cell(X,C1), set_obj_cell(X,C2), C1!=C2.";
    const UNSAT: &str = "p. not p.";

    #[test]
    fn new() -> Result<()> {
        let nav = Navigator::new(PI_1, 0)?;
        assert_eq!(nav.initial_facets.len(), 4);
        assert_eq!(nav.initial_facets, nav.current_facets);

        let nav = Navigator::new(QUEENS, 0)?;
        assert_eq!(nav.initial_facets.len(), 900);
        assert_eq!(nav.initial_facets, nav.current_facets);

        let nav = Navigator::new(GRID, 0)?;
        assert_eq!(nav.initial_facets.len(), 81);
        assert_eq!(nav.initial_facets, nav.current_facets);

        Ok(())
    }

    #[should_panic(expected = "passed logic program is unsatisfiable.")]
    #[test]
    fn new_panic() {
        let _ = Navigator::new(UNSAT, 0).unwrap();
    }

    #[test]
    fn literal() -> Result<()> {
        let nav = Navigator::new(GRID, 0)?;

        assert!(nav.literal("obj(1)").is_ok());
        assert!(nav.literal("cell(1)").is_ok());
        assert!(nav.literal("set_obj_cell(1,1)").is_ok());

        assert!(nav.literal("ojb(1)").is_err());
        assert!(nav.literal("clel(1)").is_err());
        assert!(nav.literal("set_obj_cell(1, 1)").is_err());

        assert!(nav.literal(" obj(1)").is_ok());
        assert!(nav.literal("cell(1) ").is_ok());
        assert!(nav.literal(" set_obj_cell(1,1)").is_ok());

        Ok(())
    }
}
