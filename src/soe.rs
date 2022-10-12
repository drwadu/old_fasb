use crate::navigator::{EnumMode, Navigator};
use clingo::{SolveMode, Symbol};
use std::sync::Arc;

use crate::asnc::AsnC;
use crate::cache::CACHE;
use crate::translator::Atom;
use crate::utils::ToHashSet;

pub(crate) trait Diversity {
    fn k_greedy_search_show(&mut self, sample_size: Option<usize>);
    //fn bob_search_show<'a>(
    //    &mut self,
    //    buckets: impl Iterator<Item = &'a usize>,
    //    sample_size: Option<usize>,
    //);
    fn cores(&mut self) -> Vec<Vec<Symbol>>;
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

        match n == 0 {
            true => loop {
                unsafe { solve_handle.resume().unwrap_unchecked() };

                if let Ok(Some(model)) = solve_handle.model() {
                    if let Ok(atoms) = model.symbols(clingo::ShowType::SHOWN) {
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
    }
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

    fn cores(&mut self) -> Vec<Vec<Symbol>> {
        let com = self.components();
        let sorted_com = com.0.iter().collect::<Vec<_>>();
        let cc = self
            .consequences(crate::navigator::EnumMode::Cautious, &[])
            .unwrap();

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
                        //"{}\ncov({}) :- c({:?}).\n:- not cov({}).",
                        "{}\ncov({}) :- c({:?}).",
                        //encoding, s, i, s
                        encoding,
                        s,
                        i
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
        encoding = format!("{}\n2 {{c(X) : X=0..{:?}}}.", encoding, j - 1);
        encoding = format!(
            "{}\n:- c(X),c(Y),X!=Y,con(I,X),con(I,Y).\n#show c/1.",
            encoding
        );
        println!("{}", encoding);
        use std::fs::File;
        use std::io::prelude::*;
        let mut f = File::create("tc.lp").unwrap();
        f.write_all(encoding.as_bytes()).unwrap();
        unimplemented!()
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
}