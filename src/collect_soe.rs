use crate::navigator::*;
use hashbrown::HashMap;
use std::collections::HashSet;
use std::sync::Arc;

use crate::cache::CACHE;
use crate::utils::ToHashSet;

type Element = clingo::Symbol;

pub enum Heuristic {
    Naive,
    SieveMin,
    SieveMax,
    NaiveSieve,
    DgreedySieve,
    DgreedySieveMax,
    DgreedySieveMaxPlus,
    DgreedySieveMaxAll,
    DgreedySieveMaxPlusAll,
}

pub(crate) trait Soe {
    fn collect_show(
        &mut self,
        lp: &str,
        target_atoms: &[Element],
        target_atoms_str: HashSet<String>,
    );
}

pub trait Collector {
    fn d_greedy(
        &mut self,
        ignored_atoms: &[Element],
        under: &[clingo::Literal],
        collection: &mut HashSet<Vec<Element>>,
        collection_size: &mut usize,
        observed: &mut HashSet<Element>,
    );
    fn d_greedy_show(
        &mut self,
        seed: Vec<clingo::Literal>,
        collection: &mut HashSet<Vec<Element>>,
        collection_size: &mut usize,
        missing: &mut HashSet<Element>,
        missing_str: &mut HashSet<String>,
        sizes: &mut Vec<usize>,
        lookup_table: &mut HashMap<Element, usize>,
    );

    fn s_greedy_show(&mut self, target_atoms: &[Element]);
    fn s_greedy_plus_show(&mut self, target_atoms: &[Element]);
    fn s_greedy(
        &mut self,
        ignored_atoms: &[Element],
        under: &[clingo::Literal],
        collection: &mut HashSet<Vec<Element>>,
        collection_size: &mut usize,
        lookup_table: &mut HashMap<Element, usize>,
    );
    fn template(&self) -> Vec<Element>;
    fn template_under(&mut self, under: &[clingo::Literal]) -> Vec<Element>;
}

impl Collector for Navigator {
    fn d_greedy(
        &mut self,
        ignored_atoms: &[Element],
        under: &[clingo::Literal],
        collection: &mut HashSet<Vec<clingo::Symbol>>,
        collection_size: &mut usize,
        observed: &mut HashSet<Element>,
    ) {
        let mut cache = CACHE.lock().expect("cache lock is poisoned.");
        let mut seed = self.active_facets.clone();
        seed.extend(under);
        let seed_entry = seed.iter().map(|l| l.get_integer()).collect::<Vec<_>>();
        let mut i = 0;

        let mut to_ignore = if let Some(cc) = cache.cautious_consequences.get(&seed_entry) {
            cc.clone()
        } else {
            let cc = unsafe {
                self.consequences(EnumMode::Cautious, &seed)
                    .unwrap_unchecked()
            };

            assert!(cache
                .cautious_consequences
                .put(seed_entry, cc.clone())
                .is_none());

            cc
        };
        to_ignore.extend(ignored_atoms);

        let ctl = Arc::get_mut(&mut self.control).expect("control error.");
        let mut solve_handle = unsafe {
            ctl.solve(clingo::SolveMode::YIELD, &seed)
                .unwrap_unchecked()
        };
        let lits = self.literals.clone(); // TODO: could be clone only once and given as argument?

        loop {
            unsafe { solve_handle.resume().unwrap_unchecked() };

            if let Ok(Some(model)) = solve_handle.model() {
                if let Ok(atoms) = model.symbols(clingo::ShowType::SHOWN) {
                    let non_ignored_atoms = atoms
                        .iter()
                        .filter(|a| !to_ignore.contains(a))
                        .map(|symbol| unsafe { lits.get(symbol).unwrap_unchecked() }.negate())
                        .collect::<Vec<_>>();

                    if atoms.is_empty() || (non_ignored_atoms.is_empty() && i > 0) {
                        println!("break");
                        break;
                    }

                    seed.extend(non_ignored_atoms);

                    if collection.insert(atoms.clone()) {
                        // TODO!
                        atoms
                            .iter()
                            .filter(|atom| !to_ignore.contains(atom))
                            .for_each(|atom| {
                                observed.insert(*atom);
                            });
                        *collection_size += 1;
                    }

                    i += 1;
                }
                unsafe {
                    solve_handle.close().unwrap_unchecked();
                    solve_handle = ctl
                        .solve(clingo::SolveMode::YIELD, &seed)
                        .unwrap_unchecked();
                }
            } else {
                break;
            }
        }

        unsafe { solve_handle.close().unwrap_unchecked() }
    }

    fn d_greedy_show(
        &mut self,
        mut delta: Vec<clingo::Literal>,
        collection: &mut HashSet<Vec<clingo::Symbol>>,
        collection_size: &mut usize,
        target_atoms: &mut HashSet<Element>,
        missing_str: &mut HashSet<String>,
        sizes: &mut Vec<usize>,
        lookup_table: &mut HashMap<Element, usize>,
    ) {
        let lits = self.literals.clone();
        //let mut to_ignore = unsafe { // T bar
        //    self.consequences(EnumMode::Cautious, &seed)
        //        .unwrap_unchecked()
        //};
        //to_ignore.extend(lits.keys().filter(|atom| !target_atoms.contains(atom)));
        let to_ignore = lits
            .keys()
            .filter(|atom| !target_atoms.contains(atom))
            .collect::<Vec<_>>();

        let ctl = Arc::get_mut(&mut self.control).expect("control error.");
        let mut solve_handle = unsafe {
            ctl.solve(clingo::SolveMode::YIELD, &delta)
                .unwrap_unchecked()
        };
        let mut i = 0;

        loop {
            unsafe { solve_handle.resume().unwrap_unchecked() };

            if let Ok(Some(model)) = solve_handle.model() {
                if let Ok(atoms) = model.symbols(clingo::ShowType::SHOWN) {
                    let delta_s = atoms
                        .iter()
                        .filter(|a| !to_ignore.contains(a))
                        //.filter(|a| target_atoms.contains(a))
                        .map(|symbol| unsafe { lits.get(symbol).unwrap_unchecked() }.negate())
                        .collect::<Vec<_>>();

                    //if atoms.is_empty() || (delta_s.is_empty() && i > 0) {
                    //    break;
                    //}

                    delta.extend(delta_s);

                    if collection.insert(atoms.clone()) {
                        // TODO!
                        atoms
                            .iter()
                            .filter(|atom| !to_ignore.contains(atom))
                            .for_each(|atom| {
                                target_atoms.insert(*atom);
                            });
                        *collection_size += 1;
                        i += 1;

                        let mut m = 0;
                        atoms.iter().for_each(|atom| {
                            if let Some(count) = lookup_table.get_mut(atom) {
                                *count += 1;
                            }
                            m += 1;
                            target_atoms.remove(atom);
                        });
                        println!("Answer {:?}: ", collection_size);
                        let atoms_strings = atoms.iter().map(|atom| {
                            atom.to_string().expect("atom to string conversion failed.")
                        });
                        atoms_strings.clone().for_each(|atom| {
                            missing_str.remove(&atom);
                            print!("{} ", atom)
                        });
                        println!();
                        sizes.push(m);
                    }
                }

                unsafe {
                    solve_handle.close().unwrap_unchecked();
                    solve_handle = ctl
                        .solve(clingo::SolveMode::YIELD, &delta)
                        .unwrap_unchecked();
                }
            } else {
                break;
            }
        }

        unsafe { solve_handle.close().unwrap_unchecked() }
    }

    fn s_greedy_show(&mut self, target_atoms: &[Element]) {
        let lits = self.literals.clone();

        //
        let mut n = 0;
        let mut freq_table: HashMap<clingo::Symbol, usize> = HashMap::new();
        target_atoms.iter().for_each(|atom| {
            n += 1;
            freq_table.insert(*atom, 0);
        });
        let mut chunks_table: HashMap<usize, HashSet<clingo::Symbol>> = HashMap::new();
        let mut population_size = 0;

        let ctl = Arc::get_mut(&mut self.control).expect("control error.");
        let mut i = 1;
        let mut to_observe = target_atoms.to_vec().to_hashset();
        let mut collection = vec![].to_hashset();
        let mut sizes = vec![];
        //

        while !to_observe.is_empty() {
            //
            println!(
                "### covered {:?}",
                freq_table.values().filter(|v| **v != 0).count() as f64 / n as f64
            );
            //

            let target = unsafe {
                // guess atom
                to_observe
                    .iter()
                    .next()
                    .and_then(|a| lits.get(&a))
                    .unwrap_unchecked()
            };

            let mut solve_handle = unsafe {
                ctl.solve(clingo::SolveMode::YIELD, &[*target])
                    .unwrap_unchecked()
            };

            // sat check
            if solve_handle
                .get()
                .map(|r| r == clingo::SolveResult::SATISFIABLE)
                .expect("unknwon error.")
                == false
            {
                //
                println!("is False");
                //
                break;
            }

            #[allow(clippy::needless_collect)]
            while let Ok(Some(model)) = solve_handle.model() {
                if let Ok(atoms) = model.symbols(clingo::ShowType::SHOWN) {
                    match atoms
                        .iter()
                        .map(|a| to_observe.remove(a))
                        .collect::<Vec<_>>()
                        .iter()
                        .any(|v| *v)
                    {
                        true => {
                            if collection.insert(atoms.clone()) {
                                //
                                let mut m = 0;
                                atoms.iter().for_each(|atom| {
                                    if let Some(count) = freq_table.get_mut(atom) {
                                        *count += 1;
                                    }
                                    m += 1;
                                });
                                //

                                println!("Answer {:?}: ", i);
                                let atoms_strings = atoms.iter().map(|atom| {
                                    atom.to_string().expect("atom to string conversion failed.")
                                });
                                atoms_strings.clone().for_each(|atom| print!("{} ", atom));
                                i += 1;
                                println!();

                                //
                                sizes.push(m);
                                //

                                break;
                            }
                        }
                        _ => {
                            solve_handle.resume().expect("closing solve handle failed.");
                            continue;
                        } // did not observe anything new
                    }
                }
            }

            solve_handle.close().expect("closing solve handle failed.");
        }

        //
        freq_table.iter().for_each(|(atom, freq)| {
            population_size += *freq;
            let freq_chunk = chunks_table
                .raw_entry_mut()
                .from_key(freq)
                .or_insert_with(|| (*freq, vec![*atom].to_hashset()));
            freq_chunk.1.insert(*atom);
        });
        let div = 2f64.powf(entropy(&freq_table, population_size as f64));
        let r = {
            let ts = n as f64;
            1f64 - (ts - div).abs() / ts
        };

        println!(
            "\nbins={:?}\nps={:?}\nm={:?}\n|A|={:?}\nr={:?}",
            chunks_table.len(),
            population_size,
            div,
            target_atoms.len(),
            r
        );
        print!("b ");
        for (bin_id, bin) in &chunks_table {
            let bl = bin.len();
            //(0..bl).for_each(|_| print!("#"));
            print!("{:?},{:?} ", bin_id, bl as f64 / n as f64);
        }
        println!("\n{:?}", sizes);
        println!();
        //
    }

    fn s_greedy_plus_show(&mut self, target_atoms: &[Element]) {
        let lits = self.literals.clone();

        //
        let mut n = 0;
        let mut freq_table: HashMap<clingo::Symbol, usize> = HashMap::new();
        target_atoms.iter().for_each(|atom| {
            n += 1;
            freq_table.insert(*atom, 0);
        });
        let mut chunks_table: HashMap<usize, HashSet<clingo::Symbol>> = HashMap::new();
        let mut population_size = 0;

        let ctl = Arc::get_mut(&mut self.control).expect("control error.");
        let mut i = 1;
        let mut to_observe = target_atoms.to_vec().to_hashset();
        let mut collection = vec![].to_hashset();
        let mut sizes = vec![];
        //

        while !to_observe.is_empty() {
            //
            println!(
                "### covered {:?}",
                freq_table.values().filter(|v| **v != 0).count() as f64 / n as f64
            );
            //

            let target = unsafe {
                // guess atom
                to_observe
                    .iter()
                    .next()
                    .and_then(|a| lits.get(&a))
                    .unwrap_unchecked()
            };

            let mut solve_handle = unsafe {
                ctl.solve(clingo::SolveMode::YIELD, &[*target])
                    .unwrap_unchecked()
            };

            // sat check
            if solve_handle
                .get()
                .map(|r| r == clingo::SolveResult::SATISFIABLE)
                .expect("unknwon error.")
                == false
            {
                break;
            }

            #[allow(clippy::needless_collect)]
            while let Ok(Some(model)) = solve_handle.model() {
                if let Ok(atoms) = model.symbols(clingo::ShowType::SHOWN) {
                    match atoms
                        .iter()
                        .map(|a| to_observe.remove(a))
                        .collect::<Vec<_>>()
                        .iter()
                        .any(|v| *v)
                    {
                        true => {
                            if collection.insert(atoms.clone()) {
                                //
                                let mut m = 0;
                                atoms.iter().for_each(|atom| {
                                    if let Some(count) = freq_table.get_mut(atom) {
                                        *count += 1;
                                    }
                                    m += 1;
                                });
                                //

                                println!("Answer {:?}: ", i);
                                let atoms_strings = atoms.iter().map(|atom| {
                                    atom.to_string().expect("atom to string conversion failed.")
                                });
                                atoms_strings.clone().for_each(|atom| print!("{} ", atom));
                                i += 1;
                                println!();

                                //
                                sizes.push(m);
                                //

                                break;
                            }
                        }
                        _ => {
                            solve_handle.resume().expect("closing solve handle failed.");
                            continue;
                        } // did not observe anything new
                    }
                }
            }

            solve_handle.close().expect("closing solve handle failed.");
        }

        freq_table.iter().for_each(|(atom, freq)| {
            population_size += *freq;
            let freq_chunk = chunks_table
                .raw_entry_mut()
                .from_key(freq)
                .or_insert_with(|| (*freq, vec![*atom].to_hashset()));
            freq_chunk.1.insert(*atom);
        });
        let div = 2f64.powf(entropy(&freq_table, population_size as f64));
        let r = {
            let ts = n as f64;
            1f64 - (ts - div).abs() / ts
        };

        println!(
            "\nbins={:?}\nps={:?}\nm={:?}\n|A|={:?}\nr={:?}",
            chunks_table.len(),
            population_size,
            div,
            target_atoms.len(),
            r
        );
        print!("b ");
        for (bin_id, bin) in &chunks_table {
            let bl = bin.len();
            //(0..bl).for_each(|_| print!("#"));
            print!("{:?},{:?} ", bin_id, bl as f64 / n as f64);
        }
        println!("\n{:?}", sizes);
        println!();
    }

    fn s_greedy(
        &mut self,
        ignored_atoms: &[Element],
        under: &[clingo::Literal],
        collection: &mut HashSet<Vec<clingo::Symbol>>,
        collection_size: &mut usize,
        lookup_table: &mut HashMap<clingo::Symbol, usize>,
    ) {
        let lits = self.literals.clone();

        let mut to_observe = self.inclusive_facets(under).0.to_hashset();
        ignored_atoms.iter().for_each(|s| {
            to_observe.remove(&s);
        });

        let ctl = Arc::get_mut(&mut self.control).expect("control error.");

        while !to_observe.is_empty() {
            let target = unsafe {
                to_observe
                    .iter()
                    .next()
                    .and_then(|a| lits.get(&a))
                    .unwrap_unchecked()
            };
            let mut solve_handle = unsafe {
                ctl.solve(clingo::SolveMode::YIELD, &[*target])
                    .unwrap_unchecked()
            };
            #[allow(clippy::needless_collect)]
            if let Ok(Some(model)) = solve_handle.model() {
                if let Ok(atoms) = model.symbols(clingo::ShowType::SHOWN) {
                    match atoms
                        .iter()
                        .map(|a| to_observe.remove(a))
                        .collect::<Vec<_>>()
                        .iter()
                        .any(|v| *v)
                    {
                        true => {
                            if collection.insert(atoms.clone()) {
                                // TODO!
                                atoms.iter().for_each(|atom| {
                                    if let Some(count) = lookup_table.get_mut(atom) {
                                        *count += 1;
                                    }
                                });
                                *collection_size += 1;
                            }
                            solve_handle.close().expect("closing solve handle failed.");
                        }
                        _ => continue,
                    }
                }
            } else {
                if collection.is_empty() {
                    return;
                }
                break;
            }
        }
    }

    fn template(&self) -> Vec<clingo::Symbol> {
        let facets = self.current_facets.clone();
        self.literals
            .clone()
            .into_keys()
            .filter(|a| facets.0.contains(a))
            .collect::<Vec<_>>()
    }

    fn template_under(&mut self, under: &[clingo::Literal]) -> Vec<clingo::Symbol> {
        self.inclusive_facets(under).0
    }
}

fn entropy(lookup_table: &HashMap<clingo::Symbol, usize>, sample_size: f64) -> f64 {
    -lookup_table
        .iter()
        .map(|(_, count)| *count as f64 / sample_size)
        .map(|probability| probability * probability.log2())
        .sum::<f64>()
}

impl Soe for Heuristic {
    fn collect_show(
        &mut self,
        lp: &str,
        target_atoms: &[Element],
        mut target_atoms_str: HashSet<String>,
    ) {
        let template_size = target_atoms.len();

        match self {
            Self::Naive => {
                let mut nav = unsafe { Navigator::new(lp, 0).unwrap_unchecked() };
                nav.s_greedy_show(target_atoms);
            }
            Self::NaiveSieve => {
                let mut or = ":-".to_owned();
                target_atoms_str.iter().for_each(|atom| {
                    or = format!("{} not {},", or, atom);
                });

                or = format!("{}.", &or[..or.len() - 1]);
                let mut nav =
                    unsafe { Navigator::new_lazy(format!("{}\n{}", lp, or), 0).unwrap_unchecked() };

                nav.s_greedy_show(target_atoms);
            }
            Self::SieveMin => {
                let mut sizes = vec![];
                let mut freq_table: HashMap<clingo::Symbol, usize> = HashMap::new();
                target_atoms.iter().for_each(|atom| {
                    freq_table.insert(*atom, 0);
                });

                let (mut n, mut i) = (0, 1);

                let mut or = ":-".to_owned();
                target_atoms_str.iter().for_each(|atom| {
                    or = format!("{} not {},", or, atom);
                });

                or = format!("{}.", &or[..or.len() - 1]);
                let mut nav =
                    unsafe { Navigator::new_lazy(format!("{}\n{}", lp, or), 0).unwrap_unchecked() };
                let mut lits = nav.literals.clone();

                let mut clause = target_atoms.iter().collect::<HashSet<_>>();
                let mut missing = clause.len();

                while missing != 0 {
                    let route = {
                        let (mut max, mut f) = (0, *unsafe {
                            clause
                                .iter()
                                .next()
                                .map(|atom| lits.get(atom))
                                .flatten()
                                .unwrap_unchecked()
                        });

                        if missing > 1 {
                            //for atom in clause.iter() {
                            for atom in nav.inclusive_facets(&[]).iter() {
                                let lit = unsafe { lits.get(atom).unwrap_unchecked() };
                                let count = nav.inclusive_facets_wrt(&[*lit], &clause).len();
                                if count >= max {
                                    f = *lit;
                                    max = count;
                                    if count == missing - 1 {
                                        break;
                                    }
                                }
                                let count =
                                    nav.inclusive_facets_wrt(&[lit.negate()], &clause).len();
                                if count >= max {
                                    f = lit.negate();
                                    max = count;
                                    if count == missing - 1 {
                                        break;
                                    }
                                }
                            }
                        }

                        &[f]
                    };

                    let s = nav.find_one(route);
                    if s.is_none() {
                        break;
                    }

                    let mut m = 0;
                    println!("Answer {:?}: ", i);
                    s.as_ref().map(|atoms| {
                        atoms.iter().for_each(|atom| {
                            if let Some(c) = freq_table.get_mut(atom) {
                                *c += 1;
                            }
                            let repr = unsafe { atom.to_string().unwrap_unchecked() };
                            if clause.remove(atom) {
                                n += 1;
                                target_atoms_str.remove(&repr);
                            }
                            m += 1;
                            print!("{} ", repr);
                        })
                    });
                    println!();
                    i += 1;
                    sizes.push(m);

                    if target_atoms_str.is_empty() {
                        break;
                    }

                    or = ":-".to_owned();
                    target_atoms_str.iter().for_each(|atom| {
                        or = format!("{} not {},", or, atom);
                    });
                    or = format!("{}.", &or[..or.len() - 1]);
                    nav = unsafe {
                        Navigator::new_lazy(format!("{}\n{}", lp, or), 0).unwrap_unchecked()
                    };
                    lits = nav.literals.clone();
                    missing = clause.len();

                    //
                    println!(
                        "### covered {:?}",
                        freq_table.values().filter(|v| **v != 0).count() as f64
                            / template_size as f64
                    );
                    //
                }

                let mut chunks_table: HashMap<usize, HashSet<clingo::Symbol>> = HashMap::new();
                let mut population_size = 0;
                freq_table.iter().for_each(|(atom, freq)| {
                    population_size += *freq;
                    let freq_chunk = chunks_table
                        .raw_entry_mut()
                        .from_key(freq)
                        .or_insert_with(|| (*freq, vec![*atom].to_hashset()));
                    freq_chunk.1.insert(*atom);
                });
                let div = 2f64.powf(entropy(&freq_table, population_size as f64));
                let r = {
                    let ts = template_size as f64;
                    1f64 - (ts - div).abs() / ts
                };

                println!(
                    "\nbins={:?}\nps={:?}\nm={:?}\n|A|={:?}\nr={:?}",
                    chunks_table.len(),
                    population_size,
                    div,
                    target_atoms.len(),
                    r,
                );
                print!("b ");
                for (bin_id, bin) in &chunks_table {
                    let bl = bin.len();
                    print!("{:?},{:?} ", bin_id, bl as f64 / n as f64);
                }
                println!("\n{:?}", sizes);
                println!();
            }
            Self::SieveMax => {
                let mut sizes = vec![];
                let mut freq_table: HashMap<clingo::Symbol, usize> = HashMap::new();
                target_atoms.iter().for_each(|atom| {
                    freq_table.insert(*atom, 0);
                });

                let (mut n, mut i) = (0, 1);

                let mut or = ":-".to_owned();
                target_atoms_str.iter().for_each(|atom| {
                    or = format!("{} not {},", or, atom);
                });
                or = format!("{}.", &or[..or.len() - 1]);
                let mut nav =
                    unsafe { Navigator::new_lazy(format!("{}\n{}", lp, or), 0).unwrap_unchecked() };
                let mut lits = nav.literals.clone();

                let mut clause = target_atoms.iter().collect::<HashSet<_>>();
                let mut missing = clause.len();

                while missing != 0 {
                    let route = {
                        let (mut min, mut f) = (missing, *unsafe {
                            clause
                                .iter()
                                .next()
                                .map(|atom| lits.get(atom))
                                .flatten()
                                .unwrap_unchecked()
                        });

                        if missing > 1 {
                            for atom in nav.inclusive_facets(&[]).iter() {
                                let lit = unsafe { lits.get(atom).unwrap_unchecked() };
                                let count = nav.inclusive_facets_wrt(&[*lit], &clause).len();
                                if count <= min {
                                    f = *lit;
                                    min = count;
                                    if count == 0 {
                                        break;
                                    }
                                }
                                let count =
                                    nav.inclusive_facets_wrt(&[lit.negate()], &clause).len();
                                if count <= min {
                                    f = lit.negate();
                                    min = count;
                                    if count == 0 {
                                        break;
                                    }
                                }
                            }
                        }

                        &[f]
                    };

                    let s = nav.find_one(route);
                    if s.is_none() {
                        break;
                    }

                    let mut m = 0;
                    println!("Answer {:?}: ", i);
                    s.as_ref().map(|atoms| {
                        atoms.iter().for_each(|atom| {
                            if let Some(c) = freq_table.get_mut(atom) {
                                *c += 1;
                            }
                            let repr = unsafe { atom.to_string().unwrap_unchecked() };
                            if clause.remove(atom) {
                                n += 1;
                                target_atoms_str.remove(&repr);
                            }
                            m += 1;
                            print!("{} ", repr);
                        })
                    });
                    println!();
                    i += 1;
                    sizes.push(m);

                    if target_atoms_str.is_empty() {
                        break;
                    }

                    or = ":-".to_owned();
                    target_atoms_str.iter().for_each(|atom| {
                        or = format!("{} not {},", or, atom);
                    });
                    or = format!("{}.", &or[..or.len() - 1]);
                    nav = unsafe {
                        Navigator::new_lazy(format!("{}\n{}", lp, or), 0).unwrap_unchecked()
                    };
                    lits = nav.literals.clone();
                    missing = clause.len();

                    //
                    println!(
                        "### covered {:?}",
                        freq_table.values().filter(|v| **v != 0).count() as f64
                            / template_size as f64
                    );
                    //
                }

                let mut chunks_table: HashMap<usize, HashSet<clingo::Symbol>> = HashMap::new();
                let mut population_size = 0;
                freq_table.iter().for_each(|(atom, freq)| {
                    population_size += *freq;
                    let freq_chunk = chunks_table
                        .raw_entry_mut()
                        .from_key(freq)
                        .or_insert_with(|| (*freq, vec![*atom].to_hashset()));
                    freq_chunk.1.insert(*atom);
                });
                let div = 2f64.powf(entropy(&freq_table, population_size as f64));
                let r = {
                    let ts = template_size as f64;
                    1f64 - (ts - div).abs() / ts
                };

                println!(
                    "\nbins={:?}\nps={:?}\nm={:?}\n|A|={:?}\nr={:?}",
                    chunks_table.len(),
                    population_size,
                    div,
                    target_atoms.len(),
                    r,
                );
                print!("b ");
                for (bin_id, bin) in &chunks_table {
                    let bl = bin.len();
                    print!("{:?},{:?} ", bin_id, bl as f64 / n as f64);
                }
                println!("\n{:?}", sizes);
                println!();
            }
            Self::DgreedySieve => {
                let (mut collection, mut collection_size, mut missing, mut sizes) = (
                    vec![].to_hashset(),
                    0,
                    target_atoms.to_vec().to_hashset(),
                    vec![],
                );
                let mut freq_table: HashMap<clingo::Symbol, usize> = HashMap::new();
                target_atoms.iter().for_each(|atom| {
                    freq_table.insert(*atom, 0);
                });

                let mut chunks_table: HashMap<usize, HashSet<clingo::Symbol>> = HashMap::new();
                let mut population_size = 0;

                let mut still_missing = missing.len();
                while still_missing > 0 {
                    let mut or = ":-".to_owned();

                    target_atoms_str.iter().for_each(|atom| {
                        or = format!("{} not {},", or, atom);
                    });

                    or = format!("{}.", &or[..or.len() - 1]);
                    let mut nav = unsafe {
                        Navigator::new_lazy(format!("{}\n{}", lp, or), 0).unwrap_unchecked()
                    };

                    nav.d_greedy_show(
                        vec![],
                        &mut collection,
                        &mut collection_size,
                        &mut missing,
                        &mut target_atoms_str,
                        &mut sizes,
                        &mut freq_table,
                    );

                    still_missing = missing.len();

                    //
                    freq_table.iter().for_each(|(atom, freq)| {
                        population_size += *freq;
                        let freq_chunk = chunks_table
                            .raw_entry_mut()
                            .from_key(freq)
                            .or_insert_with(|| (*freq, vec![*atom].to_hashset()));
                        freq_chunk.1.insert(*atom);
                    });
                    let div = 2f64.powf(entropy(&freq_table, population_size as f64));
                    let r = {
                        let ts = template_size as f64;
                        1f64 - (ts - div).abs() / ts
                    };

                    println!(
                        "\nbins={:?}\nps={:?}\nm={:?}\n|A|={:?}\nr={:?}",
                        chunks_table.len(),
                        population_size,
                        div,
                        target_atoms.len(),
                        r,
                    );
                    print!("b ");
                    for (bin_id, bin) in &chunks_table {
                        let bl = bin.len();
                        print!("{:?},{:?} ", bin_id, bl as f64 / target_atoms.len() as f64);
                    }
                    println!("\n{:?}", sizes);
                    println!();
                }
            }
            Self::DgreedySieveMax => {
                let (mut collection, mut collection_size, mut missing, mut sizes) = (
                    vec![].to_hashset(),
                    0,
                    target_atoms.to_vec().to_hashset(),
                    vec![],
                );

                //
                let mut freq_table: HashMap<clingo::Symbol, usize> = HashMap::new();
                target_atoms.iter().for_each(|atom| {
                    freq_table.insert(*atom, 0);
                });

                let mut chunks_table: HashMap<usize, HashSet<clingo::Symbol>> = HashMap::new();
                let mut population_size = 0;
                //

                let mut still_missing = missing.len();
                while still_missing > 0 {
                    let (mut nav, route) = match still_missing > 1 {
                        true => {
                            let mut or = ":-".to_owned();
                            target_atoms_str.iter().for_each(|atom| {
                                or = format!("{} not {},", or, atom);
                            });
                            or = format!("{}.", &or[..or.len() - 1]);
                            let mut nav = unsafe {
                                Navigator::new_lazy(format!("{}\n{}", lp, or), 0).unwrap_unchecked()
                            };

                            let lits = nav.literals.clone();
                            let route = {
                                let (mut min, mut f) = (still_missing, *unsafe {
                                    missing
                                        .iter()
                                        .next()
                                        .map(|atom| lits.get(atom))
                                        .flatten()
                                        .unwrap_unchecked()
                                });

                                for atom in missing.iter() {
                                    let lit = unsafe { lits.get(atom).unwrap_unchecked() };
                                    let missing_set =
                                        &missing.iter().map(|x| x).collect::<HashSet<_>>();

                                    let (con, facet_count, _) =
                                        nav.con_fs_cov(&[*lit], &missing_set);
                                    if con == 0 {
                                        //
                                        println!(
                                            "{} is false",
                                            atom.to_string()
                                                .expect("symbol to string conversion failed.")
                                        );
                                        //
                                        return;
                                    }
                                    if facet_count <= min {
                                        f = *lit;
                                        min = facet_count;
                                        if facet_count == 0 {
                                            break;
                                        }
                                    }
                                    let (_, facet_count, _) = nav.con_fs_cov(&[*lit], &missing_set);
                                    if facet_count <= min {
                                        f = lit.negate();
                                        min = facet_count;
                                        if facet_count == 0 {
                                            break;
                                        }
                                    }
                                }

                                vec![f]
                            };
                            (nav, route)
                        }
                        _ => {
                            let mut nav = unsafe {
                                Navigator::new_lazy(
                                    format!(
                                        "{}\n:- not {}.",
                                        lp,
                                        target_atoms_str.iter().next().expect("unknown error")
                                    ),
                                    1,
                                )
                                .unwrap_unchecked()
                            };

                            let mut m = 0;
                            println!("Answer {:?}: ", collection_size + 1);
                            nav.find_one(&[]).as_ref().map(|atoms| {
                                atoms.iter().for_each(|atom| {
                                    if let Some(c) = freq_table.get_mut(atom) {
                                        *c += 1;
                                    }
                                    let repr = unsafe { atom.to_string().unwrap_unchecked() };
                                    m += 1;
                                    print!("{} ", repr);
                                })
                            });
                            println!();
                            sizes.push(m);
                            break;
                        }
                    };

                    nav.d_greedy_show(
                        route,
                        &mut collection,
                        &mut collection_size,
                        &mut missing,
                        &mut target_atoms_str,
                        &mut sizes,
                        &mut freq_table,
                    );

                    still_missing = missing.len();

                    //
                    println!(
                        "### covered {:?}",
                        freq_table.values().filter(|v| **v != 0).count() as f64
                            / template_size as f64
                    );
                    //
                }
                //
                freq_table.iter().for_each(|(atom, freq)| {
                    population_size += *freq;
                    let freq_chunk = chunks_table
                        .raw_entry_mut()
                        .from_key(freq)
                        .or_insert_with(|| (*freq, vec![*atom].to_hashset()));
                    freq_chunk.1.insert(*atom);
                });
                let div = 2f64.powf(entropy(&freq_table, population_size as f64));
                let r = {
                    let ts = template_size as f64;
                    1f64 - (ts - div).abs() / ts
                };

                println!(
                    "\nbins={:?}\nps={:?}\nm={:?}\n|A|={:?}\nr={:?}",
                    chunks_table.len(),
                    population_size,
                    div,
                    target_atoms.len(),
                    r,
                );
                print!("b ");
                for (bin_id, bin) in &chunks_table {
                    let bl = bin.len();
                    print!("{:?},{:?} ", bin_id, bl as f64 / target_atoms.len() as f64);
                }
                println!("\n{:?}", sizes);
                println!();
                //
            }
            Self::DgreedySieveMaxAll => {
                let (mut collection, mut collection_size, mut missing, mut sizes) = (
                    vec![].to_hashset(),
                    0,
                    target_atoms.to_vec().to_hashset(),
                    vec![],
                );

                //
                let mut freq_table: HashMap<clingo::Symbol, usize> = HashMap::new();
                target_atoms.iter().for_each(|atom| {
                    freq_table.insert(*atom, 0);
                });

                let mut chunks_table: HashMap<usize, HashSet<clingo::Symbol>> = HashMap::new();
                let mut population_size = 0;
                //

                let mut still_missing = missing.len();
                while still_missing > 0 {
                    let (mut nav, route) = match still_missing > 1 {
                        true => {
                            let mut or = ":-".to_owned();
                            target_atoms_str.iter().for_each(|atom| {
                                or = format!("{} not {},", or, atom);
                            });
                            or = format!("{}.", &or[..or.len() - 1]);
                            let mut nav = unsafe {
                                Navigator::new_lazy(format!("{}\n{}", lp, or), 0).unwrap_unchecked()
                            };

                            let lits = nav.literals.clone();
                            let route = {
                                let (mut min, mut f) = (still_missing, *unsafe {
                                    missing
                                        .iter()
                                        .next()
                                        .map(|atom| lits.get(atom))
                                        .flatten()
                                        .unwrap_unchecked()
                                });

                                for atom in nav.inclusive_facets(&[]).iter() {
                                    let lit = unsafe { lits.get(atom).unwrap_unchecked() };
                                    let missing_set =
                                        &missing.iter().map(|x| x).collect::<HashSet<_>>();

                                    let (con, facet_count, _) =
                                        nav.con_fs_cov(&[*lit], &missing_set);
                                    if con == 0 {
                                        //
                                        println!(
                                            "{} is false",
                                            atom.to_string()
                                                .expect("symbol to string conversion failed.")
                                        );
                                        //
                                        return;
                                    }
                                    if facet_count <= min {
                                        f = *lit;
                                        min = facet_count;
                                        if facet_count == 0 {
                                            break;
                                        }
                                    }
                                    let (_, facet_count, _) = nav.con_fs_cov(&[*lit], &missing_set);
                                    if facet_count <= min {
                                        f = lit.negate();
                                        min = facet_count;
                                        if facet_count == 0 {
                                            break;
                                        }
                                    }
                                }

                                vec![f]
                            };
                            (nav, route)
                        }
                        _ => {
                            let mut nav = unsafe {
                                Navigator::new_lazy(
                                    format!(
                                        "{}\n:- not {}.",
                                        lp,
                                        target_atoms_str.iter().next().expect("unknown error")
                                    ),
                                    1,
                                )
                                .unwrap_unchecked()
                            };

                            let mut m = 0;
                            println!("Answer {:?}: ", collection_size + 1);
                            nav.find_one(&[]).as_ref().map(|atoms| {
                                atoms.iter().for_each(|atom| {
                                    if let Some(c) = freq_table.get_mut(atom) {
                                        *c += 1;
                                    }
                                    let repr = unsafe { atom.to_string().unwrap_unchecked() };
                                    m += 1;
                                    print!("{} ", repr);
                                })
                            });
                            println!();
                            sizes.push(m);
                            break;
                        }
                    };

                    nav.d_greedy_show(
                        route,
                        &mut collection,
                        &mut collection_size,
                        &mut missing,
                        &mut target_atoms_str,
                        &mut sizes,
                        &mut freq_table,
                    );

                    still_missing = missing.len();

                    //
                    println!(
                        "### covered {:?}",
                        freq_table.values().filter(|v| **v != 0).count() as f64
                            / template_size as f64
                    );
                    //
                }
                //
                freq_table.iter().for_each(|(atom, freq)| {
                    population_size += *freq;
                    let freq_chunk = chunks_table
                        .raw_entry_mut()
                        .from_key(freq)
                        .or_insert_with(|| (*freq, vec![*atom].to_hashset()));
                    freq_chunk.1.insert(*atom);
                });
                let div = 2f64.powf(entropy(&freq_table, population_size as f64));
                let r = {
                    let ts = template_size as f64;
                    1f64 - (ts - div).abs() / ts
                };

                println!(
                    "\nbins={:?}\nps={:?}\nm={:?}\n|A|={:?}\nr={:?}",
                    chunks_table.len(),
                    population_size,
                    div,
                    target_atoms.len(),
                    r,
                );
                print!("b ");
                for (bin_id, bin) in &chunks_table {
                    let bl = bin.len();
                    print!("{:?},{:?} ", bin_id, bl as f64 / target_atoms.len() as f64);
                }
                println!("\n{:?}", sizes);
                println!();
                //
            }
            Self::DgreedySieveMaxPlus => {
                let (mut collection, mut collection_size, mut missing, mut sizes) = (
                    vec![].to_hashset(),
                    0,
                    target_atoms.to_vec().to_hashset(),
                    vec![],
                );

                //
                let mut freq_table: HashMap<clingo::Symbol, usize> = HashMap::new();
                target_atoms.iter().for_each(|atom| {
                    freq_table.insert(*atom, 0);
                });

                let mut chunks_table: HashMap<usize, HashSet<clingo::Symbol>> = HashMap::new();
                let mut population_size = 0;
                //

                let mut still_missing = missing.len();
                while still_missing > 0 {
                    let (mut nav, route) = match still_missing > 1 {
                        true => {
                            let mut or = ":-".to_owned();
                            target_atoms_str.iter().for_each(|atom| {
                                or = format!("{} not {},", or, atom);
                            });
                            or = format!("{}.", &or[..or.len() - 1]);
                            let mut nav = unsafe {
                                Navigator::new_lazy(format!("{}\n{}", lp, or), 0).unwrap_unchecked()
                            };

                            let lits = nav.literals.clone();
                            let route = {
                                let (mut min, mut f) = (still_missing, *unsafe {
                                    missing
                                        .iter()
                                        .next()
                                        .map(|atom| lits.get(atom))
                                        .flatten()
                                        .unwrap_unchecked()
                                });

                                for atom in missing.iter() {
                                    let lit = unsafe { lits.get(atom).unwrap_unchecked() };
                                    let missing_set =
                                        &missing.iter().map(|x| x).collect::<HashSet<_>>();

                                    let (con, facet_count, trues) =
                                        nav.con_fs_cov(&[*lit], &missing_set);
                                    if con == 0 {
                                        //
                                        println!(
                                            "{} is false",
                                            atom.to_string()
                                                .expect("symbol to string conversion failed.")
                                        );
                                        //
                                        return;
                                    }
                                    if facet_count + (template_size - trues) <= min {
                                        f = *lit;
                                        min = facet_count;
                                        if facet_count == 0 {
                                            break;
                                        }
                                    }
                                    let (_, facet_count, trues) =
                                        nav.con_fs_cov(&[*lit], &missing_set);
                                    if facet_count + (template_size - trues) <= min {
                                        f = lit.negate();
                                        min = facet_count;
                                        if facet_count == 0 {
                                            break;
                                        }
                                    }
                                }

                                vec![f]
                            };
                            (nav, route)
                        }
                        _ => {
                            let mut nav = unsafe {
                                Navigator::new_lazy(
                                    format!(
                                        "{}\n:- not {}.",
                                        lp,
                                        target_atoms_str.iter().next().expect("unknown error")
                                    ),
                                    1,
                                )
                                .unwrap_unchecked()
                            };

                            let mut m = 0;
                            println!("Answer {:?}: ", collection_size + 1);
                            nav.find_one(&[]).as_ref().map(|atoms| {
                                atoms.iter().for_each(|atom| {
                                    if let Some(c) = freq_table.get_mut(atom) {
                                        *c += 1;
                                    }
                                    let repr = unsafe { atom.to_string().unwrap_unchecked() };
                                    m += 1;
                                    print!("{} ", repr);
                                })
                            });
                            println!();
                            sizes.push(m);
                            break;
                        }
                    };

                    nav.d_greedy_show(
                        route,
                        &mut collection,
                        &mut collection_size,
                        &mut missing,
                        &mut target_atoms_str,
                        &mut sizes,
                        &mut freq_table,
                    );

                    still_missing = missing.len();

                    //
                    println!(
                        "### covered {:?}",
                        freq_table.values().filter(|v| **v != 0).count() as f64
                            / template_size as f64
                    );
                    //
                }
                //
                freq_table.iter().for_each(|(atom, freq)| {
                    population_size += *freq;
                    let freq_chunk = chunks_table
                        .raw_entry_mut()
                        .from_key(freq)
                        .or_insert_with(|| (*freq, vec![*atom].to_hashset()));
                    freq_chunk.1.insert(*atom);
                });
                let div = 2f64.powf(entropy(&freq_table, population_size as f64));
                let r = {
                    let ts = template_size as f64;
                    1f64 - (ts - div).abs() / ts
                };

                println!(
                    "\nbins={:?}\nps={:?}\nm={:?}\n|A|={:?}\nr={:?}",
                    chunks_table.len(),
                    population_size,
                    div,
                    target_atoms.len(),
                    r,
                );
                print!("b ");
                for (bin_id, bin) in &chunks_table {
                    let bl = bin.len();
                    print!("{:?},{:?} ", bin_id, bl as f64 / target_atoms.len() as f64);
                }
                println!("\n{:?}", sizes);
                println!();
                //
            }
            Self::DgreedySieveMaxPlusAll => {
                let (mut collection, mut collection_size, mut missing, mut sizes) = (
                    vec![].to_hashset(),
                    0,
                    target_atoms.to_vec().to_hashset(),
                    vec![],
                );

                //
                let mut freq_table: HashMap<clingo::Symbol, usize> = HashMap::new();
                target_atoms.iter().for_each(|atom| {
                    freq_table.insert(*atom, 0);
                });

                let mut chunks_table: HashMap<usize, HashSet<clingo::Symbol>> = HashMap::new();
                let mut population_size = 0;
                //

                let mut still_missing = missing.len();
                while still_missing > 0 {
                    let (mut nav, route) = match still_missing > 1 {
                        true => {
                            let mut or = ":-".to_owned();
                            target_atoms_str.iter().for_each(|atom| {
                                or = format!("{} not {},", or, atom);
                            });
                            or = format!("{}.", &or[..or.len() - 1]);
                            let mut nav = unsafe {
                                Navigator::new_lazy(format!("{}\n{}", lp, or), 0).unwrap_unchecked()
                            };

                            let lits = nav.literals.clone();
                            let route = {
                                let (mut min, mut f) = (still_missing, *unsafe {
                                    missing
                                        .iter()
                                        .next()
                                        .map(|atom| lits.get(atom))
                                        .flatten()
                                        .unwrap_unchecked()
                                });

                                for atom in nav.inclusive_facets(&[]).iter() {
                                    let lit = unsafe { lits.get(atom).unwrap_unchecked() };
                                    let missing_set =
                                        &missing.iter().map(|x| x).collect::<HashSet<_>>();

                                    let (con, facet_count, trues) =
                                        nav.con_fs_cov(&[*lit], &missing_set);
                                    if con == 0 {
                                        //
                                        println!(
                                            "{} is false",
                                            atom.to_string()
                                                .expect("symbol to string conversion failed.")
                                        );
                                        //
                                        return;
                                    }
                                    if facet_count + (template_size - trues) <= min {
                                        f = *lit;
                                        min = facet_count;
                                        if facet_count == 0 {
                                            break;
                                        }
                                    }
                                    let (_, facet_count, trues) =
                                        nav.con_fs_cov(&[*lit], &missing_set);
                                    if facet_count + (template_size - trues) <= min {
                                        f = lit.negate();
                                        min = facet_count;
                                        if facet_count == 0 {
                                            break;
                                        }
                                    }
                                }

                                vec![f]
                            };
                            (nav, route)
                        }
                        _ => {
                            let mut nav = unsafe {
                                Navigator::new_lazy(
                                    format!(
                                        "{}\n:- not {}.",
                                        lp,
                                        target_atoms_str.iter().next().expect("unknown error")
                                    ),
                                    1,
                                )
                                .unwrap_unchecked()
                            };

                            let mut m = 0;
                            println!("Answer {:?}: ", collection_size + 1);
                            nav.find_one(&[]).as_ref().map(|atoms| {
                                atoms.iter().for_each(|atom| {
                                    if let Some(c) = freq_table.get_mut(atom) {
                                        *c += 1;
                                    }
                                    let repr = unsafe { atom.to_string().unwrap_unchecked() };
                                    m += 1;
                                    print!("{} ", repr);
                                })
                            });
                            println!();
                            sizes.push(m);
                            break;
                        }
                    };

                    nav.d_greedy_show(
                        route,
                        &mut collection,
                        &mut collection_size,
                        &mut missing,
                        &mut target_atoms_str,
                        &mut sizes,
                        &mut freq_table,
                    );

                    still_missing = missing.len();

                    //
                    println!(
                        "### covered {:?}",
                        freq_table.values().filter(|v| **v != 0).count() as f64
                            / template_size as f64
                    );
                    //
                }
                //
                freq_table.iter().for_each(|(atom, freq)| {
                    population_size += *freq;
                    let freq_chunk = chunks_table
                        .raw_entry_mut()
                        .from_key(freq)
                        .or_insert_with(|| (*freq, vec![*atom].to_hashset()));
                    freq_chunk.1.insert(*atom);
                });
                let div = 2f64.powf(entropy(&freq_table, population_size as f64));
                let r = {
                    let ts = template_size as f64;
                    1f64 - (ts - div).abs() / ts
                };

                println!(
                    "\nbins={:?}\nps={:?}\nm={:?}\n|A|={:?}\nr={:?}",
                    chunks_table.len(),
                    population_size,
                    div,
                    target_atoms.len(),
                    r,
                );
                print!("b ");
                for (bin_id, bin) in &chunks_table {
                    let bl = bin.len();
                    print!("{:?},{:?} ", bin_id, bl as f64 / target_atoms.len() as f64);
                }
                println!("\n{:?}", sizes);
                println!();
                //
            }
        }
    }
}
