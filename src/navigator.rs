use std::collections::HashMap;
use std::fmt::Debug;
use std::io::Error as IOError;
use std::io::{stdin, stdout, Write};
use std::sync::Arc;
use std::time::Instant;
use std::{cmp::Eq, hash::Hash};

use clingo::{ClingoError, Control, Literal, Part, ShowType, SolveMode, SolveResult, Symbol};
use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use itertools::Itertools;
use thiserror::Error;

use crate::cache::CACHE;
use crate::translator::Atom;
use crate::utils::{Facets, Repr, Route, ToHashSet};

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

fn show_all_weights(weight: &impl Eval, navigator: &mut Navigator) {
    weight.show_all_weights(navigator)
}

fn eval_zoom(weight: &impl Eval, navigator: &mut Navigator, facet: &str) -> (f32, Option<f32>) {
    weight.eval_zoom(navigator, facet)
}

fn show_all_zooms(weight: &impl Eval, navigator: &mut Navigator) {
    weight.show_all_zooms(navigator)
}

fn show_zoom(weight: &impl Eval, navigator: &mut Navigator, facet: &str) {
    weight.show_zoom(navigator, facet)
}

fn find_facet_with_zoom_higher_than(
    weight: &impl Eval,
    navigator: &mut Navigator,
    bound: f32,
) -> Option<String> {
    weight.find_with_zoom_higher_than(navigator, bound)
}

fn find_facet_with_zoom_lower_than(
    weight: &impl Eval,
    navigator: &mut Navigator,
    bound: f32,
) -> Option<String> {
    weight.find_with_zoom_lower_than(navigator, bound)
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
    fn show_all_weights(&self, navigator: &mut Navigator);
    fn eval_zoom(&self, navigator: &mut Navigator, facet: &str) -> (f32, Option<f32>);
    fn show_zoom(&self, navigator: &mut Navigator, facet: &str);
    fn show_all_zooms(&self, navigator: &mut Navigator);
    fn find_with_zoom_higher_than(&self, navigator: &mut Navigator, bound: f32) -> Option<String>;
    fn find_with_zoom_lower_than(&self, navigator: &mut Navigator, bound: f32) -> Option<String>;
}

#[derive(Debug, Clone)]
pub enum Weight {
    Absolute,
    FacetCounting,
}
// TODO: route as int vec
impl Eval for Weight {
    fn eval_weight(&self, navigator: &mut Navigator, facet: &str) -> (usize, Option<usize>) {
        let new_route = navigator.route.peek_step(&facet.to_owned()).0;
        let new_assumptions = navigator
            .parse_input_to_literals(&new_route)
            .collect::<Vec<Literal>>();

        match self {
            Weight::Absolute => {
                let mut cache = CACHE.lock().expect("cache lock is poisoned.");
                let cr_s = navigator.route.iter().cloned().collect::<String>();

                let count = if let Some(c) = cache.as_counts.get(&cr_s) {
                    *c
                } else {
                    let c = navigator.count(&navigator.active_facets.clone());

                    assert!(cache.as_counts.put(cr_s, c).is_none());

                    c
                };

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
        let f = match facet.starts_with('~') {
            true => &facet[1..],
            _ => facet,
        };

        if Atom(f)
            .parse(&[])
            .map(|s| navigator.current_facets.0.contains(&s))
            == Some(false)
        {
            println!("\ncurrently fasb only supports ?w for current facets: {:?} is not a current facet.\n", facet);
            return;
        }

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
                    inverse_weight.expect("computing absolute inverse weight failed.")
                );
            }
            Weight::FacetCounting => {
                let (weight, _) = self.eval_weight(navigator, facet);

                println!("{}: {:?}", facet, weight);
            }
        }
    }
    fn show_all_weights(&self, navigator: &mut Navigator) {
        if navigator.current_facets.0.is_empty() {
            println!("\nno current facets.\n");
            return;
        }

        match self {
            Weight::Absolute => navigator
                .current_facets
                .clone()
                .iter()
                .for_each(|f| self.show_weight(navigator, &f.repr())),
            Weight::FacetCounting => navigator.current_facets.clone().iter().for_each(|f| {
                self.show_weight(navigator, &f.repr());
                self.show_weight(navigator, &f.exclusive_repr());
            }),
        }
    }
    fn eval_zoom(&self, navigator: &mut Navigator, facet: &str) -> (f32, Option<f32>) {
        let new_route = navigator.route.peek_step(&facet.to_owned()).0;
        let new_assumptions = navigator
            .parse_input_to_literals(&new_route)
            .collect::<Vec<Literal>>();

        match self {
            Weight::Absolute => {
                let mut cache = CACHE.lock().expect("cache lock is poisoned.");
                let cr_s = navigator.route.iter().cloned().collect::<String>();

                let count = if let Some(c) = cache.as_counts.get(&cr_s) {
                    *c
                } else {
                    let c = navigator.count(&navigator.active_facets.clone());

                    assert!(cache.as_counts.put(cr_s, c).is_none());

                    c
                };

                let initial_count = if let Some(c) = cache.as_counts.get(&"".to_owned()) {
                    *c
                } else {
                    let c = navigator.count(&navigator.active_facets.clone());

                    assert!(cache.as_counts.put("".to_owned(), c).is_none());

                    c
                };
                let pace = (initial_count - count) as f32 / initial_count as f32;

                (
                    (initial_count - navigator.count(&new_assumptions)) as f32
                        / initial_count as f32
                        - pace,
                    Some(
                        (initial_count - (count - navigator.count(&new_assumptions))) as f32
                            / initial_count as f32
                            - pace,
                    ),
                )
            }
            Weight::FacetCounting => {
                let initial_count = navigator.initial_facets.len() * 2;
                let new_count = navigator.inclusive_facets(&new_assumptions).len() * 2;

                (
                    (initial_count - new_count) as f32 / initial_count as f32 - navigator.pace,
                    None,
                )
            }
        }
    }
    fn show_zoom(&self, navigator: &mut Navigator, facet: &str) {
        let f = match facet.starts_with('~') {
            true => &facet[1..],
            _ => facet,
        };

        if Atom(f)
            .parse(&[])
            .map(|s| navigator.current_facets.0.contains(&s))
            == Some(false)
        {
            println!("\ncurrently fasb only supports ?z for current facets: {:?} is not a current facet.\n", facet);
            return;
        }

        match self {
            Weight::Absolute => {
                let (z0, z1) = self.eval_zoom(navigator, facet);

                let inverse_facet = match facet.starts_with('~') {
                    true => facet[1..].to_owned(),
                    _ => format!("~{}", facet),
                };

                println!("{} : {:.4}%", facet, z0 * 100f32);
                println!(
                    "{} : {:.4}%",
                    inverse_facet,
                    z1.expect("unknown error.") * 100f32
                );
            }
            Weight::FacetCounting => {
                let (z, _) = self.eval_zoom(navigator, facet);

                println!("{} : {:.4}%", facet, z * 100f32);
            }
        }
    }
    fn show_all_zooms(&self, navigator: &mut Navigator) {
        if navigator.current_facets.0.is_empty() {
            println!("\nno current facets.\n");
            return;
        }

        match self {
            Weight::Absolute => navigator
                .current_facets
                .clone()
                .iter()
                .for_each(|f| self.show_zoom(navigator, &f.repr())),
            Weight::FacetCounting => navigator.current_facets.clone().iter().for_each(|f| {
                self.show_zoom(navigator, &f.repr());
                self.show_zoom(navigator, &f.exclusive_repr());
            }),
        }
    }
    fn find_with_zoom_higher_than(&self, navigator: &mut Navigator, bound: f32) -> Option<String> {
        match self {
            Self::Absolute => {
                let mut data = vec![];

                navigator.current_facets.clone().iter().for_each(|f| {
                    let (fr, fer) = (f.repr(), f.exclusive_repr());
                    let (z0, z1) = self.eval_zoom(navigator, &fr);

                    data.push((fr, z0));
                    data.push((fer, z1.expect("unknown error.")));
                });

                data.iter()
                    .find(|(_, z)| *z >= bound)
                    .map(|(f, _)| f)
                    .cloned()
            }
            Self::FacetCounting => {
                match navigator
                    .current_facets
                    .clone()
                    .iter()
                    .map(|f| f.repr())
                    .find(|f| self.eval_zoom(navigator, f).0 >= bound)
                {
                    Some(f) => Some(f),
                    _ => navigator
                        .current_facets
                        .clone()
                        .iter()
                        .map(|f| format!("~{}", f.repr()))
                        .find(|f| self.eval_zoom(navigator, f).0 >= bound),
                }
            }
        }
    }
    fn find_with_zoom_lower_than(&self, navigator: &mut Navigator, bound: f32) -> Option<String> {
        match self {
            Self::Absolute => {
                let mut data = vec![];

                navigator.current_facets.clone().iter().for_each(|f| {
                    let (fr, fer) = (f.repr(), f.exclusive_repr());
                    let (z0, z1) = self.eval_zoom(navigator, &fr);

                    data.push((fr, z0));
                    data.push((fer, z1.expect("unknown error.")));
                });

                data.iter()
                    .find(|(_, z)| *z <= bound)
                    .map(|(f, _)| f)
                    .cloned()
            }
            Self::FacetCounting => {
                match navigator
                    .current_facets
                    .clone()
                    .iter()
                    .map(|f| format!("~{}", f.repr()))
                    .find(|f| self.eval_zoom(navigator, f).0 <= bound)
                {
                    Some(f) => Some(f),
                    _ => navigator
                        .current_facets
                        .clone()
                        .iter()
                        .map(|f| f.repr())
                        .find(|f| self.eval_zoom(navigator, f).0 <= bound),
                }
            }
        }
    }
}

pub trait GoalOrientedNavigation: Send + Sync {
    fn eval_w(&self, navigator: &mut Navigator, facet: &str) -> (usize, Option<usize>);
    fn show_w(&self, navigator: &mut Navigator, facet: &str);
    fn show_a_w(&self, navigator: &mut Navigator);
    fn eval_z(&self, navigator: &mut Navigator, facet: &str) -> (f32, Option<f32>);
    fn show_z(&self, navigator: &mut Navigator, facet: &str);
    fn show_a_z(&self, navigator: &mut Navigator);
    fn find_with_zh(&self, navigator: &mut Navigator, bound: f32) -> Option<String>;
    fn find_with_zl(&self, navigator: &mut Navigator, bound: f32) -> Option<String>;
    fn filter(&self, navigator: &mut Navigator, current_facets: &[Symbol]) -> Vec<String>;
}

#[derive(Debug, Clone)]
pub enum Mode {
    GoalOriented(Weight),
    StrictlyGoalOriented(Weight),
    Explore(Weight),
    Io(u8),
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
            Self::Io(_) => panic!(),
        }
    }
}
impl GoalOrientedNavigation for Mode {
    fn eval_w(&self, navigator: &mut Navigator, facet: &str) -> (usize, Option<usize>) {
        match self {
            Self::GoalOriented(t) => eval_weight(t, navigator, facet),
            Self::StrictlyGoalOriented(t) => eval_weight(t, navigator, facet),
            Self::Explore(t) => eval_weight(t, navigator, facet),
            Self::Io(_) => panic!(),
        }
    }
    fn show_w(&self, navigator: &mut Navigator, facet: &str) {
        match self {
            Self::GoalOriented(t) => show_weight(t, navigator, facet),
            Self::StrictlyGoalOriented(t) => show_weight(t, navigator, facet),
            Self::Explore(t) => show_weight(t, navigator, facet),
            Self::Io(_) => panic!(),
        }
    }
    fn show_a_w(&self, navigator: &mut Navigator) {
        match self {
            Self::GoalOriented(t) => show_all_weights(t, navigator),
            Self::StrictlyGoalOriented(t) => show_all_weights(t, navigator),
            Self::Explore(t) => show_all_weights(t, navigator),
            Self::Io(_) => panic!(),
        }
    }
    fn eval_z(&self, navigator: &mut Navigator, facet: &str) -> (f32, Option<f32>) {
        match self {
            Self::GoalOriented(t) => eval_zoom(t, navigator, facet),
            Self::StrictlyGoalOriented(t) => eval_zoom(t, navigator, facet),
            Self::Explore(t) => eval_zoom(t, navigator, facet),
            Self::Io(_) => panic!(),
        }
    }
    fn show_z(&self, navigator: &mut Navigator, facet: &str) {
        match self {
            Self::GoalOriented(t) => show_zoom(t, navigator, facet),
            Self::StrictlyGoalOriented(t) => show_zoom(t, navigator, facet),
            Self::Explore(t) => show_zoom(t, navigator, facet),
            Self::Io(_) => panic!(),
        }
    }
    fn show_a_z(&self, navigator: &mut Navigator) {
        match self {
            Self::GoalOriented(t) => show_all_zooms(t, navigator),
            Self::StrictlyGoalOriented(t) => show_all_zooms(t, navigator),
            Self::Explore(t) => show_all_zooms(t, navigator),
            Self::Io(_) => panic!(),
        }
    }
    fn find_with_zh(&self, navigator: &mut Navigator, bound: f32) -> Option<String> {
        match self {
            Self::GoalOriented(t) => find_facet_with_zoom_higher_than(t, navigator, bound),
            Self::StrictlyGoalOriented(t) => find_facet_with_zoom_higher_than(t, navigator, bound),
            Self::Explore(t) => find_facet_with_zoom_higher_than(t, navigator, bound),
            Self::Io(_) => panic!(),
        }
    }
    fn find_with_zl(&self, navigator: &mut Navigator, bound: f32) -> Option<String> {
        match self {
            Self::GoalOriented(t) => find_facet_with_zoom_lower_than(t, navigator, bound),
            Self::StrictlyGoalOriented(t) => find_facet_with_zoom_lower_than(t, navigator, bound),
            Self::Explore(t) => find_facet_with_zoom_lower_than(t, navigator, bound),
            Self::Io(_) => panic!(),
        }
    }
    fn filter(&self, navigator: &mut Navigator, current_facets: &[Symbol]) -> Vec<String> {
        let mut cache = CACHE.lock().expect("cache lock is poisoned.");

        match self {
            // TODO:
            Self::StrictlyGoalOriented(Weight::FacetCounting) => {
                let cr_s = navigator.route.iter().cloned().collect::<String>();
                let count = current_facets.len();

                if let Some(v) = cache.max_fc_facets.get(&cr_s) {
                    println!("navigation mode : {}", self);
                    println!("filtered        : {:?}/{:?}", v.len(), count * 2);
                    println!("elapsed         : cached result\n");

                    v.to_vec()
                } else {
                    let mut data = vec![];

                    let pbs = ProgressStyle::default_bar()
                        .template("solving [{elapsed_precise}] {bar:30} {msg}")
                        .progress_chars("##-");
                    let pb = ProgressBar::new(count as u64);
                    pb.set_style(pbs);
                    pb.enable_steady_tick(10);

                    current_facets
                        .iter()
                        .progress_with(pb.clone())
                        .for_each(|f| {
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

                    println!("navigation mode : {}", self);
                    println!("filtered        : {:?}/{:?}", fs.len(), count * 2);
                    println!("elapsed         : {:?}\n", pb.elapsed());

                    fs
                }
            }
            Self::Explore(Weight::FacetCounting) => {
                let cr_s = navigator.route.clone().iter().cloned().collect::<String>();
                let count = current_facets.len();

                if let Some(v) = cache.min_fc_facets.get(&cr_s) {
                    println!("navigation mode : {}", self);
                    println!("filtered        : {:?}/{:?}", v.len(), count * 2);
                    println!("elapsed         : cached result\n");

                    v.to_vec()
                } else {
                    let mut data = vec![];

                    let pbs = ProgressStyle::default_bar()
                        .template("solving [{elapsed_precise}] {bar:30} {msg}")
                        .progress_chars("##-");
                    let pb = ProgressBar::new(count as u64);
                    pb.set_style(pbs);
                    pb.enable_steady_tick(10);

                    current_facets
                        .iter()
                        .progress_with(pb.clone())
                        .for_each(|f| {
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

                    println!("navigation mode : {}", self);
                    println!("filtered        : {:?}/{:?}", fs.len(), count * 2);
                    println!("elapsed         : {:?}\n", pb.elapsed());

                    fs
                }
            }
            Self::StrictlyGoalOriented(Weight::Absolute) => {
                let cr_s = navigator.route.iter().cloned().collect::<String>();

                if let Some(v) = cache.max_as_facets.get(&cr_s) {
                    println!("navigation mode : {}", self);
                    println!(
                        "filtered        : {:?}/{:?}",
                        v.len(),
                        current_facets.len() * 2
                    );
                    println!("elapsed         : cached result\n");

                    v.to_vec()
                } else {
                    drop(cache);

                    let mut data = vec![];

                    let pbs = ProgressStyle::default_bar()
                        .template("solving [{elapsed_precise}] {bar:30} {msg}")
                        .progress_chars("##-");
                    let pb = ProgressBar::new((current_facets.len() / 2) as u64);
                    pb.set_style(pbs);
                    pb.enable_steady_tick(10);

                    current_facets.iter().for_each(|f| {
                        let repr = f.repr();
                        let neg_repr = format!("~{}", repr);

                        let (w0, w1) = eval_weight(&Weight::Absolute, navigator, &repr);
                        pb.inc(1);

                        data.push((repr, w0));
                        data.push((neg_repr, w1.expect("unknown error.")));
                    });
                    pb.finish_using_style();

                    let max = data.iter().map(|(_, w)| w).max().expect("unknown error.");

                    let fs = data
                        .iter()
                        .cloned()
                        .filter(|(_, w)| w == max)
                        .map(|(f_s, _)| f_s)
                        .collect::<Vec<String>>();

                    let mut cache = CACHE.lock().expect("cache lock is poisoned.");
                    assert!(cache.max_as_facets.put(cr_s, fs.clone()).is_none());

                    println!("navigation mode : {}", self);
                    println!(
                        "filtered        : {:?}/{:?}",
                        fs.len(),
                        current_facets.len() * 2
                    );
                    println!("elapsed         : {:?}\n", pb.elapsed());

                    fs
                }
            }
            Self::Explore(Weight::Absolute) => {
                let cr_s = navigator.route.iter().cloned().collect::<String>();

                if let Some(v) = cache.min_as_facets.get(&cr_s) {
                    println!("navigation mode : {}", self);
                    println!(
                        "filtered        : {:?}/{:?}",
                        v.len(),
                        current_facets.len() * 2
                    );
                    println!("elapsed         : cached result\n");

                    v.to_vec()
                } else {
                    drop(cache);

                    let mut data = vec![];

                    let pbs = ProgressStyle::default_bar()
                        .template("solving [{elapsed_precise}] {bar:30} {msg}")
                        .progress_chars("##-");
                    let pb = ProgressBar::new((current_facets.len() / 2) as u64);
                    pb.set_style(pbs);
                    pb.enable_steady_tick(10);

                    current_facets.iter().for_each(|f| {
                        let repr = f.repr();
                        let neg_repr = format!("~{}", repr);

                        let (w0, w1) = eval_weight(&Weight::Absolute, navigator, &repr);
                        pb.inc(1);

                        data.push((repr, w0));
                        data.push((neg_repr, w1.expect("unknown error.")));
                    });
                    pb.finish_using_style();

                    let min = data.iter().map(|(_, w)| w).min().expect("unknown error.");

                    let fs = data
                        .iter()
                        .cloned()
                        .filter(|(_, w)| w == min)
                        .map(|(f_s, _)| f_s)
                        .collect::<Vec<String>>();

                    let mut cache = CACHE.lock().expect("cache lock is poisoned.");
                    assert!(cache.min_as_facets.put(cr_s, fs.clone()).is_none());

                    println!("navigation mode : {}", self);
                    println!(
                        "filtered        : {:?}/{:?}",
                        fs.len(),
                        current_facets.len() * 2
                    );
                    println!("elapsed         : {:?}\n", pb.elapsed());

                    fs
                }
            }
            Self::GoalOriented(_) => {
                println!("\nnavigation mode : {}", self);
                println!(
                    "filtered        : {:?}/{:?}",
                    current_facets.len() * 2,
                    current_facets.len() * 2
                );
                println!("elapsed         : cached result\n");

                // NOTE: avoid .map
                println!("{}", navigator.current_facets);

                vec![]
            }
            Self::Io(_) => panic!(),
        }
    }
}

#[derive(Debug)]
pub(crate) enum EnumMode {
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
    pub(crate) control: Arc<Control>,
    pub(crate) literals: Literals,
    pub(crate) n: usize,
    pub current_facets: Facets,
    pub(crate) initial_facets: Facets,
    pub(crate) active_facets: Vec<Literal>,
    pub(crate) route: Route,
    pace: f32,
}
impl Navigator {
    pub fn new(source: impl Into<String>, n: usize) -> Result<Self> {
        let mut ctl = Control::new(vec!["0".to_owned()])?;

        let pbs = ProgressStyle::default_bar()
            .template("startup [{elapsed_precise}] {msg}")
            .progress_chars("#-");
        let pb = ProgressBar::new(30);
        pb.set_style(pbs);
        pb.enable_steady_tick(10);

        let logic_program = source.into();
        pb.set_message("| process: 1/6 adding program");
        ctl.add("base", &[], &logic_program)?;
        pb.set_message("| process: 2/6 grounding");
        ctl.ground(&[Part::new("base", &[])?])?;
        pb.inc(1);

        let n_cpus = num_cpus::get().to_string();
        ctl.configuration_mut() // activates parallel competition based search
            .map(|c| {
                c.root()
                    .and_then(|rk| c.map_at(rk, "solve.parallel_mode"))
                    .and_then(|sk| c.value_set(sk, &n_cpus))
            })??;
        let mut literals: Literals = HashMap::new();

        pb.set_message("| process: 3/6 collecting literals");
        for atom in ctl.symbolic_atoms()?.iter()? {
            literals.insert(atom.symbol()?, atom.literal()?);
        }
        pb.inc(2);

        pb.set_message("| process: 4/6 SAT check");
        let mut solve_handle = ctl.solve(SolveMode::YIELD, &[])?;
        let sat = solve_handle.get()? == SolveResult::SATISFIABLE;
        solve_handle.close()?;
        pb.inc(3);

        let initial_facets = match sat {
            true => {
                pb.set_message("| process: 5/6 computing brave consequences");
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
                pb.inc(4);

                pb.set_message("| process: 6/6 computing cautious consequences");
                pb.inc(4);
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

                let fs = match cc.is_empty() {
                    true => Facets(bc),
                    _ => Facets(bc.difference(&cc)),
                };

                let mut cache = CACHE.lock().expect("cache lock is poisoned.");
                cache.inclusive_facets.put(vec![], fs.clone());
                drop(cache);

                fs
            }
            _ => panic!("program is unsatisfiable."),
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

    #[cfg(not(tarpaulin_include))]
    pub(crate) fn assume(&mut self, assumptions: &[Literal]) {
        Arc::get_mut(&mut self.control)
            .expect("control error.")
            .backend()
            .and_then(|mut b| b.assume(assumptions))
            .expect("backend assumption failed.")
    }

    #[cfg(not(tarpaulin_include))]
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

    pub fn satisfiable(&mut self, assumptions: &[Literal]) -> bool {
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

    pub fn find_one(&mut self, assumptions: &[Literal]) -> Option<Vec<Symbol>> {
        let ctl = Arc::get_mut(&mut self.control).expect("control error.");

        let mut solve_handle = ctl
            .solve(SolveMode::YIELD, assumptions)
            .expect("getting solve handle failed.");

        let ret = match solve_handle.model() {
            Ok(Some(model)) => {
                            Some(model
                                .symbols(ShowType::SHOWN)
                                .expect("getting Symbols failed."))}
            _ => None
        };
        solve_handle.close().expect("closing solve handle failed.");
        ret

    }

    #[cfg(not(tarpaulin_include))]
    pub(crate) fn consequences(
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

    pub(crate) fn literal(&self, str: impl AsRef<str> + Debug) -> Result<Literal> {
        let s = str.as_ref();
        let negative_prefixes = &['~']; //

        match negative_prefixes.iter().any(|p| s.starts_with(*p)) {
            true => match Atom(&s[1..]).parse(negative_prefixes) {
                Some(s) => self.literals.get(&s).map(|l| l.negate()).ok_or_else(|| {
                    NavigatorError::InvalidInput(format!("unknown literal: {:?}", str))
                }),
                _ => {
                    println!("\n[INFO] cannot parse input");
                    Err(NavigatorError::InvalidInput(
                        "parsing literal failed.".to_owned(),
                    ))
                }
            },
            _ => match Atom(s).parse(negative_prefixes) {
                Some(s) => self.literals.get(&s).cloned().ok_or_else(|| {
                    NavigatorError::InvalidInput(format!("unknown literal: {:?}", str))
                }),
                _ => {
                    println!("\n[INFO] cannot parse input");
                    Err(NavigatorError::InvalidInput(
                        "parsing literal failed.".to_owned(),
                    ))
                }
            },
        }
    }

    pub fn inclusive_facets(&mut self, assumptions: &[Literal]) -> Facets {
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
                let facets = self.inclusive_facets(&route); // avoid that by using Lemma: does solve handle find second solution?
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

    #[cfg(not(tarpaulin_include))]
    pub(crate) fn update(&mut self, mode: &Mode) {
        match mode {
            Mode::GoalOriented(Weight::FacetCounting)
            | Mode::StrictlyGoalOriented(Weight::FacetCounting)
            | Mode::Explore(Weight::FacetCounting) => {
                let initial_count = (self.initial_facets.len() * 2) as f32;

                let assumptions = self.active_facets.clone();

                let mut cache = CACHE.lock().expect("cache lock is poisoned.");
                let lits = assumptions
                    .iter()
                    .map(|l| l.get_integer())
                    .collect::<Vec<i32>>();
                let new_facets = cache
                    .inclusive_facets
                    .get(&lits)
                    .cloned()
                    .unwrap_or_else(|| {
                        let fs = self.inclusive_facets(&assumptions);
                        cache.inclusive_facets.put(lits, fs.clone());
                        fs
                    });
                drop(cache);

                let new_count = (new_facets.len() * 2) as f32;

                self.current_facets = new_facets;
                self.pace = (initial_count - new_count) / initial_count;
            }
            _ => {
                let mut cache = CACHE.lock().expect("cache lock is poisoned.");

                let initial_count =
                    cache
                        .as_counts
                        .get(&"".to_owned())
                        .cloned()
                        .unwrap_or_else(|| {
                            let c = self.count(&[]);
                            cache.as_counts.put("".to_owned(), c);
                            c
                        }) as f32;
                let assumptions = self.active_facets.clone();
                let curr_route_str = self.route.iter().cloned().collect::<String>();
                let new_count = cache
                    .as_counts
                    .get(&curr_route_str)
                    .cloned()
                    .unwrap_or_else(|| {
                        let c = self.count(&assumptions);
                        cache.as_counts.put(curr_route_str, c);
                        c
                    }) as f32;

                let lits = assumptions
                    .iter()
                    .map(|l| l.get_integer())
                    .collect::<Vec<i32>>();
                let new_facets = cache
                    .inclusive_facets
                    .get(&lits)
                    .cloned()
                    .unwrap_or_else(|| {
                        let fs = self.inclusive_facets(&assumptions);
                        cache.inclusive_facets.put(lits, fs.clone());
                        fs
                    });
                drop(cache);

                self.current_facets = new_facets;
                self.pace = (initial_count - new_count) / initial_count;
            }
        }
    }

    #[cfg(not(tarpaulin_include))]
    pub fn navigate(&mut self) {
        self.assume(&self.active_facets.clone());

        let ctl = Arc::get_mut(&mut self.control).expect("control error.");

        let mut iter = ctl
            .all_models()
            .expect("solving failed.")
            .map(|model| model.symbols);

        println!();

        match iter.next() {
            Some(first_model) => {
                println!("Answer 1: ");
                for atom in first_model.clone() {
                    // quickfix
                    print!(
                        "{} ",
                        atom.to_string()
                            .expect("Symbol to String conversion failed.")
                    );
                }
                println!();

                for (i, model) in iter.enumerate() {
                    if model != first_model {
                        // quickfix
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
                }
                println!("SATISFIABLE\n");
            }
            _ => println!("UNSATISFIABLE\n"),
        }
    }

    #[cfg(not(tarpaulin_include))]
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

                let mut i = 1;

                match handle.get().expect("getting first solve result failed.")
                    != SolveResult::SATISFIABLE
                {
                    true => println!("UNSATISFIABLE\n"),
                    _ => {
                        let mut prev = vec![];
                        while let Some(model) = handle.model().expect("getting model failed.") {
                            let curr = model
                                .symbols(ShowType::SHOWN)
                                .expect("getting Symbols failed.");

                            match !prev.is_empty() && prev == curr.clone() {
                                true => handle.resume().expect("solve handle failed resuming."),
                                _ => {
                                    println!("Answer {:?}: ", i);
                                    for atom in curr.iter() {
                                        print!(
                                            "{} ",
                                            atom.to_string()
                                                .expect("Symbol to String conversion failed.")
                                        );
                                    }
                                    println!();

                                    prev = curr.clone();

                                    if i == n.unwrap_or(self.n) as u64 {
                                        println!("SATISFIABLE\n");

                                        handle.close().expect("closing solve handle failed.");

                                        return;
                                    }

                                    i += 1;

                                    handle.resume().expect("solve handle failed resuming.");
                                }
                            }
                        }

                        println!("SATISFIABLE\n");

                        handle.close().expect("closing solve handle failed.");
                    }
                }
            }
        }
    }

    #[cfg(not(tarpaulin_include))]
    pub fn parse_input_to_literals<'a, S>(
        &'a self,
        input: &'a [S],
    ) -> impl Iterator<Item = Literal> + 'a
    where
        S: AsRef<str> + Debug,
    {
        input
            .iter()
            .map(move |s| self.literal(s))
            .filter_map(Result::ok)
    }

    pub fn activate(&mut self, facets: &[String], mode: &Mode) {
        println!("\nsolving...");
        let start = Instant::now();

        for s in facets {
            let lit = self.literal(s);
            if lit.is_err() {
                println!("\n[ERROR] {:?}\n", lit);
                break;
            };

            self.route.activate(s);
            self.active_facets.push(unsafe { lit.unwrap_unchecked() });
        }

        self.update(mode);

        let elapsed = start.elapsed();

        println!("call    : --activate");
        println!("elapsed : {:?}\n", elapsed);
    }

    pub fn deactivate_any<S>(&mut self, facets: &[S], mode: &Mode)
    where
        S: Repr + Eq + Hash,
    {
        println!("\nsolving...");
        let start = Instant::now();

        facets.iter().unique().for_each(|f| {
            self.route.deactivate_any(f.repr()).iter().for_each(|pos| {
                self.active_facets.remove(*pos);
            });
        });

        self.update(mode);

        let elapsed = start.elapsed();

        println!("call    : --deactivate");
        println!("elapsed : {:?}\n", elapsed);
    }

    #[cfg(not(tarpaulin_include))]
    pub fn user_input(&self) -> String {
        let mut user_input = String::new();
        stdout()
            .flush()
            .and_then(|_| stdin().read_line(&mut user_input))
            .expect("IO operation failed.");

        user_input.trim().to_owned()
    }

    #[cfg(not(tarpaulin_include))]
    pub fn info(&mut self) {
        print!("{}", self.route);
        match self.satisfiable(&self.active_facets.clone()) {
            true => print!(" [ {:?}% ] ~> ", (self.pace * 100f32).round() as usize),
            _ => print!(" [ UNSAT ] ~> "),
        }
    }
}

pub fn first_solution_to_vec(source: impl Into<String>) -> Vec<String> {
    unsafe {
        let mut ctl = Control::new(vec!["0".to_owned()]).unwrap_unchecked();

        let logic_program = source.into();
        ctl.add("base", &[], &logic_program).unwrap_unchecked();
        ctl.ground(&[Part::new("base", &[]).unwrap_unchecked()])
            .unwrap_unchecked();

        let n_cpus = num_cpus::get().to_string();
        ctl.configuration_mut() // activates parallel competition based search
            .map(|c| {
                c.root()
                    .and_then(|rk| c.map_at(rk, "solve.parallel_mode"))
                    .and_then(|sk| c.value_set(sk, &n_cpus))
            })
            .unwrap_unchecked()
            .unwrap_unchecked();
        let mut handle = ctl.solve(SolveMode::YIELD, &[]).expect("solving failed.");

        let res = match handle.get().expect("getting first solve result failed.")
            != SolveResult::SATISFIABLE
        {
            true => vec![],
            _ => {
                if let Some(model) = handle.model().unwrap_unchecked() {
                    model
                        .symbols(ShowType::SHOWN)
                        .unwrap_unchecked()
                        .iter()
                        .map(|s| s.to_string().unwrap_unchecked())
                        .collect::<Vec<_>>()
                } else {
                    panic!()
                }
            }
        };

        handle.close().expect("closing solve handle failed.");

        res
    }
}

#[allow(unused)]
pub fn first_n_solutions_to_vec(source: impl Into<String>) -> Vec<Vec<String>> {
    unsafe {
        let mut ctl = Control::new(vec!["0".to_owned()]).unwrap_unchecked();

        let logic_program = source.into();
        ctl.add("base", &[], &logic_program).unwrap_unchecked();
        ctl.ground(&[Part::new("base", &[]).unwrap_unchecked()])
            .unwrap_unchecked();

        let n_cpus = num_cpus::get().to_string();
        ctl.configuration_mut() // activates parallel competition based search
            .map(|c| {
                c.root()
                    .and_then(|rk| c.map_at(rk, "solve.parallel_mode"))
                    .and_then(|sk| c.value_set(sk, &n_cpus))
            })
            .unwrap_unchecked()
            .unwrap_unchecked();
        let mut handle = ctl.solve(SolveMode::YIELD, &[]).expect("solving failed.");

        let res = match handle.get().expect("getting first solve result failed.")
            != SolveResult::SATISFIABLE
        {
            true => vec![],
            _ => {
                if let Some(model) = handle.model().unwrap_unchecked() {
                    model
                        .symbols(ShowType::SHOWN)
                        .unwrap_unchecked()
                        .iter()
                        .map(|s| s.to_string().unwrap_unchecked())
                        .collect::<Vec<_>>()
                } else {
                    panic!()
                }
            }
        };

        handle.close().expect("closing solve handle failed.");

        // res
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use rand::Rng;

    const PI_1: &str = "a;b. c;d :- b. e.";
    const QUEENS: &str = "
    #const n = 8.
    {q(I ,1..n)} == 1 :- I = 1..n.
    {q(1..n, J)} == 1 :- J = 1..n.
    :- {q(D-J, J)} >= 2, D = 2..2*n.
    :- {q(D+J, J)} >= 2, D = 1-n..n-1.
    ";
    const GRID: &str = "
    cell(1..U) :- U=9.
    obj(1..9).
    {set_obj_cell(X,C) : obj(X)} = 1 :- cell(C).
    :- set_obj_cell(X,C1), set_obj_cell(X,C2), C1!=C2.
    #show set_obj_cell/2.";
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

    #[should_panic(expected = "program is unsatisfiable.")]
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

    #[test]
    fn inclusive_facets() -> Result<()> {
        let nav = Navigator::new(GRID, 0)?;
        assert_eq!(nav.current_facets.len(), 81);

        let nav = Navigator::new(QUEENS, 0)?;
        assert_eq!(nav.current_facets.len(), 900);

        let mut nav = Navigator::new(PI_1, 0)?;
        assert_eq!(
            nav.current_facets
                .to_strings()
                .collect::<Vec<String>>()
                .to_hashset(),
            vec![
                "a".to_owned(),
                "b".to_owned(),
                "c".to_owned(),
                "d".to_owned()
            ]
            .to_hashset()
        );

        assert_eq!(
            nav.inclusive_facets(&[nav.literal("a")?, nav.literal("~a")?]),
            Facets(vec![])
        );
        assert_eq!(nav.inclusive_facets(&[nav.literal("a")?]), Facets(vec![]));
        assert_eq!(nav.inclusive_facets(&[nav.literal("~b")?]), Facets(vec![]));
        assert_eq!(nav.inclusive_facets(&[nav.literal("c")?]), Facets(vec![]));
        assert_eq!(nav.inclusive_facets(&[nav.literal("d")?]), Facets(vec![]));

        assert_eq!(
            nav.inclusive_facets(&[nav.literal("~a")?])
                .to_strings()
                .collect::<Vec<String>>()
                .to_hashset(),
            vec!["c".to_owned(), "d".to_owned()].to_hashset()
        );
        assert_eq!(
            nav.inclusive_facets(&[nav.literal("~c")?])
                .to_strings()
                .collect::<Vec<String>>()
                .to_hashset(),
            vec!["a".to_owned(), "b".to_owned(), "d".to_owned()].to_hashset()
        );
        assert_eq!(
            nav.inclusive_facets(&[nav.literal("~d")?])
                .to_strings()
                .collect::<Vec<String>>()
                .to_hashset(),
            vec!["a".to_owned(), "b".to_owned(), "c".to_owned()].to_hashset()
        );

        Ok(())
    }

    #[test]
    fn activate_deactivate() -> Result<()> {
        let mut rng = rand::thread_rng();

        let mut cache = CACHE.lock().expect("cache lock is poisoned.");
        cache.inclusive_facets.clear();
        drop(cache);

        let mut nav = Navigator::new(GRID, 0)?;
        let lits = nav.clone().literals;

        nav.activate(
            &["bla".to_owned()],
            &Mode::GoalOriented(Weight::FacetCounting),
        );
        nav.deactivate_any(
            &[Symbol::create_id("bla", true)?],
            &Mode::GoalOriented(Weight::FacetCounting),
        );
        assert_eq!(nav.active_facets, vec![]);

        nav.activate(
            &["set_obj_cell(1, 1)".to_owned()],
            &Mode::GoalOriented(Weight::FacetCounting),
        );
        assert_eq!(nav.active_facets, vec![]);

        let sym0 = lits
            .keys()
            .nth(rng.gen_range(0..3))
            .map(|s| s.repr())
            .ok_or(NavigatorError::None)?;
        nav.activate(&[sym0.clone()], &Mode::GoalOriented(Weight::FacetCounting));
        assert_eq!(
            nav.active_facets,
            vec![sym0.clone()]
                .iter()
                .flat_map(|s| nav.literal(s))
                .collect::<Vec<Literal>>()
        );
        assert_eq!(nav.route, Route(vec![sym0.clone()]));
        assert_eq!(nav.pace.round(), 0.21_f32.round());

        let sym1 = lits
            .keys()
            .nth(rng.gen_range(4..7))
            .map(|s| s.repr())
            .ok_or(NavigatorError::None)?;
        nav.activate(
            &[sym1.clone(), sym1.clone(), sym1.clone()],
            &Mode::GoalOriented(Weight::FacetCounting),
        );
        assert_eq!(
            nav.active_facets,
            vec![sym0.clone(), sym1.clone(), sym1.clone(), sym1.clone()]
                .iter()
                .flat_map(|s| nav.literal(s))
                .collect::<Vec<Literal>>()
        );
        nav.deactivate_any(
            &[Symbol::create_id(&sym1.clone(), true)?],
            &Mode::GoalOriented(Weight::FacetCounting),
        );
        assert_eq!(
            nav.active_facets,
            vec![sym0.clone()]
                .iter()
                .flat_map(|s| nav.literal(s))
                .collect::<Vec<Literal>>()
        );
        assert_eq!(nav.route, Route(vec![sym0.clone()]));
        assert_eq!(nav.pace.round(), 0.21_f32.round());

        let sym2 = lits
            .keys()
            .nth(rng.gen_range(0..7))
            .map(|s| s.repr())
            .map(|s| format!("~{}", s))
            .ok_or(NavigatorError::None)?;
        nav.activate(&[sym2.clone()], &Mode::GoalOriented(Weight::FacetCounting));
        assert_eq!(
            nav.active_facets,
            vec![sym0.clone(), sym2.clone()]
                .iter()
                .flat_map(|s| nav.literal(s))
                .collect::<Vec<Literal>>()
        );
        assert_eq!(nav.route, Route(vec![sym0.clone(), sym2.clone()]));
        let dsym2 = Symbol::create_id(&sym2.clone(), true)?;
        nav.deactivate_any(
            &[dsym2, dsym2, dsym2],
            &Mode::GoalOriented(Weight::FacetCounting),
        );
        assert_eq!(
            nav.active_facets,
            vec![sym0.clone()]
                .iter()
                .flat_map(|s| nav.literal(s))
                .collect::<Vec<Literal>>()
        );
        assert_eq!(nav.route, Route(vec![sym0.clone()]));
        nav.deactivate_any(
            &[Symbol::create_id(&sym0.clone(), true)?],
            &Mode::GoalOriented(Weight::FacetCounting),
        );
        assert_eq!(nav.active_facets, vec![]);
        assert_eq!(nav.route, Route(vec![]));
        assert_eq!(nav.pace.round(), 0.21_f32.round());

        let mut nav = Navigator::new(PI_1, 0)?;
        nav.activate(
            &[
                "a".to_owned(),
                "b".to_owned(),
                "c".to_owned(),
                "d".to_owned(),
                "e".to_owned(),
                "~a".to_owned(),
                "~b".to_owned(),
                "~c".to_owned(),
                "~d".to_owned(),
                "~e".to_owned(),
            ],
            &Mode::GoalOriented(Weight::FacetCounting),
        );
        assert_eq!(
            nav.active_facets
                .iter()
                .map(|l| l.get_integer())
                .collect::<Vec<i32>>()
                .to_hashset(),
            nav.literals
                .values()
                .cloned()
                .chain(nav.literals.values().map(|l| l.negate()))
                .map(|l| l.get_integer())
                .collect::<Vec<i32>>()
                .to_hashset()
        );
        assert_eq!(nav.pace.round(), 1.0_f32.round());
        nav.active_facets = vec![];
        nav.route = Route(vec![]);
        nav.update(&Mode::GoalOriented(Weight::FacetCounting));
        assert_eq!(nav.pace.round(), 0.0_f32.round());

        Ok(())
    }

    #[test]
    fn count() -> Result<()> {
        let mut nav = Navigator::new(PI_1, 0)?;

        assert_eq!(nav.count(&[]), 3);

        let msrs = [
            nav.count(&[nav.literal("a")?]),
            nav.count(&[nav.literal("~b")?]),
            nav.count(&[nav.literal("c")?]),
            nav.count(&[nav.literal("d")?]),
            nav.count(&[nav.literal("b")?, nav.literal("c")?]),
            nav.count(&[nav.literal("b")?, nav.literal("d")?]),
            nav.count(&[nav.literal("~c")?, nav.literal("~d")?]),
        ];
        assert!(msrs.iter().all(|c| *c == 1));

        let other = [
            nav.count(&[nav.literal("b")?]),
            nav.count(&[nav.literal("~a")?]),
            nav.count(&[nav.literal("~c")?]),
            nav.count(&[nav.literal("~d")?]),
        ];
        assert!(other.iter().all(|c| *c == 2));

        assert_eq!(nav.count(&[nav.literal("~e")?]), 0);
        assert_eq!(nav.count(&[nav.literal("e")?]), 3);

        Ok(())
    }

    #[test]
    fn current_route_is_maximal_safe() -> Result<()> {
        let mut nav = Navigator::new(PI_1, 0)?;

        assert!(nav.satisfiable(&[]));

        [
            ["a", "~b", "~c", "~d"],
            ["b", "~a", "c", "~d"],
            ["b", "~a", "~c", "d"],
        ]
        .iter()
        .for_each(|v| {
            nav.activate(
                &v.iter().map(|s| s.to_string()).collect::<Vec<String>>(),
                &Mode::GoalOriented(Weight::FacetCounting),
            );
            assert!(nav.current_route_is_maximal_safe());
            nav.route = Route(vec![]);
            nav.active_facets = vec![];
        });

        [["~c", "e"], ["~d", "e"], ["b", "~a"]]
            .iter()
            .for_each(|v| {
                nav.activate(
                    &v.iter().map(|s| s.to_string()).collect::<Vec<String>>(),
                    &Mode::GoalOriented(Weight::FacetCounting),
                );
                assert!(!nav.current_route_is_maximal_safe());
                nav.route = Route(vec![]);
                nav.active_facets = vec![];
            });

        Ok(())
    }

    #[test]
    fn satisfiable() -> Result<()> {
        let mut nav = Navigator::new(PI_1, 0)?;

        assert_eq!(nav.count(&[]), 3);

        let sat = [
            nav.satisfiable(&[nav.literal("a")?]),
            nav.satisfiable(&[nav.literal("~b")?]),
            nav.satisfiable(&[nav.literal("c")?]),
            nav.satisfiable(&[nav.literal("d")?]),
            nav.satisfiable(&[nav.literal("b")?, nav.literal("c")?]),
            nav.satisfiable(&[nav.literal("b")?, nav.literal("d")?]),
            nav.satisfiable(&[nav.literal("b")?]),
            nav.satisfiable(&[nav.literal("~a")?]),
            nav.satisfiable(&[nav.literal("~c")?]),
            nav.satisfiable(&[nav.literal("~d")?]),
        ];
        assert!(sat.iter().all(|b| *b));

        let unsat = [
            nav.satisfiable(&[nav.literal("a")?, nav.literal("~a")?]),
            nav.satisfiable(&[nav.literal("a")?, nav.literal("b")?]),
            nav.satisfiable(&[nav.literal("c")?, nav.literal("d")?]),
            nav.satisfiable(&[nav.literal("~e")?]),
        ];
        assert!(!unsat.iter().any(|b| *b));

        Ok(())
    }

    #[test]
    fn eval_weight_t() -> Result<()> {
        let mut nav = Navigator::new(GRID, 0)?;

        let ifs = nav
            .current_facets
            .clone()
            .iter()
            .map(|f| eval_weight(&Weight::FacetCounting, &mut nav, &f.repr()).0)
            .collect::<Vec<usize>>();
        assert_eq!(ifs.len(), nav.current_facets.len());
        assert!(ifs.iter().all(|w| *w == 34));

        let efs = nav
            .current_facets
            .clone()
            .iter()
            .map(|f| eval_weight(&Weight::FacetCounting, &mut nav, &format!("~{}", f.repr())).0)
            .collect::<Vec<usize>>();
        assert_eq!(efs.len(), nav.current_facets.len());
        assert!(efs.iter().all(|w| *w == 2));

        let mut nav = Navigator::new(PI_1, 0)?;

        assert_eq!(
            eval_weight(&Weight::FacetCounting, &mut nav, "a"),
            (nav.current_facets.len() * 2, None)
        );
        assert_eq!(
            eval_weight(&Weight::FacetCounting, &mut nav, "~b"),
            (nav.current_facets.len() * 2, None)
        );
        assert_eq!(
            eval_weight(&Weight::FacetCounting, &mut nav, "c"),
            (nav.current_facets.len() * 2, None)
        );
        assert_eq!(
            eval_weight(&Weight::FacetCounting, &mut nav, "d"),
            (nav.current_facets.len() * 2, None)
        );
        assert_eq!(
            eval_weight(&Weight::FacetCounting, &mut nav, "b"),
            (4, None)
        );
        assert_eq!(
            eval_weight(&Weight::FacetCounting, &mut nav, "~c"),
            (2, None)
        );
        assert_eq!(
            eval_weight(&Weight::FacetCounting, &mut nav, "~d"),
            (2, None)
        );

        assert_eq!(eval_weight(&Weight::Absolute, &mut nav, "a"), (2, Some(1)));
        assert_eq!(eval_weight(&Weight::Absolute, &mut nav, "~b"), (2, Some(1)));
        assert_eq!(eval_weight(&Weight::Absolute, &mut nav, "c"), (2, Some(1)));
        assert_eq!(eval_weight(&Weight::Absolute, &mut nav, "d"), (2, Some(1)));
        assert_eq!(eval_weight(&Weight::Absolute, &mut nav, "b"), (1, Some(2)));
        assert_eq!(eval_weight(&Weight::Absolute, &mut nav, "~c"), (1, Some(2)));
        assert_eq!(eval_weight(&Weight::Absolute, &mut nav, "~d"), (1, Some(2)));

        Ok(())
    }

    #[test]
    fn eval_zoom_t() -> Result<()> {
        let mut nav = Navigator::new(GRID, 0)?;

        let ifs = nav
            .current_facets
            .clone()
            .iter()
            .map(|f| eval_zoom(&Weight::FacetCounting, &mut nav, &f.repr()).0)
            .collect::<Vec<f32>>();
        assert_eq!(ifs.len(), nav.current_facets.len());
        assert!(ifs.iter().all(|z| *z as usize == 0.2987_f32 as usize));

        let efs = nav
            .current_facets
            .clone()
            .iter()
            .map(|f| eval_zoom(&Weight::FacetCounting, &mut nav, &format!("~{}", f.repr())).0)
            .collect::<Vec<f32>>();
        assert_eq!(efs.len(), nav.current_facets.len());
        assert!(efs.iter().all(|z| *z as usize == 0.012346_f32 as usize));

        let mut nav = Navigator::new(PI_1, 0)?;

        assert_eq!(
            eval_zoom(&Weight::FacetCounting, &mut nav, "a"),
            (1.0_f32, None)
        );
        assert_eq!(
            eval_zoom(&Weight::FacetCounting, &mut nav, "~b"),
            (1.0_f32, None)
        );
        assert_eq!(
            eval_zoom(&Weight::FacetCounting, &mut nav, "c"),
            (1.0_f32, None)
        );
        assert_eq!(
            eval_zoom(&Weight::FacetCounting, &mut nav, "d"),
            (1.0_f32, None)
        );
        assert_eq!(
            eval_zoom(&Weight::FacetCounting, &mut nav, "b"),
            (0.5_f32, None)
        );
        assert_eq!(
            eval_zoom(&Weight::FacetCounting, &mut nav, "~c"),
            (0.25_f32, None)
        );
        assert_eq!(
            eval_zoom(&Weight::FacetCounting, &mut nav, "~d"),
            (0.25_f32, None)
        );

        assert_eq!(
            eval_zoom(&Weight::Absolute, &mut nav, "a"),
            (2_f32 / 3_f32, Some(1_f32 / 3_f32))
        );
        assert_eq!(
            eval_zoom(&Weight::Absolute, &mut nav, "~b"),
            (2_f32 / 3_f32, Some(1_f32 / 3_f32))
        );
        assert_eq!(
            eval_zoom(&Weight::Absolute, &mut nav, "c"),
            (2_f32 / 3_f32, Some(1_f32 / 3_f32))
        );
        assert_eq!(
            eval_zoom(&Weight::Absolute, &mut nav, "d"),
            (2_f32 / 3_f32, Some(1_f32 / 3_f32))
        );
        assert_eq!(
            eval_zoom(&Weight::Absolute, &mut nav, "b"),
            (1_f32 / 3_f32, Some(2_f32 / 3_f32))
        );
        assert_eq!(
            eval_zoom(&Weight::Absolute, &mut nav, "~c"),
            (1_f32 / 3_f32, Some(2_f32 / 3_f32))
        );
        assert_eq!(
            eval_zoom(&Weight::Absolute, &mut nav, "~d"),
            (1_f32 / 3_f32, Some(2_f32 / 3_f32))
        );

        Ok(())
    }

    #[test]
    fn filter_t() -> Result<()> {
        let mut nav = Navigator::new(GRID, 0)?;
        let fs = nav.current_facets.clone().0;

        let filtered = filter(&Mode::GoalOriented(Weight::FacetCounting), &mut nav, &fs);
        assert_eq!(filtered.len(), 0);

        let filtered = filter(
            &Mode::StrictlyGoalOriented(Weight::FacetCounting),
            &mut nav,
            &fs,
        );
        assert_eq!(filtered.len(), nav.current_facets.len());
        assert_eq!(
            filtered.to_hashset(),
            nav.current_facets
                .to_strings()
                .collect::<Vec<String>>()
                .to_hashset()
        );
        let filtered = filter(&Mode::Explore(Weight::FacetCounting), &mut nav, &fs);
        assert_eq!(filtered.len(), nav.current_facets.len());
        assert_eq!(
            filtered.to_hashset(),
            nav.current_facets
                .to_strings()
                .map(|s| format!("~{}", s))
                .collect::<Vec<String>>()
                .to_hashset()
        );

        let mut cache = CACHE.lock().expect("cache lock is poisoned.");
        cache.max_fc_facets.clear();
        cache.min_fc_facets.clear();
        drop(cache);

        let mut nav0 = Navigator::new(PI_1, 0)?;
        let fs0 = nav0.current_facets.clone().0;

        let filtered0 = filter(&Mode::GoalOriented(Weight::FacetCounting), &mut nav0, &fs0);
        assert_eq!(filtered0.len(), 0);

        let filtered0 = filter(
            &Mode::StrictlyGoalOriented(Weight::FacetCounting),
            &mut nav0,
            &fs0,
        );
        assert_eq!(
            filtered0.to_hashset(),
            vec![
                "a".to_owned(),
                "~b".to_owned(),
                "c".to_owned(),
                "d".to_owned()
            ]
            .to_hashset()
        );
        let filtered = filter(&Mode::Explore(Weight::FacetCounting), &mut nav0, &fs0);
        assert_eq!(
            filtered.to_hashset(),
            vec!["~c".to_owned(), "~d".to_owned()].to_hashset()
        );

        let filtered = filter(&Mode::GoalOriented(Weight::Absolute), &mut nav0, &fs0);
        assert_eq!(filtered.len(), 0);

        let filtered = filter(
            &Mode::StrictlyGoalOriented(Weight::Absolute),
            &mut nav0,
            &fs0,
        );
        assert_eq!(
            filtered.to_hashset(),
            vec![
                "a".to_owned(),
                "~b".to_owned(),
                "c".to_owned(),
                "d".to_owned()
            ]
            .to_hashset()
        );

        let filtered = filter(&Mode::Explore(Weight::Absolute), &mut nav0, &fs0);
        assert_eq!(
            filtered.to_hashset(),
            vec![
                "b".to_owned(),
                "~c".to_owned(),
                "~d".to_owned(),
                "~a".to_owned()
            ]
            .to_hashset()
        );

        Ok(())
    }
}
