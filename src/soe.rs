use crate::navigator::*;
use hashbrown::HashMap;
use itertools::Itertools;
use std::collections::HashSet;
use std::sync::Arc;

use crate::cache::CACHE;
use crate::dlx::Matrix;
use crate::translator::Atom;
use crate::utils::ToHashSet;

type Element = clingo::Symbol;

pub(crate) enum Heuristic {
    Ediv,
    Erep,
}
pub(crate) trait Cover<S>
where
    S: Sampler,
{
    fn search_perfect_sample_show(&mut self, sampler: &mut S, ignored_atoms: &[Element]);

    fn collect_show(
        &mut self,
        sampler: &mut S,
        route: &[clingo::Literal],
        ignored_atoms: &[Element],
        collection: HashSet<Vec<Element>>,
        template: &[Element],
    );
}
impl<S> Cover<S> for Heuristic
where
    S: Sampler,
{
    fn collect_show(
        &mut self,
        sampler: &mut S,
        route: &[clingo::Literal],
        ignored_atoms: &[Element],
        mut e: HashSet<Vec<Element>>,
        template: &[Element],
    ) {
        let template_size = template.len();

        eprintln!("c template size: {:?}", template_size);

        let mut freq_table: HashMap<clingo::Symbol, usize> = HashMap::new();
        template.iter().for_each(|atom| {
            freq_table.insert(*atom, 0);
        });
        let mut e_size = 0;
        let mut n_uniques = 0;

        match self {
            Self::Erep => {
                eprintln!("erep",);
                sampler.assisting_naive_approach_representative_search(
                    ignored_atoms,
                    route,
                    &mut e,
                    &mut e_size,
                    &mut freq_table,
                );

                //if exact_cover(&e, &template, template_size) {
                //    return;
                //}

                eprintln!("c splitting chunks");

                let (mut proper_chunk_atoms, mut unique_chunk_atoms) = (Vec::new(), Vec::new());
                let mut chunks_table: HashMap<usize, HashSet<clingo::Symbol>> = HashMap::new();
                let mut population_size = 0;
                freq_table.iter().for_each(|(atom, freq)| {
                    population_size += *freq;
                    let freq_chunk = chunks_table
                        .raw_entry_mut()
                        .from_key(freq)
                        .or_insert_with(|| (*freq, vec![*atom].to_hashset()));
                    freq_chunk.1.insert(*atom);
                    if *freq == 1 {
                        n_uniques += 1;
                        unique_chunk_atoms.push(atom);
                    } else {
                        proper_chunk_atoms.push(atom);
                    }
                });

                let ghd = n_uniques as f32 / template_size as f32;
                eprintln!("c ghd={:.2}", ghd);
                //for (k, v) in &chunks_table {
                //    //println!("{:?} {:?}", k, v.len());
                //    println!(
                //        "{:?} {:?}",
                //        k,
                //        v.iter().map(|s| s.to_string().unwrap()).collect::<Vec<_>>()
                //    );
                //}
                let div = 2f64.powf(entropy(&freq_table, population_size as f64));
                let r = {
                    let ts = template_size as f64;
                    1f64 - (ts - div).abs() / ts
                };
                for (bin_id, bin) in &chunks_table {
                    let bl = bin.len();
                    (0..bl).for_each(|_| print!("#"));
                    println!(" {:?} ({:?})", bin_id, bl);
                }
                println!(
                    "bins={:?},m={:.2},|A|={:?},r={:.2}",
                    chunks_table.len(),
                    div,
                    template_size,
                    r
                );
                println!(
                    "accuracy: {:.2}",
                    -((freq_table
                        .values()
                        .map(|f| (*f as f64 / population_size as f64).log2())
                        .sum::<f64>())
                        / population_size as f64)
                );

                if exact_cover(&e, &template, template_size) {
                    return;
                }
                // try to add n-1 answer sets s_1,...,s_n-1 for each unique atom where n-chunk is the smallest
                // proper chunk s.t. each s_i contains atom but proper chunk atom (try as much as
                // possible incrementally)

                /*
                let pigeons = template
                    .iter()
                    .filter(|atom| !unique_chunk_atoms.contains(atom))
                    .collect::<Vec<_>>();
                let pigeons_lits = pigeons
                    .iter()
                    .map(|atom| sampler.ext(atom))
                    .collect::<Vec<_>>();
                let (bc, cc) = (
                    sampler.within(&pigeons_lits),
                    sampler.covered(&pigeons_lits),
                );
                for pigeon in &pigeons {
                    println!("pigeon: {:?}", pigeon.to_string().unwrap());
                    let pigeons_lits = [sampler.ext(pigeon)];
                    let (bc, cc) = (
                        sampler.within(&pigeons_lits),
                        sampler.covered(&pigeons_lits),
                    );
                    let holes = bc.difference(&cc).clone().collect::<HashSet<_>>();
                    println!("{:?}", holes.len() < pigeons.len());
                    //filter(|atom| !unique_chunk_atoms.contains(atom))
                    println!(
                        "holes: {:?}",
                        holes
                            .iter()
                            .map(|s| s.to_string().unwrap())
                            .collect::<Vec<_>>()
                    );
                    println!(
                        "pigeons: {:?}",
                        pigeons
                            .iter()
                            .map(|s| s.to_string().unwrap())
                            .collect::<Vec<_>>()
                    );
                }
                //let unique_chunk_cc_fold = sampler.covered(&flip);
                let chunks_w_s = chunks_table
                    .iter()
                    .map(|(weight, chunk)| (weight, chunk, chunk.len()))
                    .collect::<Vec<_>>();
                let biggest_chunk = unsafe {
                    chunks_w_s
                        .iter()
                        .map(|(_, _, size)| size)
                        .position_max()
                        .and_then(|idx| chunks_w_s.get(idx))
                        .unwrap_unchecked()
                };

                let flip = biggest_chunk
                    .1
                    .iter()
                    .map(|s| sampler.ext(s).negate())
                    .collect::<Vec<_>>();
                let biggest_chunk_bc_fold = sampler.within(&flip);
                let biggest_chunk_cc_fold = sampler.covered(&flip);
                let fs = biggest_chunk_bc_fold
                    .difference(&biggest_chunk_cc_fold)
                    .clone()
                    .collect::<Vec<_>>();
                println!(
                    "biggest chunk: {:?}",
                    biggest_chunk.0 //.1
                                    //.iter()
                                    //.map(|s| s.to_string().unwrap())
                                    //.collect::<Vec<_>>()
                );
                */

                return;
            }
            Self::Ediv => {
                eprintln!("ediv",);
                let mut observed = vec![].to_hashset();
                sampler.assisting_k_greedy_search(
                    ignored_atoms,
                    route,
                    &mut e,
                    &mut e_size,
                    &mut observed,
                );

                let mut amount_covered = observed.len() as f32 / template_size as f32;
                println!("c amount covered: {:.2}", amount_covered);

                if exact_cover(&e, &template, template_size) {
                    return;
                }

                return;
            }
        }
    }
    fn search_perfect_sample_show(&mut self, sampler: &mut S, ignored_atoms: &[Element]) {
        let template = sampler.template();
        let template_size = template.len();
        #[cfg(feature = "with_stats")]
        {
            //eprintln!(
            //    "{:?}",
            //    &template
            //        .iter()
            //        .map(|s| s.to_string().unwrap())
            //        .collect::<Vec<_>>()
            //);
            eprintln!("c template size: {:?}", template_size);
        }

        //
        let mut f_lookup_table: HashMap<clingo::Symbol, usize> = HashMap::new();
        template.iter().for_each(|atom| {
            f_lookup_table.insert(*atom, 0);
        });

        let (mut incidence_matrix, mut incidence_matrix_rows, mut e, mut e_size) =
            (Matrix::new(template_size), vec![], vec![].to_hashset(), 0);

        match self {
            Self::Erep => {
                #[cfg(feature = "with_stats")]
                {
                    eprint!("c collecting initial representative subset...",);
                }
                // sample representivaley
                sampler.assisting_naive_approach_representative_search(
                    ignored_atoms,
                    &[],
                    &mut e,
                    &mut e_size,
                    &mut f_lookup_table,
                );
                #[cfg(feature = "with_stats")]
                {
                    eprintln!("done",);
                }
                let collection_as_vec = e.iter().collect::<Vec<_>>();
                collection_as_vec.iter().for_each(|answer_set| {
                    let row = template
                        .iter()
                        .map(|atom| answer_set.contains(atom))
                        .collect::<Vec<_>>();

                    incidence_matrix.add_row(&row);
                    incidence_matrix_rows.push(row);
                });
                #[cfg(feature = "with_stats")]
                {
                    eprint!("c exact cover check...",);
                }
                let exact_covers = crate::dlx::solve_all(incidence_matrix);
                if let Some(ec) = exact_covers.iter().next() {
                    #[cfg(feature = "with_stats")]
                    {
                        eprintln!("positive",);
                    }
                    let models = ec
                        .iter()
                        .map(|idx| unsafe { collection_as_vec.get_unchecked(*idx) });
                    for (i, model) in models.enumerate() {
                        println!("Answer {:?}:", i + 1);
                        model.iter().for_each(|atom| {
                            print!("{} ", unsafe { atom.to_string().unwrap_unchecked() })
                        });
                        println!();
                    }
                    return;
                }
                drop(collection_as_vec);
                //eprintln!(
                //    "c entropy={:2.}",
                //    entropy(&f_lookup_table, template_size as f32)
                //);

                let mut chunks_table: HashMap<usize, HashSet<clingo::Symbol>> = HashMap::new();
                let mut sum_over_freqs = 0;
                let mut uniques = 0;
                f_lookup_table.iter().for_each(|(atom, freq)| {
                    let freq_chunk = chunks_table
                        .raw_entry_mut()
                        .from_key(freq)
                        .or_insert_with(|| (*freq, vec![*atom].to_hashset()));
                    freq_chunk.1.insert(*atom);
                    sum_over_freqs += freq;
                    if *freq == 1 {
                        uniques += 1;
                    }
                });

                #[cfg(feature = "with_stats")]
                {
                    eprintln!("negative");
                    eprintln!("c ghd={:.2}", uniques as f32 / template_size as f32);

                    //for r in &incidence_matrix_rows {
                    //    for v in r {
                    //        match v {
                    //            true => print!("1"),
                    //            _ => print!("0"),
                    //        }
                    //    }
                    //    println!();
                    //}

                    //for model in e.iter().map(|v| stringify(&v)) {
                    //    println!("{:?}", model);
                    //}
                    //for (k, v) in &f_lookup_table {
                    //    println!("{:?} : {:?}", k.to_string().unwrap(), v);
                    //}
                    for (k, v) in &chunks_table {
                        println!(
                            //"{:?} : {:?}",
                            //k,
                            //stringify(&v.clone().into_iter().collect::<Vec<_>>())
                            "{:?} {:?}",
                            k,
                            v.len()
                        );
                    }
                }

                let mut proper_chunk_atoms = chunks_table
                    .iter()
                    .filter(|(freq, _)| **freq > 1)
                    .map(|(_, chunk)| chunk)
                    .fold(vec![].to_hashset(), |acc, chunk| {
                        acc.union(chunk).cloned().collect::<HashSet<_>>()
                    });
                let mut facets_table: HashMap<
                    clingo::Symbol,
                    (
                        HashSet<clingo::Symbol>,
                        HashSet<clingo::Symbol>,
                        HashSet<clingo::Symbol>,
                    ),
                > = HashMap::new();
                //println!("maxw: {:?}", max_weighted_facet.to_string().unwrap());

                let uniques = template
                    .iter()
                    .filter(|atom| !proper_chunk_atoms.contains(atom))
                    .collect::<Vec<_>>();
                let ulits = uniques
                    .iter()
                    .map(|s| sampler.ext(s).negate())
                    .collect::<Vec<_>>();
                let acbc = sampler.within(&ulits);
                let accc = sampler.covered(&ulits);
                let anti_concept = acbc.difference(&accc).cloned().collect::<Vec<_>>();
                println!("anti_concept");
                for s in accc {
                    print!("{:?}", s.to_string().unwrap());
                }
                println!();

                println!("{:?}", e_size);
                sampler.assisting_k_greedy_search(
                    &[],
                    //&anti_concept.iter().map(|s| sampler.ext(s)).collect::<Vec<_>>(),
                    &ulits,
                    &mut e,
                    &mut e_size,
                    &mut vec![].to_hashset(),
                );
                println!("{:?}", e_size);

                //let max_weighted_facet = unsafe {
                //    anti_concept
                //        .iter()
                //        .map(|atom| {
                //            let lit = sampler.ext(atom);

                //            let bc = sampler.within(&[lit]);
                //            let cc = sampler.covered(&[lit]);
                //            bc.difference(&cc).count()

                //        })
                //        .position_min()
                //        .and_then(|idx| anti_concept.get(idx))
                //        .unwrap_unchecked()
                //};
                //println!("maxw: {:?}", max_weighted_facet.to_string().unwrap());
                return;

                while !proper_chunk_atoms.is_empty() {
                    let min_chunk = unsafe {
                        chunks_table
                            .keys()
                            .filter(|freq| **freq > 1)
                            .min()
                            .and_then(|freq| chunks_table.get(freq))
                            .map(|set| set.iter().collect::<Vec<_>>())
                            .unwrap_unchecked()
                    };

                    let max_weighted_facet = unsafe {
                        min_chunk
                            .iter()
                            .map(|atom| {
                                let lit = sampler.ext(atom);

                                let bc = sampler.within(&[lit]);
                                let cc = sampler.covered(&[lit]);
                                let fs = bc.difference(&cc).cloned().collect::<HashSet<_>>();

                                let count = fs.len(); // > 0
                                facets_table.insert(**atom, (bc, cc, fs));
                                count
                            })
                            .position_min()
                            .and_then(|idx| min_chunk.get(idx))
                            .unwrap_unchecked()
                    };
                    println!("maxw: {:?}", max_weighted_facet.to_string().unwrap());

                    ditify(
                        sampler,
                        &mut facets_table,
                        &max_weighted_facet,
                        &mut proper_chunk_atoms,
                    );
                    //println!(
                    //    "{:?}",
                    //    proper_chunk_atoms
                    //        .iter()
                    //        .map(|s| s.to_string().unwrap())
                    //        .collect::<Vec<_>>()
                    //);
                    return;
                }

                return;
            }
            Self::Ediv => {
                println!("c collecting E");
                sampler.assisting_k_greedy_search(
                    &[],
                    &[],
                    &mut e,
                    &mut e_size,
                    &mut vec![].to_hashset(),
                );
                let mut amount_covered =
                    f_lookup_table.values().sum::<usize>() as f32 / template_size as f32; // consider vec
                let mut indits: HashMap<clingo::Symbol, HashSet<clingo::Symbol>> = HashMap::new();
                println!("c covered {:.2}", amount_covered);
                while amount_covered < 1.0 {
                    let ue = f_lookup_table
                        .iter()
                        .filter(|(_, count)| **count == 1)
                        .map(|(atom, _)| *atom)
                        .collect::<HashSet<_>>();
                    let se = f_lookup_table
                        .iter()
                        .filter(|(_, count)| **count == 0)
                        .map(|(atom, _)| *atom)
                        .collect::<Vec<_>>();
                    if se.iter().any(|atom| sampler.overlap(&[*atom])) {
                        println!("non-existing");
                        return;
                    }

                    /*
                    se.iter().for_each(|atom| {
                        let cap = sampler
                            .covered(&[sampler.ext(atom)])
                            .intersection(&ue)
                            .cloned()
                            .collect::<HashSet<_>>();
                        if !cap.is_empty() {
                            let indit = indits
                                .raw_entry_mut()
                                .from_key(atom)
                                .or_insert_with(|| (*atom, cap.clone()));
                            indit.1.extend(cap);
                        }
                    });
                    println!("ue: {:?}", stringify(&ue.into_iter().collect::<Vec<_>>()));
                    println!("se: {:?}", stringify(&se));
                    if indits.is_empty() {
                        let min_weighted = unsafe {
                            se.iter()
                                .map(|atom| {
                                    let lit = sampler.ext(atom);
                                    let bc = sampler.within(&[lit]);
                                    let cc = sampler.covered(&[lit]);
                                    bc.difference(&cc).count()
                                })
                                .position_min()
                                .and_then(|idx| se.get(idx))
                                .unwrap_unchecked()
                        };
                        println!("minw: {:?}", min_weighted.to_string().unwrap());
                    } else {
                        for (k, v) in &indits {
                            println!(
                                "{:?} {:?}",
                                k.to_string().unwrap(),
                                v.iter().map(|s| s.to_string().unwrap()).collect::<Vec<_>>()
                            );
                        }
                    }
                    */
                    return;

                    // according to knuth: choose a s.t. number of subsets compatible with a is
                    // minimal, i.e., minimal absolute weight, however for performance we choose
                    // facet-counting weight
                }
                /*
                for (i, atom) in template.iter().enumerate() {
                    #[cfg(feature = "with_stats")]
                    {
                        eprint!(
                            "c collecting [{:.2}]...",
                            (i + 1) as f32 / template_size as f32
                        );
                        // TODO: progress bar
                    }
                    //dbg!(&atom.to_string().unwrap());
                    //sampler.k_greedy_search_show(std::iter::empty(),None);
                    sampler.assisting_k_greedy_search(
                        std::iter::empty(),
                        &[sampler.ext(atom)],
                        &mut collection,
                        &mut collection_size,
                        &mut f_lookup_table,
                    );
                    let stats = stats(&f_lookup_table, collection_size as f32);
                    let covered = 1.0 - (stats.0.len() as f32 / template_size as f32);
                    let entropy = stats.1;
                    let ghd = stats.2;
                    #[cfg(feature = "with_stats")]
                    {
                        eprintln!("done",);
                        eprintln!(
                            "c {:.2} | siz={:?} cov={:.2} ghd={:.2} ent={:.2}",
                            1.0 - (2.0 - (covered + ghd)),
                            collection_size,
                            covered,
                            ghd,
                            entropy
                        );
                    }
                    collection.iter().for_each(|answer_set| {
                        let row = template
                            .iter()
                            .map(|atom| answer_set.contains(atom))
                            .collect::<Vec<_>>();

                        incidence_matrix.add_row(&row);
                        incidence_matrix_rows.push(row);
                    });

                    #[cfg(feature = "with_stats")]
                    {
                        //for r in &incidence_matrix_rows {
                        //    for v in r {
                        //        match v {
                        //            true => print!("1"),
                        //            _ => print!("0"),
                        //        }
                        //    }
                        //    println!();
                        //}
                        /*
                            for model in collection.iter().map(|v| stringify(&v)) {
                                println!("{:?}", model);
                            }
                            for (k, v) in &f_lookup_table {
                                println!("{:?} : {:?}", k.to_string().unwrap(), v);
                            }
                        */
                        if !crate::dlx::solve_all(incidence_matrix.clone()).is_empty() {
                            eprintln!(" found one");
                            return;
                        }
                    }
                }
                //
                return;
                */
            }
        }

        /*
        #[cfg(feature = "with_stats")]
        {
            eprint!("c sampling initial collection...");
        }
        let init_sample = sampler.assisting_naive_approach_representative_search(
            ignored_atoms,
            &[],
            &mut vec![].to_hashset(),
            &mut 0,
            &mut f_lookup_table,
        );

        #[cfg(feature = "with_stats")]
        {
            eprintln!("done");
            eprintln!("c initializing incidence matrix");
        }
        let mut incidence_matrix = Matrix::new(template_size);
        let mut f_lookup_table: HashMap<clingo::Symbol, usize> = HashMap::new();
        template.iter().for_each(|atom| {
            f_lookup_table.insert(*atom, 0);
        });

        let (mut sample_size, mut incidence_matrix_rows) = (0, vec![]);

        init_sample.iter().for_each(|answer_set| {
            let row = template
                .iter()
                .map(|atom| {
                    let entry = answer_set.contains(atom);
                    if entry {
                        let count = unsafe { f_lookup_table.get_mut(atom).unwrap_unchecked() };
                        *count += 1;
                    }
                    entry
                })
                .collect::<Vec<_>>();
            sample_size += 1;

            incidence_matrix.add_row(&row);
            incidence_matrix_rows.push(row);
        });

        #[cfg(feature = "with_stats")]
        {
            eprintln!("c init sample size {:?}", sample_size);
            eprint!("c perfect sample checking...");
        }
        let mut exact_covers = crate::dlx::solve_all(incidence_matrix); // TODO: impl first found
        let found_perfect_sample = !exact_covers.is_empty();
        if found_perfect_sample {
            #[cfg(feature = "with_stats")]
            {
                eprintln!("positive");
            }
            // NOTE: consider dropping init_sample and reading output from incidence_matrix_rows
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

        #[cfg(feature = "with_stats")]
        {
            eprintln!("negative");
            for r in &incidence_matrix_rows {
                for v in r {
                    match v {
                        true => print!("1"),
                        _ => print!("0"),
                    }
                }
                println!();
            }
            eprint!("c chunking...");
        }
        f_lookup_table.retain(|_, count| *count > 0); // NOTE: removing atoms that are projected away
                                                      //drop(init_sample);

        let mut chunks: HashMap<usize, HashSet<clingo::Symbol>> = HashMap::new();
        let (mut n_uniques, mut summed_occurences) = (0, 0);

        f_lookup_table.iter().for_each(|(k, v)| {
            if *v == 1 {
                n_uniques += 1;
            }
            summed_occurences += v;
            let c = chunks
                .raw_entry_mut()
                .from_key(v)
                .or_insert_with(|| (*v, HashSet::new()));
            c.1.insert(*k);
        });

        #[cfg(feature = "with_stats")]
        {
            eprintln!("done");
        }
        //let mut w_lookup_table: HashMap<f32, usize> = HashMap::new();
        //let (uniques_chunk, proper_chunks) = (
        //    unsafe { chunks.get(&1).unwrap_unchecked() },
        //    chunks
        //        .iter()
        //        .filter(|(count, _)| **count > 1)
        //        .map(|(count, chunk)| {
        //            let weight = *count as f32 / sample_size as f32;
        //            chunk})
        //        .collect::<Vec<_>>(),
        //);
        //let (mut i0, mut i1, mut i2) = (
        //    vec![].to_hashset(),
        //    vec![].to_hashset(),
        //    vec![].to_hashset(),
        //);

        let n_chunks = chunks.len();
        let (ghd, err, (maf, maxmaf)) = (
            n_uniques as f32 / template_size as f32,
            1f32 - (template_size as f32 / summed_occurences as f32), // NOTE: summed_occurences >= template_size since each atoms occurs at least once
            {
                let mean = chunks.keys().sum::<usize>() as f32 / n_chunks as f32;
                (mean, *unsafe { chunks.keys().max().unwrap_unchecked() })
            },
        );
        let scatter_factor = n_chunks as f32 / template_size as f32;
        let mut chunk_sizes = chunks
            .iter()
            .map(|(_, chunk)| chunk.len())
            .collect::<Vec<_>>();
        chunk_sizes.sort_unstable();
        let (uniques_chunk, (chunk_sizes_mean, chunk_sizes_max)) =
            (unsafe { chunks.get(&1).unwrap_unchecked() }, {
                let mean = template_size as f32 / n_chunks as f32;

                (mean, *unsafe {
                    chunk_sizes.iter().max().unwrap_unchecked()
                })
            });

        #[cfg(feature = "with_stats")]
        {
            eprintln!("c ghd\terr\tscf\taaf\tmaf\tacs\tmcs");
            eprintln!(
                "c {:.2}\t{:.2}\t{:.2}\t{:.2}\t{:.2}\t{:.2}\t{:.2}",
                ghd,
                err,
                scatter_factor,
                maf / sample_size as f32,
                maxmaf as f32 / sample_size as f32,
                chunk_sizes_mean / template_size as f32,
                chunk_sizes_max as f32 / template_size as f32
            );
            eprintln!("{:?}", chunks.keys().collect::<Vec<_>>());
            eprintln!(
                "{:?}",
                chunks
                    .iter()
                    //.filter(|(k, v)| **k > 1)
                    .map(|(_, v)| v.len())
                    .collect::<Vec<_>>()
            );
        }
        */
        /*
        let mut content = false;
        let mut chunk_weights = chunks.keys().collect::<Vec<_>>();
        chunk_weights.sort_unstable();
        let mut sorted_chunk_weights = chunk_weights.iter().rev();
        while !found_perfect_sample || !content {
            let weight = unsafe { sorted_chunk_weights.next().unwrap_unchecked() };
            let target_chunk = unsafe {
                chunks
                    .get(*weight)
                    .unwrap_unchecked()
            };
            let target_chunk_template = target_chunk.iter().collect::<Vec<_>>();

            #[cfg(feature = "with_stats")]
            {
                eprint!(
                    "c flattening... {:.2} {:.2}",
                    target_chunk.len() as f32 / template_size as f32,
                    **weight as f32 / sample_size as f32
                );
            }
            println!(
                "\ntct: {:?}",
                target_chunk_template
                    .iter()
                    .map(|s| s.to_string().unwrap())
                    .collect::<Vec<_>>()
            );

            let covered_contents = target_chunk_template
                .iter()
                .map(|s| sampler.covered(&[sampler.ext(s)]))
                .collect::<Vec<_>>();
            println!(
                "cc: {:?}",
                covered_contents
                    .iter()
                    .map(|v| v.iter().map(|s| s.to_string().unwrap()).collect::<Vec<_>>())
                    .collect::<Vec<_>>()
            );
            #[cfg(feature = "with_stats")]
            {
                let mut iter = covered_contents.iter().cloned();
                let common = unsafe {
                    iter.next()
                        .map(|a| {
                            iter.fold(a, |b, c| {
                                b.intersection(&c).cloned().collect::<HashSet<_>>()
                            })
                        })
                        .unwrap_unchecked()
                };
                eprintln!(" {:?}", common.len() as f32 / template_size as f32);
            }

            let target_chunk_atoms = f_lookup_table
                .keys()
                .filter(|k| !target_chunk.contains(k))
                .collect::<Vec<_>>();

            let mut holes_and_pigeons: HashMap<Vec<bool>, HashSet<clingo::Symbol>> = HashMap::new();
            let mut holes_exist = false;
            covered_contents.iter().enumerate().for_each(|(idx, ccs)| {
                let row = target_chunk_atoms
                    .iter()
                    .map(|a| ccs.contains(a))
                    .collect::<Vec<_>>();
                //println!("row={:?}", row);
                if row.iter().any(|v| *v) {
                    holes_exist = true;
                    let atom = unsafe { target_chunk_template.get_unchecked(idx) };
                    let c = holes_and_pigeons
                        .raw_entry_mut()
                        .from_key(&row)
                        .or_insert_with(|| (row, vec![**atom].to_hashset()));
                    c.1.insert(**atom);
                }
            });
            //println!("holes_and_pigeons: {:?}", holes_and_pigeons);
            println!();
            for (k, v) in holes_and_pigeons.iter().map(|(k, v)| {
                (
                    target_chunk_atoms
                        .clone()
                        .iter()
                        .enumerate()
                        .filter(|(i, _)| k[*i] == true)
                        .map(|(_, a)| a.to_string().unwrap())
                        .collect::<Vec<_>>(),
                    v.iter().map(|s| s.to_string().unwrap()).collect::<Vec<_>>(),
                )
            }) {
                println!("{:?} {:?}", k, v);
            }
            println!();

            //println!(
            //    "tca: {:?}",
            //    target_chunk_atoms
            //        .iter()
            //        .map(|s| s.to_string().unwrap())
            //        .collect::<Vec<_>>()
            //);

            #[cfg(feature = "with_stats")]
            {
                eprintln!("c holes? {:?}", holes_exist);
            }

            //return;
        }
        */

        return;

        /*
        match self {
            Self::Unnamed => {
                eprintln!("c starting heuristic unnamed");
                {
                    // sets of atoms clustered by their number of occurences
                    let mut chunks: HashMap<usize, HashSet<clingo::Symbol>> = HashMap::new();

                    let (mut uniques, mut value) = (0, 0);

                    rf_lookup_table.iter().for_each(|(k, v)| {
                        if *v == 1 {
                            uniques += 1;
                        }
                        value += v;
                        let c = chunks
                            .raw_entry_mut()
                            .from_key(v)
                            .or_insert_with(|| (*v, HashSet::new()));
                        c.1.insert(*k);
                    });

                    let uniques_chunk = unsafe { chunks.get(&1).unwrap_unchecked() };

                    let (uni, err) = (
                        uniques as f32 / template_size as f32,
                        1f32 - (template_size as f32 / value as f32),
                    ); // NOTE: value >= template_size
                    eprintln!("c uni: {:.2}\terr: {:.2}", uni, err,);

                    let deviations_chunk_size =
                        chunks.keys().map(|size| (*size - 1)).collect::<Vec<_>>();
                    eprintln!(
                        "c chunk size deviation: {:.2} {:?}",
                        deviations_chunk_size.iter().sum::<usize>() as f32 / chunks.len() as f32,
                        deviations_chunk_size.iter().max().unwrap()
                    );
                    eprintln!(
                        "c chunks variability: {:.2}",
                        chunks.keys().count() as f32 / template_size as f32,
                    );

                    let uniques_chunk_template = uniques_chunk.iter().collect::<Vec<_>>();
                    let covered_under = uniques_chunk_template
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

                    // check whether perfect sample is out of question
                    let mut perfect_sample_possible = sampler.admits_perfect_sample(&common);
                    match perfect_sample_possible {
                        true => {
                            eprintln!("c generating unique chunk template");
                            let uniques_template = uniques_chunk.iter().collect::<Vec<_>>();
                            println!(
                                "{:?}",
                                uniques_template
                                    .iter()
                                    .map(|s| s.to_string().unwrap())
                                    .collect::<Vec<_>>()
                            );

                            // for all or next smallest?
                            // for all
                            let proper_chunk_atoms = rf_lookup_table
                                .keys()
                                .filter(|k| !uniques_chunk.contains(k))
                                .collect::<Vec<_>>();

                            // next smallest
                            //let next_to_unique = unsafe {observation_table.keys().filter(|k| **k > 1).min().unwrap_unchecked()};
                            //let fold_proper_chunks = unsafe  { observation_table.get(next_to_unique).unwrap_unchecked() };

                            eprintln!(
                                "{:?}",
                                proper_chunk_atoms
                                    .iter()
                                    .map(|s| s.to_string().unwrap())
                                    .collect::<Vec<_>>()
                            );

                            let mut inevitables: HashMap<Vec<bool>, HashSet<clingo::Symbol>> =
                                HashMap::new();
                            covered_under.iter().enumerate().for_each(
                                |(uniques_idx, cautious_consequences)| {
                                    let row = proper_chunk_atoms
                                        .iter()
                                        .map(|atom| cautious_consequences.contains(atom))
                                        .collect::<Vec<_>>();
                                    if row.iter().any(|v| *v) {
                                        perfect_sample_possible = false;
                                        let atom = unsafe {
                                            uniques_chunk_template.get_unchecked(uniques_idx)
                                        };
                                        let c = inevitables
                                            .raw_entry_mut()
                                            .from_key(&row)
                                            .or_insert_with(|| (row, vec![**atom].to_hashset()));
                                        c.1.insert(**atom);
                                    }
                                },
                            );
                            println!();
                            for (k, v) in inevitables.iter().map(|(k, v)| {
                                (
                                    proper_chunk_atoms
                                        .clone()
                                        .iter()
                                        .enumerate()
                                        .filter(|(i, _)| k[*i] == true)
                                        .map(|(_, a)| a.to_string().unwrap())
                                        .collect::<Vec<_>>(),
                                    v.iter().map(|s| s.to_string().unwrap()).collect::<Vec<_>>(),
                                )
                            }) {
                                println!("{:?} {:?}", k, v);
                            }
                            println!();

                            let mut rm_cols = vec![];
                            inevitables.values().for_each(|symbols| {
                                symbols.iter().for_each(|symbol| {
                                    rm_cols.push(unsafe {
                                        template.iter().position(|x| x == symbol).unwrap_unchecked()
                                    });
                                })
                            });

                            println!(
                                "{:?}",
                                template
                                    .iter()
                                    .map(|s| s.to_string().unwrap())
                                    .collect::<Vec<_>>()
                            );
                            println!();
                            for r in &rows {
                                for v in r {
                                    match v {
                                        true => print!("1"),
                                        _ => print!("0"),
                                    }
                                }
                                println!();
                            }
                            println!();
                            let (mut idxs, mut lookup_table_flattened) =
                                (vec![], HashMap::<usize, usize>::new());
                            let flattened_sample = rows
                                .iter()
                                .enumerate()
                                .filter(|(_, row)| {
                                    !rm_cols
                                        .iter()
                                        .any(|idx| unsafe { *row.get_unchecked(*idx) })
                                })
                                .map(|(k, row)| {
                                    idxs.push(k);
                                    for (j, a) in row.iter().enumerate() {
                                        match !rm_cols.contains(&j) {
                                            true => match a {
                                                true => print!("1"),
                                                _ => print!("0"),
                                            },
                                            _ => print!("x"),
                                        }
                                    }
                                    println!();
                                    let r = row
                                        .iter()
                                        .enumerate()
                                        .filter(|(i, _)| !rm_cols.contains(i))
                                        .map(|(i, bit)| {
                                            if *bit {
                                                let count = lookup_table_flattened
                                                    .raw_entry_mut()
                                                    .from_key(&i)
                                                    .or_insert_with(|| (i, 1));
                                                *count.1 += 1;
                                            }
                                            *bit
                                        })
                                        .collect::<Vec<_>>();

                                    r
                                })
                                .collect::<Vec<_>>();
                            println!();
                            let mut flattened_incidence_matrix =
                                Matrix::new(template_size - rm_cols.len());

                            println!("{:?}", flattened_sample.len());
                            println!("{:?}", rows.len());

                            if !perfect_sample_possible {
                                eprintln!("c there is no perfect sample");
                                eprintln!("c starting optimization");
                            }

                            match flattened_sample.len() == rows.len() {
                                true => {
                                    eprintln!("c starting max-weighted search");
                                    todo!(
                                        "det max-weighted
                                               -> keep only biggest answer set of max-weighted
                                               -> resample withing subspace of max-weighted
                                               -> check"
                                    )
                                }
                                _ => {
                                    flattened_sample
                                        .iter()
                                        .for_each(|r| flattened_incidence_matrix.add_row(&r));
                                    eprintln!(
                                        "c flattened by {:.2}",
                                        1.0 - (flattened_sample.len() as f32 / rows.len() as f32)
                                    );

                                    let matchings =
                                        crate::dlx::solve_all(flattened_incidence_matrix);
                                    //println!("{:?}", matchings);
                                    match matchings.len() > 0 {
                                        true => {
                                            eprintln!("c flattening successfull");

                                            let exact_cover = unsafe {
                                                matchings.iter().next().unwrap_unchecked()
                                            };
                                            let ignore = idxs
                                                .iter()
                                                .filter(|i| !exact_cover.contains(i))
                                                .collect::<HashSet<_>>();

                                            for (j, model) in init_sample
                                                .iter()
                                                .enumerate()
                                                .filter(|(i, _)| !ignore.contains(i))
                                            {
                                                println!("Answer {:?}:", j + 1);
                                                model.iter().for_each(|atom| {
                                                    print!("{} ", unsafe {
                                                        atom.to_string().unwrap_unchecked()
                                                    })
                                                });
                                                println!();
                                            }
                                            return;
                                        }
                                        _ => {
                                            let represented_after_flattening =
                                                lookup_table_flattened
                                                    .keys()
                                                    .collect::<HashSet<_>>();
                                            let target_atoms = inevitables.keys().fold(
                                                vec![].to_hashset(),
                                                |s, v| {
                                                    s.union(&v.to_hashset())
                                                        .cloned()
                                                        .collect::<HashSet<_>>()
                                                },
                                            );
                                            dbg!(inevitables);
                                            dbg!(proper_chunk_atoms);
                                            //let missing_atoms_after_flattening = template
                                            //    .iter()
                                            //    .enumerate()
                                            //    .filter(|(i, a)| {
                                            //        !represented_after_flattening.contains(i)
                                            //            && proper_chunk_atoms.clone().contains(&a)
                                            //    })
                                            //    .map(|(_, atom)| atom)
                                            //    .collect::<HashSet<_>>();

                                            //println!(
                                            //    "c missing after flattening: {:?}",
                                            //    missing_atoms_after_flattening.len() as f32
                                            //        / template_size as f32
                                            //);
                                            //dbg!(missing_atoms_after_flattening
                                            //    .iter()
                                            //    .map(|s| s.to_string().unwrap())
                                            //    .collect::<Vec<_>>());
                                            todo!(
                                            "for each entirely missing atom:
                                                say m in [k,m] check whether [l,m] subset cc(m), if yes, ignore"
                                        );
                                        }
                                    }
                                }
                            }
                        }
                        _ => return eprintln!("c there is no perfect sample"),
                    }

                    //println!("common: {:?}", common.iter().map(|s| s.to_string().unwrap()).collect::<Vec<_>>());
                    //println!("common: {:?}", common.len());
                }
            }
            _ => (),
        }
        */
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
    fn assisting_k_greedy_search(
        &mut self,
        ignored_atoms: &[Element],
        under: &[clingo::Literal],
        collection: &mut HashSet<Vec<clingo::Symbol>>,
        collection_size: &mut usize,
        observed: &mut HashSet<Element>,
    );
    fn naive_approach_representative_search_show(
        &mut self,
        ignored_atoms: impl Iterator<Item = clingo::Symbol>,
    );
    fn assisting_naive_approach_representative_search(
        &mut self,
        ignored_atoms: &[Element],
        under: &[clingo::Literal],
        collection: &mut HashSet<Vec<clingo::Symbol>>,
        collection_size: &mut usize,
        lookup_table: &mut HashMap<clingo::Symbol, usize>,
    );
    fn template(&self) -> Vec<clingo::Symbol>;
    fn template_under(&mut self, under: &[clingo::Literal]) -> Vec<clingo::Symbol>;
    fn ext(&self, symbol: &clingo::Symbol) -> clingo::Literal; // TODO; generic
    fn covered(&mut self, under: &[clingo::Literal]) -> HashSet<clingo::Symbol>;
    fn within(&mut self, under: &[clingo::Literal]) -> HashSet<clingo::Symbol>;
    fn sat(&mut self, under: &[clingo::Literal]) -> bool;
    fn admits_perfect_sample(&mut self, under: &HashSet<clingo::Symbol>) -> bool;
    fn overlap(&mut self, facets: &[clingo::Symbol]) -> bool;
    fn give_one(&mut self, facets: &[clingo::Symbol]) -> Vec<clingo::Symbol>;
}

impl Sampler for Navigator {
    // TODO!
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

    fn assisting_k_greedy_search(
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
        //println!("to_ignore: {:?}", stringify(&to_ignore));

        let ctl = Arc::get_mut(&mut self.control).expect("control error.");
        let mut solve_handle = unsafe {
            ctl.solve(clingo::SolveMode::YIELD, &seed)
                .unwrap_unchecked()
        };
        let lits = self.literals.clone(); // TODO: could be clone only once and given as argument?
                                          //for (k, v) in &lits {
                                          //    println!("{:?} {:?}", k.to_string().unwrap(), v);
                                          //}

        loop {
            unsafe { solve_handle.resume().unwrap_unchecked() };

            if let Ok(Some(model)) = solve_handle.model() {
                if let Ok(atoms) = model.symbols(clingo::ShowType::SHOWN) {
                    let non_ignored_atoms = atoms
                        .iter()
                        .filter(|a| !to_ignore.contains(a))
                        .map(|symbol| unsafe { lits.get(symbol).unwrap_unchecked() }.negate())
                        .collect::<Vec<_>>();
                    println!("atoms: {:?}", stringify(&atoms));
                    //println!("seed: {:?}", seed);
                    //println!("non_ignored_atoms: {:?}", non_ignored_atoms);

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

    fn assisting_naive_approach_representative_search(
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
                                println!("atoms: {:?}", stringify(&atoms));
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

    fn overlap(&mut self, facets: &[clingo::Symbol]) -> bool {
        println!("atoms={:?}", stringify(facets));
        let a =
            self.inclusive_facets(&facets.iter().map(|atom| self.ext(atom)).collect::<Vec<_>>());
        println!("fpi={:?}", stringify(&a.0));
        let (n, mut seen) = (a.len(), facets.to_vec().to_hashset());
        n > a
            .iter()
            .map(|atom| {
                self.inclusive_facets(&[self.ext(atom)])
                    .iter()
                    .filter(|facet| seen.insert(**facet))
                    .count()
            })
            .sum::<usize>()
    }

    fn give_one(&mut self, facets: &[clingo::Symbol]) -> Vec<clingo::Symbol> {
        self.find_one(&facets.iter().map(|s| self.ext(s)).collect::<Vec<_>>())
            .unwrap()
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
        let sol = crate::dlx::solve_all(im);
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

pub(crate) fn stringify(v: &[clingo::Symbol]) -> Vec<String> {
    v.iter()
        .map(|s| unsafe { s.to_string().unwrap_unchecked() })
        .collect::<Vec<_>>()
}

fn stats(
    lookup_table: &HashMap<clingo::Symbol, usize>,
    sample_size: f32,
) -> (Vec<clingo::Symbol>, f32, f32) {
    let (mut uniques, mut apparents) = (0, 0);
    let mut missing_atoms = vec![];
    let entropy = lookup_table
        .iter()
        .map(|(atom, count)| {
            match *count == 0 {
                true => missing_atoms.push(*atom),
                _ => {
                    apparents += 1;
                    if *count == 1 {
                        uniques += 1;
                    }
                }
            }
            *count as f32 / sample_size
        })
        .map(|probability| probability * (1.0 / probability).log2())
        //.map(|probability| probability * (1.0 / probability).ln())
        .sum::<f32>();

    (missing_atoms, entropy, uniques as f32 / apparents as f32)
}

fn entropy(lookup_table: &HashMap<clingo::Symbol, usize>, sample_size: f64) -> f64 {
    -lookup_table
        .iter()
        .map(|(_, count)| *count as f64 / sample_size)
        //.map(|probability| probability * probability.log2())
        .map(|probability| {
            println!("{:?}", probability);
            probability * probability.log2()
        })
        .sum::<f64>()
}

fn ditify(
    sampler: &mut impl Sampler,
    facets_table: &mut HashMap<
        clingo::Symbol,
        (
            HashSet<clingo::Symbol>,
            HashSet<clingo::Symbol>,
            HashSet<clingo::Symbol>,
        ),
    >,
    inspect: &clingo::Symbol,
    current_indits: &mut HashSet<clingo::Symbol>,
) {
    let entry = unsafe { facets_table.get(inspect).unwrap_unchecked() };
    let mut seen = entry.2.clone();
    println!("{:?}", current_indits);
    entry.1.iter().for_each(|f| {
        current_indits.remove(f);
    });
    let mut n = 0;
    let m = entry
        .2
        .iter()
        .map(|atom| {
            n += 1;
            let lit = sampler.ext(atom);
            sampler
                .within(&[lit])
                .difference(&sampler.covered(&[lit]))
                .filter(|f| seen.insert(**f))
                .count()
        })
        .sum::<usize>();
    if n > m {
        println!("c overlap");
    }
    println!("{:?}", current_indits);

    //println!("fpi={:?}", stringify(&a.0));
    //let (n, mut seen) = (a.len(), facets.to_vec().to_hashset());
    //n > a
    //    .iter()
    //    .map(|atom| {
    //        self.inclusive_facets(&[self.ext(atom)])
    //            .iter()
    //            .filter(|facet| seen.insert(**facet))
    //            .count()
    //    })
    //    .sum::<usize>()
}

fn exact_cover(e: &HashSet<Vec<Element>>, template: &[Element], n_cols: usize) -> bool {
    let mut im = Matrix::new(n_cols);
    let collection = e.iter().collect::<Vec<_>>();
    println!("len {:?}", collection.len());

    collection.iter().for_each(|answer_set| {
        let row = template
            .iter()
            .map(|atom| answer_set.contains(atom))
            .collect::<Vec<_>>();

        im.add_row(&row);
    });
    eprint!("c exact cover check...",);
    let exact_covers = crate::dlx::solve_all(im);
    if let Some(ec) = exact_covers.iter().next() {
        eprintln!("positive");
        let models = ec
            .iter()
            .map(|idx| unsafe { collection.get_unchecked(*idx) });
        for (i, model) in models.enumerate() {
            println!("Answer {:?}:", i + 1);
            model
                .iter()
                .for_each(|atom| print!("{} ", unsafe { atom.to_string().unwrap_unchecked() }));
            println!();
        }
        return true;
    } else {
        eprintln!("negative");
        return false;
    }
}
