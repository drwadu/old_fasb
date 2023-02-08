use crate::navigator::*;
use hashbrown::HashMap;
use std::collections::HashSet;
use std::str::MatchIndices;
use std::sync::Arc;

use crate::cache::CACHE;
use crate::dlx::Matrix;
use crate::translator::Atom;
use crate::utils::ToHashSet;

pub(crate) enum Heuristic {
    Unnamed,
}
pub(crate) trait Cover<S>
where
    S: Sampler,
{
    fn search_perfect_sample_show(
        &mut self,
        sampler: &mut S,
        ignored_atoms: impl Iterator<Item = clingo::Symbol>,
    );
}
impl<S> Cover<S> for Heuristic
where
    S: Sampler,
{
    fn search_perfect_sample_show(
        &mut self,
        sampler: &mut S,
        ignored_atoms: impl Iterator<Item = clingo::Symbol>,
    ) {
        let template = sampler.template();
        let init_sample = sampler.naive_approach_representative_search(ignored_atoms); // rows
        let template_size = template.len();

        let mut incidence_matrix = Matrix::new(template_size);
        let mut lookup_table: HashMap<clingo::Symbol, usize> = HashMap::new();
        template.iter().for_each(|atom| {
            lookup_table.insert(*atom, 0);
        });

        let mut sample_size = 0;
        init_sample.iter().for_each(|answer_set| {
            let row = template
                .iter()
                .map(|atom| {
                    let entry = answer_set.contains(atom);
                    if entry {
                        let count = unsafe { lookup_table.get_mut(atom).unwrap_unchecked() };
                        *count += 1;
                    }
                    entry
                })
                .collect::<Vec<_>>();
            sample_size += 1;

            incidence_matrix.add_row(&row);
        });
        eprintln!("c init sample size {:?}", sample_size);

        eprintln!("c exact cover check");
        // check
        let exact_covers = crate::dlx::solve(incidence_matrix); // TODO: impl first found
        if !exact_covers.is_empty() {
            // NOTE: consider dropping init_sample and reading output from columns_vec
            let models = exact_covers
                .iter()
                .next()
                .map(|ec| {
                    ec.iter()
                        .map(|idx| unsafe { init_sample.get_unchecked(*idx) })
                })
                .expect("unknown error.");
            for (i, model) in models.enumerate() {
                println!("Answer {:?}:", i + 1);
                model
                    .iter()
                    .for_each(|atom| print!("{} ", unsafe { atom.to_string().unwrap_unchecked() }));
                println!();
            }
            return;
        }

        eprintln!("c imperfect");

        lookup_table.retain(|_, count| *count > 0); // removing atoms that are projected away
                                                    //drop(init_sample);

        match self {
            Self::Unnamed => {
                eprintln!("c starting heuristic unnamed");
                //#[cfg(feature = "with_stats")]
                {
                    let mut observation_table: HashMap<usize, HashSet<clingo::Symbol>> =
                        HashMap::new();

                    let (mut uniques, mut value) = (0, 0);

                    lookup_table.iter().for_each(|(k, v)| {
                        if *v == 1 {
                            uniques += 1;
                        }
                        value += v;
                        let c = observation_table
                            .raw_entry_mut()
                            .from_key(v)
                            .or_insert_with(|| (*v, HashSet::new()));
                        c.1.insert(*k);
                    });

                    let uniques_chunk = unsafe { observation_table.get(&1).unwrap_unchecked() };

                    //dbg!(observation_table
                    //    .iter()
                    //    .map(|(a, b)| (
                    //        a,
                    //        b.iter().map(|s| s.to_string().unwrap()).collect::<Vec<_>>(),
                    //        b.len(),
                    //    ))
                    //    .collect::<Vec<_>>());

                    eprintln!(
                        "c uni: {:.2}\terr: {:.2}",
                        uniques as f32 / template_size as f32,
                        1f32 - (template_size as f32 / value as f32), // NOTE: s >= n_atoms
                    );

                    //println!("uniques chunk size: {:?}", uniques_chunk.len());
                    let covered_under = uniques_chunk
                        .iter()
                        .map(|s| sampler.covered(&[sampler.ext(s)]))
                        .collect::<Vec<_>>();
                    let mut iter = covered_under.iter().cloned();
                    let common = unsafe {
                        iter.next()
                            .map(|a| {
                                iter.fold(a, |b, c| {
                                    b.intersection(&c).cloned().collect::<HashSet<_>>()
                                })
                            })
                            .unwrap_unchecked()
                    };

                    // check whether perfect sample can be excluded already
                    let mut perfect_sample_possible = sampler.admits_perfect_sample(&common);
                    match perfect_sample_possible {
                        true => {
                            eprintln!("c generating unique chunk template");
                            let left = uniques_chunk.iter().collect::<Vec<_>>();
                            println!("{:?}",left
                                .iter()
                                .map(|s| s.to_string().unwrap())
                                .collect::<Vec<_>>());
                            let uniques = uniques_chunk
                                .iter()
                                .map(|atom| sampler.ext(atom))
                                .collect::<Vec<_>>();
                            //let apply_bc_left = uniques_chunk
                            //    .iter()
                            //    .map(|atom| sampler.within(&[sampler.ext(atom)])) // TODO: caching
                            //    .collect::<Vec<_>>();

                            //let local_proper_chunks = observation_table
                            //    .iter()
                            //    .map(|(occurences_number, chunk)| {
                            //        (occurences_number, chunk, chunk.len())
                            //    })
                            //    .filter(|(occurences_number, _, _)| **occurences_number > 1)
                            //    .collect::<Vec<_>>();

                            // for all or next smallest?
                            // for all
                            let fold_proper_chunks =
                                lookup_table.keys().filter(|k| !uniques_chunk.contains(k)).collect::<Vec<_>>();
                            let right = lookup_table
                                .keys()
                                .filter(|k| !uniques_chunk.contains(k))
                                .collect::<Vec<_>>();

                            // next smallest
                            //let next_to_unique = unsafe {observation_table.keys().filter(|k| **k > 1).min().unwrap_unchecked()};
                            //let fold_proper_chunks = unsafe  { observation_table.get(next_to_unique).unwrap_unchecked() };
                            let right_lits = fold_proper_chunks.iter()
                                .map(|atom| sampler.ext(atom))
                                .collect::<Vec<_>>();

                            eprintln!("{:?}", fold_proper_chunks.iter().map(|s| s.to_string().unwrap()).collect::<Vec<_>>());
                            let mut g = Matrix::new(fold_proper_chunks.len());
                            let mut i = 0;
                            covered_under.iter().for_each(
                                |cautious_consequences| {
                                    println!("{:?}", i);
                                    let row = fold_proper_chunks.iter()
                                        .map(|atom| cautious_consequences.contains(atom))
                                        .collect::<Vec<_>>();
                                    println!("{:?}", row);
                                    g.add_row(&row);
                                    perfect_sample_possible = !row.iter().any(|v| *v);
                                    i += 1;
                                    //g.add_row(&row);
                                },
                            );

                            if !perfect_sample_possible {
                                return eprintln!("c there is no perfect sample")
                            }
                            //right_lits.iter().enumerate().for_each(|(i, lit_r)| {
                            //    let row = uniques
                            //        .iter()
                            //        .map(|lit_l| {
                            //            let mut under = vec![*lit_r, lit_l.negate()];
                            //            under.extend(right_lits.iter().filter(|l| *l != lit_r));
                            //            sampler.sat(&under)
                            //        })
                            //        .collect::<Vec<_>>();
                            //    let rrow = row
                            //        .iter()
                            //        .map(|v| match v {
                            //            true => 1,
                            //            _ => 0,
                            //        })
                            //        .collect::<Vec<_>>();
                            //    g_.insert(right[i].to_string().unwrap(), rrow);
                            //    g.add_row(&row);
                            //});
                            println!("c running dlx");
                            let matchings = crate::dlx::solve(g);
                            println!("{:?}", matchings);
                        }
                        _ => return eprintln!("c there is no perfect sample"),
                    }

                    //println!("common: {:?}", common.iter().map(|s| s.to_string().unwrap()).collect::<Vec<_>>());
                    //println!("common: {:?}", common.len());
                }
            }
            _ => (),
        }
        //let local_count_vec = columns_vec.iter().map(|atom|;

        //for (_k, v) in lookup_table {
        //    if v > 0 {
        //        //dbg!(k.to_string().unwrap(),v);
        //        (0..v).for_each(|_| print!("#"));
        //        println!();
        //    }
        //}
    }
}

pub trait Sampler {
    fn k_greedy_search_show(
        &mut self,
        ignored_atoms: impl Iterator<Item = clingo::Symbol>,
        sample_size: Option<usize>,
    );
    fn naive_approach_representative_search_show(
        &mut self,
        ignored_atoms: impl Iterator<Item = clingo::Symbol>,
    );
    fn naive_approach_representative_search(
        &mut self,
        ignored_atoms: impl Iterator<Item = clingo::Symbol>,
    ) -> Vec<Vec<clingo::Symbol>>;
    fn template(&self) -> Vec<clingo::Symbol>;
    fn ext(&self, symbol: &clingo::Symbol) -> clingo::Literal; // TODO; generic
    fn covered(&mut self, under: &[clingo::Literal]) -> HashSet<clingo::Symbol>;
    fn within(&mut self, under: &[clingo::Literal]) -> HashSet<clingo::Symbol>;
    fn sat(&mut self, under: &[clingo::Literal]) -> bool;
    fn admits_perfect_sample(&mut self, under: &HashSet<clingo::Symbol>) -> bool;
}

impl Sampler for Navigator {
    fn k_greedy_search_show(
        &mut self,
        ignored_atoms: impl Iterator<Item = clingo::Symbol>,
        sample_size: Option<usize>,
    ) {
        let n = sample_size.unwrap_or(0);

        let mut cache = CACHE.lock().expect("cache lock is poisoned.");
        let mut seed = self.active_facets.clone();
        let seed_entry = seed.iter().map(|l| l.get_integer()).collect::<Vec<_>>();

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
        let lits = self.literals.clone();
        let mut i = 1;

        match n == 0 {
            true => loop {
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
                                .filter(|a| !to_ignore.contains(a))
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
                        solve_handle = ctl
                            .solve(clingo::SolveMode::YIELD, &seed)
                            .unwrap_unchecked();
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
                                .filter(|a| !to_ignore.contains(a))
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
                        solve_handle = ctl
                            .solve(clingo::SolveMode::YIELD, &seed)
                            .unwrap_unchecked();
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

    fn naive_approach_representative_search_show(
        &mut self,
        ignored_atoms: impl Iterator<Item = clingo::Symbol>,
    ) {
        let lits = self.literals.clone();

        let mut to_observe = self.inclusive_facets(&[]).0.to_hashset();
        ignored_atoms.for_each(|s| {
            to_observe.remove(&s);
        });

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
                            println!("Answer {:?}: ", i);
                            let atoms_strings = atoms.iter().map(|atom| {
                                atom.to_string().expect("atom to string conversion failed.")
                            });
                            atoms_strings.clone().for_each(|atom| print!("{} ", atom));

                            solve_handle.close().expect("closing solve handle failed.");

                            i += 1;
                            println!();
                        }
                        _ => continue, //
                    }
                }
            } else {
                if i == 1 {
                    println!("UNSATISFIABLE");
                }
                break;
            }
        }
    }

    fn naive_approach_representative_search(
        &mut self,
        ignored_atoms: impl Iterator<Item = clingo::Symbol>,
    ) -> Vec<Vec<clingo::Symbol>> {
        let lits = self.literals.clone();

        let mut to_observe = self.inclusive_facets(&[]).0.to_hashset();
        ignored_atoms.for_each(|s| {
            to_observe.remove(&s);
        });

        let ctl = Arc::get_mut(&mut self.control).expect("control error.");

        let mut collection = vec![];

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
                            collection.push(atoms);
                            solve_handle.close().expect("closing solve handle failed.");
                        }
                        _ => continue,
                    }
                }
            } else {
                if collection.is_empty() {
                    return collection;
                }
                break;
            }
        }

        collection
    }
    fn template(&self) -> Vec<clingo::Symbol> {
        let facets = self.current_facets.clone();
        self.literals
            .clone()
            .into_keys()
            .filter(|a| facets.0.contains(a))
            .collect::<Vec<_>>()
    }
    fn ext(&self, symbol: &clingo::Symbol) -> clingo::Literal {
        *unsafe { self.literals.get(symbol).unwrap_unchecked() }
    }

    fn covered(&mut self, under: &[clingo::Literal]) -> HashSet<clingo::Symbol> {
        unsafe {
            self.consequences(crate::navigator::EnumMode::Cautious, under)
                .unwrap_unchecked()
                .to_hashset()
        }
    }

    fn within(&mut self, under: &[clingo::Literal]) -> HashSet<clingo::Symbol> {
        unsafe {
            self.consequences(crate::navigator::EnumMode::Brave, under)
                .unwrap_unchecked()
                .to_hashset()
        }
    }

    fn sat(&mut self, under: &[clingo::Literal]) -> bool {
        self.satisfiable(under)
    }

    fn admits_perfect_sample(&mut self, under: &HashSet<clingo::Symbol>) -> bool {
        under
            .intersection(&self.current_facets.0.to_hashset())
            .count()
            == 0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn kniff() {
        let mut im = crate::dlx::Matrix::new(21);
        let arr = [
            [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1,
            ],
            [
                0, 0, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0,
            ],
            [
                1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            [
                0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0,
            ],
            [
                0, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            [
                0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
        ];
        let rows = arr
            .iter()
            .map(|v| v.iter().map(|u| *u == 1).collect::<Vec<_>>())
            .for_each(|v| {
                im.add_row(&v);
            });
        let sol = crate::dlx::solve(im);
        dbg!(sol);
    }
}

/*

                    let mean_occurences = observation_table
                        .iter()
                        .map(|(occurences, _)| occurences)
                        .sum::<usize>() as f32
                        / template_size as f32;
                    let std_occurences = observation_table
                        .iter()
                        .map(|(occurences, _)| (*occurences as f32 - mean_occurences).powf(2.0))
                        .sum::<f32>()
                        .sqrt();

                    let local_proper_chunks = observation_table
                        .iter()
                        .map(|(occurences_number, chunk)| (occurences_number, chunk, chunk.len()))
                        .filter(|(occurences_number, _, _)| **occurences_number > 1)
                        .collect::<Vec<_>>();
                    let n_local_proper_chunks = local_proper_chunks.len();
                    let rel_proper_chunk_sizes = local_proper_chunks
                        .iter()
                        .map(|(_, _, chunk_size)| *chunk_size as f32 / template_size as f32)
                        .collect::<Vec<_>>();
                    let mean_rel_proper_chunk_size =
                        rel_proper_chunk_sizes.iter().sum::<f32>() / n_local_proper_chunks as f32;
                    let std_rel_proper_chunk_size = rel_proper_chunk_sizes
                        .iter()
                        .map(|chunk_size| {
                            (*chunk_size as f32 - mean_rel_proper_chunk_size).powf(2.0)
                        })
                        .sum::<f32>()
                        .sqrt();

                    eprintln!(
                        "occ: {:.2}+-{:.2}\nrls: {:.2}+-{:.2}\n---",
                        mean_occurences,
                        std_occurences,
                        mean_rel_proper_chunk_size,
                        std_rel_proper_chunk_size,
                    );
*/
