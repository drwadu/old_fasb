use crate::navigator::*;
use clingo::{SolveMode, Symbol};
use std::sync::Arc;

use crate::asnc::AsnC;
use crate::cache::CACHE;
use crate::translator::Atom;
use crate::utils::ToHashSet;
use itertools::partition;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

pub(crate) trait Diversity {
    fn k_greedy_search_show(&mut self, sample_size: Option<usize>);
    fn h0_perfect_sample_search_show(&mut self);
    fn naive_approach_representative_sample_show(&mut self);
    fn find_perfect_core(&mut self) -> Vec<HashSet<String>>;
    fn show_find_cores_encoding(&mut self);
    fn cores_in(&mut self);
}

impl Diversity for Navigator {
    fn k_greedy_search_show(&mut self, sample_size: Option<usize>) {
        let n = sample_size.unwrap_or(0);

        let mut cache = CACHE.lock().expect("cache lock is poisoned.");
        let mut seed = self.active_facets.clone();
        let seed_entry = seed.iter().map(|l| l.get_integer()).collect::<Vec<_>>();

        #[allow(non_snake_case)]
        let A = if let Some(cc) = cache.cautious_consequences.get(&seed_entry) {
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

        let ctl = Arc::get_mut(&mut self.control).expect("control error.");
        let mut solve_handle = unsafe { ctl.solve(SolveMode::YIELD, &seed).unwrap_unchecked() };
        let lits = self.literals.clone();
        let mut i = 1;

        #[cfg(feature = "with_stats")]
        let mut syms: Vec<clingo::Symbol> = vec![];

        match n == 0 {
            true => loop {
                unsafe { solve_handle.resume().unwrap_unchecked() };

                if let Ok(Some(model)) = solve_handle.model() {
                    if let Ok(atoms) = model.symbols(clingo::ShowType::SHOWN) {
                        //if let Ok(atoms) = model.symbols(clingo::ShowType::ALL) {
                        //println!("{:?}", model.symbols(clingo::ShowType::ALL));
                        if atoms.is_empty() {
                            break;
                        }

                        #[cfg(feature = "with_stats")]
                        syms.extend(atoms.iter().filter(|s| !A.contains(s)));

                        println!("Answer {:?}: ", i);
                        let atoms_strings = atoms
                            .iter()
                            .map(|atom| unsafe { atom.to_string().unwrap_unchecked() });
                        atoms_strings.clone().for_each(|atom| print!("{} ", atom));
                        seed.extend(
                            atoms
                                .iter()
                                .filter(|a| !A.contains(a))
                                .map(|atom| unsafe { atom.to_string().unwrap_unchecked() })
                                .flat_map(|atom| {
                                    lits.get(unsafe { &Atom(&atom).parse(&[]).unwrap_unchecked() })
                                        .cloned()
                                })
                                .map(|l| l.negate()),
                        );
                    }

                    unsafe {
                        solve_handle.close().unwrap_unchecked();
                        solve_handle = ctl.solve(SolveMode::YIELD, &seed).unwrap_unchecked();
                    }

                    i += 1;
                    println!();
                } else {
                    if i == 1 {
                        println!("UNSATISFIABLE");
                    }
                    break;
                }
            },
            _ => loop {
                unsafe { solve_handle.resume().unwrap_unchecked() };

                if let Ok(Some(model)) = solve_handle.model() {
                    if let Ok(atoms) = model.symbols(clingo::ShowType::SHOWN) {
                        if atoms.is_empty() {
                            break;
                        }
                        println!("Answer {:?}: ", i);
                        let atoms_strings = atoms
                            .iter()
                            .map(|atom| unsafe { atom.to_string().unwrap_unchecked() });
                        atoms_strings.clone().for_each(|atom| print!("{} ", atom));
                        seed.extend(
                            atoms
                                .iter()
                                .filter(|a| !A.contains(a))
                                .map(|atom| unsafe { atom.to_string().unwrap_unchecked() })
                                .flat_map(|atom| {
                                    lits.get(unsafe { &Atom(&atom).parse(&[]).unwrap_unchecked() })
                                        .cloned()
                                })
                                .map(|l| l.negate()),
                        );
                    }

                    unsafe {
                        solve_handle.close().unwrap_unchecked();
                        solve_handle = ctl.solve(SolveMode::YIELD, &seed).unwrap_unchecked();
                    }

                    i += 1;
                    println!();

                    if i == n + 1 {
                        break;
                    }
                } else {
                    if i == 1 {
                        println!("UNSATISFIABLE");
                    }
                    break;
                }
            },
        }

        unsafe { solve_handle.close().unwrap_unchecked() }

        #[cfg(feature = "with_stats")]
        {
            let syms_s = syms.to_hashset();
            let (n, m) = (syms.len(), syms_s.len()); // - i * A.len()
            match n == m {
                true => println!("completely diverse"),
                _ => println!("not completely diverse ({:?})", n - m),
            }
            let bcs_n = unsafe {
                self.consequences(EnumMode::Brave, &[])
                    .unwrap_unchecked()
                    .len()
            };
            match m == bcs_n {
                true => println!("completely diverse"),
                _ => println!("not representative ({:?})", m as f32 / bcs_n as f32),
            }
        }
    }
    fn find_perfect_core(&mut self) -> Vec<HashSet<String>> {
        let com = self.components();
        let cc = unsafe {
            self.consequences(crate::navigator::EnumMode::Cautious, &[])
                .unwrap_unchecked()
        };
        let sorted_com = com
            .0
            .iter()
            .filter(|(cover, (_, _))| *cover != &cc)
            .collect::<Vec<_>>();
        // println!("sorted_com: {:?}", sorted_com);

        let mut encoding = "".to_owned();
        let mut j = 0;
        sorted_com
            .iter()
            .filter(|(cover, (_, _))| cover.to_hashset() != cc.to_hashset())
            .enumerate()
            .for_each(|(i, (cover, (_, content)))| {
                cover.iter().filter(|a| !cc.contains(a)).for_each(|a| {
                    let s = unsafe { a.to_string().unwrap_unchecked() };
                    encoding = format!(
                        "{}\ncov({}) :- c({:?}).\n:- not cov({}).",
                        encoding, s, i, s,
                    )
                });
                content.iter().filter(|a| !cc.contains(a)).for_each(|a| {
                    encoding = format!(
                        "{}\ncon({},{:?}) :- c({:?}).",
                        encoding,
                        unsafe { a.to_string().unwrap_unchecked() },
                        i,
                        i
                    );
                });

                j += 1;
            });
        // choose at least 2 components
        encoding = format!("{}\n2 {{c(X) : X=0..{:?}}}.", encoding, j - 1);

        // contents of components may not overlap
        // project on components
        encoding = format!(
            "{}\n:- c(X),c(Y),X!=Y,con(I,X),con(I,Y).\n#show c/1.",
            encoding
        );
        // println!("{}", encoding);

        crate::navigator::first_solution_to_vec(encoding)
            .iter()
            .map(|s| s.replace("c(", "").replace(")", "").parse::<usize>().ok())
            .flatten()
            .map(|u| unsafe { sorted_com.get_unchecked(u) }.1 .0.clone())
            .collect::<Vec<_>>()
    }
    fn show_find_cores_encoding(&mut self) {
        let com = self.components();
        let cc = unsafe {
            self.consequences(crate::navigator::EnumMode::Cautious, &[])
                .unwrap_unchecked()
        };
        let sorted_com = com
            .0
            .iter()
            .filter(|(cover, (_, _))| *cover != &cc)
            .collect::<Vec<_>>();
        // println!("sorted_com: {:?}", sorted_com);

        let mut encoding = "".to_owned();
        let mut j = 0;
        sorted_com
            .iter()
            .filter(|(cover, (_, _))| cover.to_hashset() != cc.to_hashset())
            .enumerate()
            .for_each(|(i, (cover, (_, content)))| {
                cover.iter().filter(|a| !cc.contains(a)).for_each(|a| {
                    let s = unsafe { a.to_string().unwrap_unchecked() };
                    encoding = format!(
                        "{}\ncov({}) :- c({:?}).\n:- not cov({}).",
                        encoding, s, i, s,
                    )
                });
                content.iter().filter(|a| !cc.contains(a)).for_each(|a| {
                    encoding = format!(
                        "{}\ncon({},{:?}) :- c({:?}).",
                        encoding,
                        unsafe { a.to_string().unwrap_unchecked() },
                        i,
                        i
                    );
                });

                j += 1;
            });
        // choose at least 2 components
        encoding = format!("{}\n2 {{c(X) : X=0..{:?}}}.", encoding, j - 1);

        // project on components
        encoding = format!("{}\n#show c/1.", encoding);
        println!("{}", encoding);
    }
    fn cores_in(&mut self) {
        let com = self.components();
        let sorted_com = com.0.iter().collect::<Vec<_>>();
        let cc = unsafe {
            self.consequences(crate::navigator::EnumMode::Cautious, &[])
                .unwrap_unchecked()
        };

        let mut encoding = "".to_owned();
        let mut j = 0;

        let lits = self.literals.clone();
        for a in unsafe {
            self.consequences(crate::navigator::EnumMode::Brave, &[])
                .unwrap_unchecked()
        } {
            encoding = format!(
                "{}\n% {} {:?}",
                encoding,
                unsafe { a.to_string().unwrap_unchecked() },
                unsafe { lits.get(&a).unwrap_unchecked().get_integer() }
            );
        }
        sorted_com
            .iter()
            .filter(|(cover, (_, _))| cover.to_hashset() != cc.to_hashset())
            .enumerate()
            .for_each(|(i, (cover, (_, content)))| {
                cover.iter().filter(|a| !cc.contains(a)).for_each(|a| {
                    let s = unsafe { lits.get(a).unwrap_unchecked().get_integer() };
                    encoding = format!("{}\ncov({:?}) :- c({:?}).", encoding, s, i)
                });
                content.iter().filter(|a| !cc.contains(a)).for_each(|a| {
                    encoding = format!(
                        "{}\ncon({:?},{:?}) :- c({:?}).",
                        encoding,
                        unsafe { lits.get(a).unwrap_unchecked().get_integer() },
                        i,
                        i
                    );
                });

                j += 1;
            });
        encoding = format!(
            "{}\n2 {{c(X) : X=0..{:?}}}.\nb(X) :- con(X,_).\nt(X) :- cov(X).\n#show c/1.\n#show b/1.\n#show t/1.",
            encoding,
            j - 1
        );

        println!("{}", encoding);
    }
    fn naive_approach_representative_sample_show(&mut self) {
        let lits = self.literals.clone();

        //#[allow(non_snake_case)]
        //let A = unsafe {
        //    self.consequences(crate::navigator::EnumMode::Cautious, &[])
        //        .unwrap_unchecked()
        //};
        let mut to_observe = self.inclusive_facets(&[]).0.to_hashset();
        // let mut delta: Vec<clingo::Literal> = vec![];

        let ctl = Arc::get_mut(&mut self.control).expect("control error.");
        let mut i = 1;

        while !to_observe.is_empty() {
            let target = unsafe {
                to_observe
                    .iter()
                    .next()
                    .map(|s| s.to_string().ok())
                    .flatten()
                    .and_then(|s| crate::translator::Atom(&s).parse(&[]))
                    .and_then(|a| lits.get(&a))
                    .unwrap_unchecked()
            };
            //delta = delta
            //    .iter()
            //    .chain([*target].iter())
            //    .cloned()
            //    .collect::<Vec<_>>();
            //println!(
            //    "target={:?}\tdelta={:?}\tto_observe={:?}",
            //    target, delta, to_observe
            //);
            let mut solve_handle =
                //unsafe { ctl.solve(SolveMode::YIELD, &delta).unwrap_unchecked() };
                unsafe { ctl.solve(SolveMode::YIELD, &[*target]).unwrap_unchecked() };
            #[allow(clippy::needless_collect)]
            if let Ok(Some(model)) = solve_handle.model() {
                // SAT
                if let Ok(atoms) = model.symbols(clingo::ShowType::SHOWN) {
                    //let covered = atoms
                    //    .iter()
                    //    .filter(|a| to_observe.remove(a))
                    //    .map(|a| unsafe { lits.get(a).unwrap_unchecked() }.negate())
                    //    .collect::<Vec<_>>();

                    match atoms
                        .iter()
                        .map(|a| to_observe.remove(a))
                        .collect::<Vec<_>>()
                        .iter()
                        .any(|v| *v)
                    {
                        //match !covered.is_empty() {
                        true => {
                            // coverage increased
                            // reap
                            println!("Answer {:?}: ", i);
                            let atoms_strings = atoms.iter().map(|atom| {
                                atom.to_string().expect("atom to string conversion failed.")
                            });
                            atoms_strings.clone().for_each(|atom| print!("{} ", atom));

                            solve_handle.close().expect("closing solve handle failed.");

                            i += 1;
                            println!();

                            //if let Some(l) = delta.last_mut() {
                            //    *l = l.negate()
                            //}
                            //delta.extend(covered);
                        }
                        _ => continue, //
                    }
                }
            } else {
                // UNSAT
                if i == 1 {
                    println!("UNSATISFIABLE");
                }
                break;
            }
        }
    }

    fn h0_perfect_sample_search_show(&mut self) {
        //let a = restrict_to
        //    .iter()
        //    .map(|s| crate::translator::Atom(s).parse(&[]).unwrap())
        //    .collect::<HashSet<_>>();
        let (bcs, ccs) = unsafe {
            (
                self.consequences(EnumMode::Brave, &[]).unwrap_unchecked(),
                self.consequences(EnumMode::Cautious, &[])
                    .unwrap_unchecked(),
            )
        };

        let lits = self.literals.clone();

        let start_mwf = Instant::now();
        let fs = self.inclusive_facets(&[]).0;
        let mwf = {
            let mut max_w = bcs.len(); // |F+| <= |BC|

            let (mut ws, mut i) = (vec![], 0);
            let gfc = fs.len();
            for f in &fs {
                if Instant::now()
                    .checked_duration_since(start_mwf)
                    .map(|d| d.as_secs())
                    == Some(60)
                {
                    break;
                }
                let l = unsafe { lits.get(f).unwrap_unchecked() };
                let fc = self.inclusive_facets(&[*l]).len();
                if fc < max_w {
                    max_w = fc
                }
                ws.push((f, fc));
                i += 1;
            }

            match i == gfc {
                true => ws
                    .iter()
                    .filter(|(_, w)| *w == max_w)
                    .map(|(f, _)| **f)
                    .collect::<Vec<_>>(),
                _ => fs,
            }
        };

        let mut mwf_bcs = mwf
            .iter()
            .map(|f| {
                (f, unsafe {
                    self.consequences(EnumMode::Brave, &[*lits.get(f).unwrap_unchecked()])
                        .unwrap_unchecked()
                        .to_hashset()
                })
            })
            .collect::<Vec<_>>();
        let all_cover = mwf_bcs.iter().fold(HashSet::new(), |acc, (_, bcs)| {
            acc.union(bcs).cloned().collect()
        });
        let no_perfect_sample_exists = all_cover != bcs.to_hashset();
        if no_perfect_sample_exists {
            println!("no perfect sample exists");
            return;
        }

        #[allow(non_snake_case)]
        let mut F = all_cover
            .iter()
            .map(|f| {
                (f, unsafe {
                    self.consequences(EnumMode::Cautious, &[*lits.get(f).unwrap_unchecked()])
                        .unwrap_unchecked()
                })
            })
            .collect::<Vec<_>>();
        F.sort_unstable_by_key(|(_, ccs)| ccs.len());
        F.reverse();
        #[allow(non_snake_case)]
        let mut X = vec![];
        let ctl = Arc::get_mut(&mut self.control).expect("control error.");
        for (f, ccs) in F {
            let mut solve_handle = unsafe {
                ctl.solve(SolveMode::YIELD, &[*lits.get(f).unwrap_unchecked()])
                    .unwrap_unchecked()
            };
            println!("f={:?}", f);
            #[allow(non_snake_case)]
            let bc_X = X
                .iter()
                .fold(HashSet::new(), |acc, ats| acc.union(ats).cloned().collect());

            if ccs.iter().any(|a| bc_X.contains(a)) {
                X.clear();
                break;
            }

            while let Ok(Some(model)) = solve_handle.model() {
                println!("X = {:?}", X);
                if let Ok(atoms) = model.symbols(clingo::ShowType::SHOWN) {
                    match atoms.iter().any(|a| bc_X.contains(a)) {
                        true => continue, //
                        _ => {
                            X.push(atoms.to_hashset());
                            solve_handle.close().expect("closing solve handle failed.");
                            break;
                        }
                    }
                } else {
                    unimplemented!()
                }
            }
        }
        //        #[allow(clippy::needless_collect)]
        //        while let Ok(Some(model)) = solve_handle.model() {
        //            if let Ok(atoms) = model.symbols(clingo::ShowType::SHOWN) {
        //                let bc_X = X.iter().fold(HashSet::new(), |acc, ats| { acc.union(ats).cloned().collect() });
        //                match atoms
        //                    .iter()
        //                    .any(|a| bc_X.contains(&a))
        //                {
        //                    true => continue, //
        //                    _ => {
        //                        //println!("Answer {:?}: ", i);
        //                        //let atoms_strings = atoms.iter().map(|atom| {
        //                        //    atom.to_string().expect("atom to string conversion failed.")
        //                        //});
        //                        //atoms_strings.clone().for_each(|atom| print!("{} ", atom));
        //                        X.push(atoms.to_hashset());
        //                        solve_handle.close().expect("closing solve handle failed.");
        //                    }
        //                }
        //            }
        //        } else { unimplemented!!() }
        //    }
        //    }

        mwf_bcs.sort_unstable_by_key(|(_, bcs)| bcs.len());
        let iter = mwf_bcs.iter().rev().collect::<Vec<_>>();

        for i in mwf_bcs.len() - 1..1 {}

        println!("mwf does not cover bc: {:?}", no_perfect_sample_exists);
        println!("all_cover: {:?}", all_cover.len());
        println!("bcs: {:?}", bcs.len());
        println!("mwf: {:?}", mwf.len());
        // let mwf = fs.iter().filter(|(_, w)| w == &max_w);
        unimplemented!()
    }
}

//println!("{}", encoding);
//use std::fs::File;
//use std::io::prelude::*;
//let mut f = File::create("tc.lp").unwrap();
//f.write_all(encoding.as_bytes()).unwrap();
//unimplemented!()
//fn bob_search_show<'a>(
//    &mut self,
//    buckets: impl Iterator<Item = &'a usize>,
//    sample_size: Option<usize>,
//);
//fn bob_search_show<'a>(
//    &mut self,
//    mut buckets: impl Iterator<Item = &'a usize>,
//    sample_size: Option<usize>,
//) {
//    let n = sample_size.unwrap_or(0);

//    let mut cache = CACHE.lock().expect("cache lock is poisoned.");
//    let mut seed = self.active_facets.clone();
//    let seed_entry = seed.iter().map(|l| l.get_integer()).collect::<Vec<_>>();

//    #[allow(non_snake_case)]
//    let A = if let Some(cc) = cache.cautious_consequences.get(&seed_entry) {
//        cc.clone()
//    } else {
//        let cc = unsafe {
//            self.consequences(EnumMode::Cautious, &seed)
//                .unwrap_unchecked()
//        };

//        assert!(cache
//            .cautious_consequences
//            .put(seed_entry, cc.clone())
//            .is_none());

//        cc
//    };

//    let ctl = Arc::get_mut(&mut self.control).expect("control error.");
//    let mut solve_handle = unsafe { ctl.solve(SolveMode::YIELD, &seed).unwrap_unchecked() };
//    let lits = self.literals.clone();
//    let mut i = 1;
//    match n == 0 {
//        true => loop {
//            let reap_n = match buckets.next() {
//                Some(n_) => *n_,
//                _ => 1,
//            };
//            let mut j = 0;
//            let mut atoms_ = vec![];
//            let mut seen_shown = vec![];

//            while j < reap_n {
//                solve_handle.resume().unwrap();
//                if let Ok(Some(model)) = solve_handle.model() {
//                    if let Ok(atoms) = model.symbols(clingo::ShowType::SHOWN) {
//                        let projected_solution = atoms.to_hashset();
//                        if !seen_shown.contains(&projected_solution) {
//                            println!("Answer {:?}: ", i);
//                            let atoms_strings =
//                                atoms.iter().map(|atom| atom.to_string().unwrap());
//                            atoms_strings.clone().for_each(|atom| print!("{} ", atom));
//                            atoms_ = atoms.clone();
//                            seen_shown.push(projected_solution);
//                        } else {
//                            continue;
//                        }
//                    }

//                    i += 1;
//                    println!();
//                } else {
//                    if i == 1 {
//                        println!("UNSATISFIABLE");
//                    } else if j == 1 {
//                        println!("###");
//                    }
//                    return;
//                }

//                j += 1;
//            }
//            println!("###");

//            // atoms_
//            //     .iter()
//            //     .filter(|a| !cc.contains(a))
//            //     .map(|atom| atom.to_string().unwrap())
//            //     .for_each(|s| {
//            //         dbg!(s);
//            //     });
//            seed.extend(
//                atoms_
//                    .iter()
//                    .filter(|a| !cc.contains(a))
//                    .map(|atom| atom.to_string().unwrap())
//                    .flat_map(|atom| {
//                        self.lits
//                            .get(&translator::Atom(&atom).parse(&[]).unwrap())
//                            .cloned()
//                    })
//                    .map(|l| l.negate()),
//            );

//            solve_handle.close().unwrap();
//            solve_handle = self.ctl.solve(SolveMode::YIELD, &seed).unwrap();
//        },
//        _ => loop {
//            solve_handle.resume().unwrap();
//            if let Ok(Some(model)) = solve_handle.model() {
//                if let Ok(atoms) = model.symbols(clingo::ShowType::SHOWN) {
//                    println!("Answer {:?}: ", i);
//                    let atoms_strings = atoms.iter().map(|atom| atom.to_string().unwrap());
//                    atoms_strings.clone().for_each(|atom| print!("{} ", atom));
//                    seed.extend(
//                        atoms
//                            .iter()
//                            .filter(|a| !cc.contains(a))
//                            .map(|atom| atom.to_string().unwrap())
//                            .flat_map(|atom| {
//                                self.lits
//                                    .get(&translator::Atom(&atom).parse(&[]).unwrap())
//                                    .cloned()
//                            })
//                            .map(|l| l.negate()),
//                    );
//                }
//                solve_handle.close().unwrap();
//                solve_handle = self.ctl.solve(SolveMode::YIELD, &seed).unwrap();
//                i += 1;
//                println!();
//                if i == n + 1 {
//                    break;
//                }
//            } else {
//                if i == 1 {
//                    println!("UNSATISFIABLE");
//                }
//                break;
//            }
//        },
//    }

//    solve_handle.close().unwrap()
//}
