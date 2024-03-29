use std::fmt::Display;
use std::time::Instant;

use clingo::{Literal, Symbol};
use hashbrown::HashMap;
use rand::seq::SliceRandom;

use crate::asnc::AsnC;
use crate::config::CONFIG;
use crate::navigator::{filter, GoalOrientedNavigation, Mode, Navigator, Weight};
use crate::soe::{Cover, Sampler};
use crate::utils::{Repr, Route, ToSymbol};

pub type Input<'a> = std::str::SplitWhitespace<'a>;

pub fn parse_mode(input: (Option<&str>, Option<&str>)) -> Option<Mode> {
    match input {
        (Some("--goal-oriented"), None)
        | (Some("--go"), None)
        | (Some("--fc"), None)
        | (Some("--facet-counting"), None)
        | (Some("--goal-oriented"), Some("--facet--counting"))
        | (None, None)
        | (Some("--go"), Some("--fc")) => Some(Mode::GoalOriented(Weight::FacetCounting)),
        (Some("--goal-oriented"), Some("--absolute"))
        | (Some("--goal-oriented"), Some("--abs"))
        | (Some("--go"), Some("--absolute"))
        | (Some("--go"), Some("--abs"))
        | (Some("--abs"), None) => Some(Mode::GoalOriented(Weight::Absolute)),
        (Some("--strictly-goal-oriented"), Some("--absolute"))
        | (Some("--strictly-goal-oriented"), Some("--abs"))
        | (Some("--sgo"), Some("--absolute"))
        | (Some("--sgo"), Some("--abs")) => Some(Mode::StrictlyGoalOriented(Weight::Absolute)),
        (Some("--strictly-goal-oriented"), Some("--facet-counting"))
        | (Some("--strictly-goal-oriented"), Some("--fc"))
        | (Some("--sgo"), Some("--facet-counting"))
        | (Some("--sgo"), Some("--fc"))
        | (Some("--sgo"), None) => Some(Mode::StrictlyGoalOriented(Weight::FacetCounting)),
        (Some("--explore"), Some("--absolute"))
        | (Some("--explore"), Some("--abs"))
        | (Some("--expl"), Some("--absolute"))
        | (Some("--expl"), Some("--abs")) => Some(Mode::Explore(Weight::Absolute)),
        (Some("--explore"), Some("--facet-counting"))
        | (Some("--explore"), Some("--fc"))
        | (Some("--expl"), Some("--facet-counting"))
        | (Some("--expl"), Some("--fc"))
        | (Some("--expl"), None) => Some(Mode::Explore(Weight::FacetCounting)),
        (Some("--!"), Some(s)) => Some(Mode::Io(
            s[2..].parse::<u8>().expect("error: invalid command."),
        )),
        (Some("--go"), Some("--U")) => Some(Mode::GoalOriented(Weight::Information)),
        (Some("--sgo"), Some("--U")) => Some(Mode::StrictlyGoalOriented(Weight::Information)),
        (Some("--expl"), Some("--U")) => Some(Mode::Explore(Weight::Information)),
        _ => None,
    }
}

pub fn parse_args(args: impl Iterator<Item = String>) -> Option<(Mode, usize)> {
    let (n_p, xs_p): (Vec<String>, Vec<String>) = args.partition(|s| s[2..].starts_with('n'));
    let n = n_p
        .get(0)
        .and_then(|s| s[4..].parse::<usize>().ok())
        .unwrap_or(3);

    let mut xs = xs_p.iter().take(2);

    let t = (xs.next().map(|s| s.as_ref()), xs.next().map(|s| s.as_ref()));

    let mode = parse_mode(t)?;

    Some((mode, n))
}

pub fn manual() {
    println!();
    CONFIG.manual.iter().for_each(|s| println!("{}", s));
    println!();
}

pub fn source(navigator: &Navigator) {
    println!("\n{}\n", navigator.logic_program)
}

pub fn facets(navigator: &Navigator) {
    println!("{}", navigator.current_facets)
}

pub fn facets_count(navigator: &Navigator) {
    println!("\n{:?}\n", navigator.current_facets.len() * 2)
}

pub fn initial_facets(navigator: &Navigator) {
    println!("{}", navigator.initial_facets)
}

pub fn initial_facets_count(navigator: &Navigator) {
    println!("\n{:?}\n", navigator.initial_facets.len() * 2)
}

pub fn activate(mode: &Mode, navigator: &mut Navigator, input: Input) {
    let facets = input.map(|s| s.to_owned()).collect::<Vec<String>>();

    navigator.activate(&facets, mode);
}

pub fn activate_where(mode: &Mode, navigator: &mut Navigator, mut input: Input) {
    let facets = match input.next() {
        Some("*~") => {
            let p = input
                .next()
                .map(|n| n.replace('(', "").replace(')', ""))
                .expect("error: provide atom name.");
            let u = input
                .next()
                .and_then(|s| s.parse::<usize>().ok())
                .expect("error: provide upper bound.");
            let l = input
                .next()
                .and_then(|s| s.parse::<usize>().ok())
                .unwrap_or(0);
            (l..u + 1)
                .map(|i| format!("~{}({:?})", p, i))
                .collect::<Vec<_>>()
        }
        Some("r*~") => {
            let p = input.next().expect("error: provide atom name.");
            let u = input
                .next()
                .and_then(|s| s.parse::<usize>().ok())
                .expect("error: provide upper bound.");
            let l = input
                .next()
                .and_then(|s| s.parse::<usize>().ok())
                .unwrap_or(0);
            (l..u + 1)
                .map(|i| format!("~{}", p.replace('_', &i.to_string())))
                .collect::<Vec<_>>()
        }
        Some("*") => {
            let p = input
                .next()
                .map(|n| n.replace('(', "").replace(')', ""))
                .expect("error: provide atom name.");
            let u = input
                .next()
                .and_then(|s| s.parse::<usize>().ok())
                .expect("error: provide upper bound.");
            let l = input
                .next()
                .and_then(|s| s.parse::<usize>().ok())
                .unwrap_or(0);
            (l..u + 1)
                .map(|i| format!("{}({:?})", p, i))
                .collect::<Vec<_>>()
        }
        Some("r*") => {
            let p = input.next().expect("error: provide atom name.");
            let u = input
                .next()
                .and_then(|s| s.parse::<usize>().ok())
                .expect("error: provide upper bound.");
            let l = input
                .next()
                .and_then(|s| s.parse::<usize>().ok())
                .unwrap_or(0);
            (l..u + 1)
                .map(|i| p.replace('_', &i.to_string()))
                .collect::<Vec<_>>()
        }
        _ => todo!(),
    };

    navigator.activate(&facets, mode);
}
pub fn activate_all_of(mode: &Mode, navigator: &mut Navigator, mut input: Input) {
    let facets = {
        let p = input.next().expect("error: provide atom name.");
        navigator
            .inclusive_facets(&[])
            .iter()
            .map(|f| unsafe { f.to_string().unwrap_unchecked() })
            .filter(|s| s.starts_with(p))
            .collect::<Vec<_>>()
    };
    //dbg!(&facets);
    navigator.activate(&facets, mode);
}

pub fn deactivate(mode: &Mode, navigator: &mut Navigator, input: Input) {
    navigator.deactivate_any(
        &input
            .map(|s| s.to_owned().symbol())
            .collect::<Vec<Symbol>>(),
        mode,
    );
}

pub fn deactivate_where(mode: &Mode, navigator: &mut Navigator, mut input: Input) {
    let facets = match input.next() {
        Some("*~") => {
            let p = input
                .next()
                .map(|n| n.replace('(', "").replace(')', ""))
                .expect("error: provide atom name.");
            let u = input
                .next()
                .and_then(|s| s.parse::<usize>().ok())
                .expect("error: provide upper bound.");
            let l = input
                .next()
                .and_then(|s| s.parse::<usize>().ok())
                .unwrap_or(0);
            (l..u + 1)
                .map(|i| format!("~{}({:?})", p, i).symbol())
                .collect::<Vec<_>>()
        }
        Some("*") => {
            let p = input
                .next()
                .map(|n| n.replace('(', "").replace(')', ""))
                .expect("error: provide atom name.");
            let u = input
                .next()
                .and_then(|s| s.parse::<usize>().ok())
                .expect("error: provide upper bound.");
            let l = input
                .next()
                .and_then(|s| s.parse::<usize>().ok())
                .unwrap_or(0);
            (l..u + 1)
                .map(|i| format!("{}({:?})", p, i).symbol())
                .collect::<Vec<_>>()
        }
        _ => todo!(),
    };

    navigator.deactivate_any(&facets, mode);
}

pub fn clear_route(mode: &Mode, navigator: &mut Navigator) {
    match navigator.route.0.is_empty() {
        true => println!("\n[INFO] rcurrent route is already empty\n"),
        _ => {
            navigator.route = Route(vec![]);
            navigator.active_facets = vec![];
            navigator.update(mode);
        }
    }
}

pub fn navigate(navigator: &mut Navigator) {
    println!("\nsolving...");
    let start = Instant::now();

    navigator.navigate();

    let elapsed = start.elapsed();

    println!("call    : ?-navigate");
    println!("elapsed : {:?}\n", elapsed);
}

pub fn navigate_n(navigator: &mut Navigator, mut input: Input) {
    let n = input.next().and_then(|n| n.parse::<usize>().ok());

    println!("\nsolving...");
    let start = Instant::now();

    navigator.navigate_n(n);

    let elapsed = start.elapsed();

    println!("call    : ?-navigate-n {:?}", n.unwrap_or(navigator.n));
    println!("elapsed : {:?}\n", elapsed);
}

pub fn q_zoom(
    mode: &(impl GoalOrientedNavigation + Display),
    navigator: &mut Navigator,
    mut input: Input,
) {
    match input.next() {
        Some(f) => {
            println!("\nsolving...\n");
            let start = Instant::now();

            mode.show_z(navigator, f);

            let elapsed = start.elapsed();

            println!("\ncall    : ?-zoom {}", f);
            println!(
                "zoom    : {}",
                format!("{}", mode)
                    .split_whitespace()
                    .next()
                    .expect("could not retrieve weight parameter.")
            );
            println!("elapsed : {:?}\n", elapsed);
        }
        _ => {
            println!("\nsolving...\n");
            let start = Instant::now();

            mode.show_a_z(navigator);

            let elapsed = start.elapsed();

            println!("\ncall    : ?-zoom");
            println!(
                "zoom    : {}",
                format!("{}", mode)
                    .split_whitespace()
                    .next()
                    .expect("could not retrieve weight parameter.")
            );
            println!("elapsed : {:?}\n", elapsed);
        }
    };
}

pub fn q_zoom_n(
    mode: &(impl GoalOrientedNavigation + Display),
    navigator: &mut Navigator,
    mut input: Input,
) {
    match input.next() {
        Some(n) => {
            println!("\nsolving...\n");
            let start = Instant::now();

            navigator
                .current_facets
                .clone()
                .iter()
                .take(n.parse::<usize>().expect("parsing n failed."))
                .for_each(|f| mode.show_z(navigator, &f.repr()));

            let elapsed = start.elapsed();

            println!("\ncall    : ?-zoom-n {}", n);
            println!(
                "zoom    : {}",
                format!("{}", mode)
                    .split_whitespace()
                    .next()
                    .expect("could not retrieve weight parameter.")
            );
            println!("elapsed : {:?}\n", elapsed);
        }
        _ => q_zoom(mode, navigator, input), // all
    };
}

pub fn q_weight(
    mode: &(impl GoalOrientedNavigation + std::fmt::Display),
    navigator: &mut Navigator,
    mut input: Input,
) {
    match input.next() {
        Some(f) => {
            #[cfg(feature = "with_stats")]
            {
                println!("\nsolving...\n");
                let start = Instant::now();
            }

            mode.show_w(navigator, f);

            #[cfg(feature = "with_stats")]
            {
                let elapsed = start.elapsed();

                println!("\ncall    : ?-weight {}", f);
                println!(
                    "weight  : {}",
                    format!("{}", mode)
                        .split_whitespace()
                        .next()
                        .expect("could not retrieve weight parameter.")
                );
                println!("elapsed : {:?}\n", elapsed);
            }
        }
        _ => {
            #[cfg(feature = "with_stats")]
            {
                println!("\nsolving...\n");
                let start = Instant::now();
            }

            mode.show_a_w(navigator);

            #[cfg(feature = "with_stats")]
            {
                let elapsed = start.elapsed();

                println!("\ncall    : ?-weight");
                println!(
                    "weight  : {}",
                    format!("{}", mode)
                        .split_whitespace()
                        .next()
                        .expect("could not retrieve weight parameter.")
                );
                println!("elapsed : {:?}\n", elapsed);
            }
        }
    };
}

pub fn q_weight_n(
    mode: &(impl GoalOrientedNavigation + std::fmt::Display),
    navigator: &mut Navigator,
    mut input: Input,
) {
    match input.next() {
        Some(n) => {
            #[cfg(feature = "with_stats")]
            {
                println!("\nsolving...\n");
                let start = Instant::now();
            }

            navigator
                .current_facets
                .clone()
                .iter()
                .take(n.parse::<usize>().expect("parsing n failed."))
                .for_each(|f| mode.show_w(navigator, &f.repr()));

            #[cfg(feature = "with_stats")]
            {
                let elapsed = start.elapsed();

                println!("\ncall    : ?-weight-n {}", n);
                println!(
                    "weight  : {}",
                    format!("{}", mode)
                        .split_whitespace()
                        .next()
                        .expect("could not retrieve weight parameter.")
                );
                println!("elapsed : {:?}\n", elapsed);
            }
        }
        _ => {
            #[cfg(feature = "with_stats")]
            {
                println!("\nsolving...\n");
                let start = Instant::now();
            }

            mode.show_a_w(navigator);

            #[cfg(feature = "with_stats")]
            {
                let elapsed = start.elapsed();

                println!("\ncall    : ?-weight");
                println!(
                    "weight  : {}",
                    format!("{}", mode)
                        .split_whitespace()
                        .next()
                        .expect("could not retrieve weight parameter.")
                );
                println!("elapsed : {:?}\n", elapsed);
            }
        }
    };
}

pub fn q_route_safe(navigator: &mut Navigator, mut input: Input) {
    match input.next() {
        Some(arg) => match arg.chars().next() {
            // some route
            Some('<') => {
                println!("\nsolving...\n");
                let start = Instant::now();

                let facets = input
                    .chain(vec![&*arg])
                    .map(|s| s.replace('<', "").replace('>', ""))
                    .filter(|s| !s.is_empty())
                    .collect::<Vec<String>>();

                let assumptions = navigator
                    .parse_input_to_literals(&facets)
                    .collect::<Vec<Literal>>();

                println!("{:?}", navigator.satisfiable(&assumptions));

                let elapsed = start.elapsed();

                println!("\ncall    : ?-route-safe {}", Route(facets));
                println!("elapsed : {:?}\n", elapsed);
            }
            // peeking on current route
            Some('+') => {
                println!("\nsolving...\n");
                let start = Instant::now();

                let route = navigator.route.peek_steps(input);
                let assumptions = navigator
                    .parse_input_to_literals(&route.0)
                    .collect::<Vec<Literal>>();

                println!("{:?}", navigator.satisfiable(&assumptions));

                let elapsed = start.elapsed();

                println!("\ncall    : ?-route-safe {}", route);
                println!("elapsed : {:?}\n", elapsed);
            }
            _ => println!("\ninvalid input: {:?}\n\nsee `?man ?rs` for syntax", arg),
        },
        // current route
        _ => {
            println!("\nsolving...\n");
            let start = Instant::now();

            let assumptions = navigator.active_facets.clone();

            println!("{:?}", navigator.satisfiable(&assumptions));

            let elapsed = start.elapsed();

            println!("\ncall    : ?-route-safe {}", navigator.route);
            println!("elapsed : {:?}\n", elapsed);
        }
    }
}

pub fn q_route_maximal_safe(navigator: &mut Navigator, mut input: Input) {
    match input.next() {
        Some(s) => match s.chars().next() {
            // some route
            Some('<') => {
                println!("\nsolving...\n");
                let start = Instant::now();

                let facets = input
                    .chain(vec![&*s])
                    .map(|s| s.replace('<', "").replace('>', ""))
                    .filter(|s| !s.is_empty())
                    .collect::<Vec<String>>();

                let assumptions = navigator
                    .parse_input_to_literals(&facets)
                    .collect::<Vec<Literal>>();

                match navigator.satisfiable(&assumptions) {
                    false => println!("{:?}", false),
                    _ => {
                        let facets = navigator.inclusive_facets(&assumptions);
                        println!(
                            "{:?}",
                            facets.is_empty()
                                || facets.to_strings().all(|s| {
                                    !navigator.satisfiable(
                                        &navigator
                                            .parse_input_to_literals(
                                                &navigator.route.peek_step(&s).0,
                                            )
                                            .collect::<Vec<Literal>>(),
                                    )
                                })
                        )
                    }
                }

                let elapsed = start.elapsed();

                println!("\ncall    : ?-route-maximal-safe {}", Route(facets));
                println!("elapsed : {:?}\n", elapsed);
            }
            // peeking on current route
            Some('+') => {
                println!("\nsolving...\n");
                let start = Instant::now();

                let route = navigator.route.peek_steps(input);
                let assumptions = navigator
                    .parse_input_to_literals(&route.0)
                    .collect::<Vec<Literal>>();

                match navigator.satisfiable(&assumptions) {
                    false => println!("{:?}", false),
                    _ => {
                        let facets = navigator.inclusive_facets(&assumptions);
                        println!(
                            "{:?}",
                            facets.is_empty()
                                || facets.to_strings().all(|s| {
                                    !navigator.satisfiable(
                                        &navigator
                                            .parse_input_to_literals(
                                                &navigator.route.peek_step(&s).0,
                                            )
                                            .collect::<Vec<Literal>>(),
                                    )
                                })
                        )
                    }
                }

                let elapsed = start.elapsed();

                println!("\ncall    : ?-route-maximal-safe {}", route);
                println!("elapsed : {:?}\n", elapsed);
            }
            _ => println!("\ninvalid input: {:?}\n\nsee `?man ?rms` for syntax", s),
        },
        // current route
        _ => {
            println!("\nsolving...\n");
            let start = Instant::now();

            println!("{:?}", navigator.current_route_is_maximal_safe());

            let elapsed = start.elapsed();

            println!("\ncall    : ?-route-maximal-safe {}", navigator.route);
            println!("elapsed : {:?}\n", elapsed);
        }
    }
}

pub fn step(
    mode_: &Mode,
    mode: &impl GoalOrientedNavigation,
    navigator: &mut Navigator,
    current_facets: &[Symbol],
) {
    if navigator.current_facets.0.is_empty() {
        println!("[INFO] no current facets");
        return;
    }

    #[cfg(feature = "with_stats")]
    {
        println!("\ncall            : --step");
    }
    filter(mode, navigator, current_facets)
        .iter()
        .for_each(|s| print!("{} ", s));
    print!("\ntype facet to activate: ");

    activate(mode_, navigator, navigator.user_input().split_whitespace());

    navigate(navigator);
}

pub fn step_n(
    mode_: &Mode,
    mode: &impl GoalOrientedNavigation,
    navigator: &mut Navigator,
    current_facets: &[Symbol],
    input: Input,
) {
    if navigator.current_facets.0.is_empty() {
        println!("\n[INFO] nno current facets\n");
        return;
    }

    println!("\ncall            : --step-n");
    filter(mode, navigator, current_facets)
        .iter()
        .for_each(|s| print!("{} ", s));
    print!("\n\ntype facet to activate: ");

    activate(mode_, navigator, navigator.user_input().split_whitespace());

    navigate_n(navigator, input);
}

pub fn random_safe_steps(mode_: &Mode, nav: &mut Navigator, mut input: Input) {
    match input.next().map(|n| n.parse::<usize>()) {
        Some(Ok(n)) => {
            let t = (input.next(), input.next());

            let mut m = 0;

            if nav.current_facets.0.is_empty() {
                println!("[INFO] no current facets");
                return;
            }

            print!("solving...");
            match parse_mode(t) {
                Some(Mode::GoalOriented(_)) | None => {
                    #[cfg(feature = "with_stats")]
                    let start = Instant::now();

                    while !nav.current_route_is_maximal_safe() && m != n {
                        print!("{:?}.", m + 1);
                        let mut rng = rand::thread_rng();
                        nav.current_facets
                            .clone()
                            .0
                            .choose(&mut rng)
                            .map(|s| nav.activate(&[s.repr()], mode_))
                            .expect("random step failed.");
                        m += 1;
                    }
                    println!("done");

                    #[cfg(feature = "with_stats")]
                    {
                        let elapsed = start.elapsed();
                        println!("\ncall            : --random-safe-steps {:?}", n);
                        println!("navigation mode : goal-oriented");
                        println!("elapsed         : {:?}\n", elapsed);
                    }
                }
                Some(mode) => {
                    #[cfg(feature = "with_stats")]
                    let start = Instant::now();

                    while !nav.current_route_is_maximal_safe() && m != n {
                        print!("{:?}.", m + 1);
                        let mut rng = rand::thread_rng();
                        filter(&mode, nav, nav.current_facets.clone().as_ref())
                            .choose(&mut rng)
                            .map(|s| nav.activate(&[s.to_string()], mode_))
                            .expect("random step failed.");
                        m += 1;
                    }
                    println!("done");

                    #[cfg(feature = "with_stats")]
                    {
                        let elapsed = start.elapsed();
                        println!("call            : --random-safe-steps {:?}", n);
                        println!("navigation mode : {}", mode);
                        println!("elapsed         : {:?}\n", elapsed);
                    }
                }
            }
        }
        _ => random_safe_walk(mode_, nav, input),
    }
}

pub fn random_safe_walk(mode_: &Mode, nav: &mut Navigator, mut input: Input) {
    match parse_mode((input.next(), input.next())) {
        Some(Mode::GoalOriented(_)) | None => {
            if nav.current_facets.0.is_empty() {
                println!("\n[INFO] no current facets\n");
                return;
            }

            println!("\nsolving...\n");
            let start = Instant::now();

            let mut i = 0;
            while !nav.current_route_is_maximal_safe() {
                println!("step {:?}", i);
                let mut rng = rand::thread_rng();
                nav.current_facets
                    .clone()
                    .0
                    .choose(&mut rng)
                    .map(|s| nav.activate(&[s.repr()], mode_))
                    .expect("random step failed.");
                i += 1;
            }

            let elapsed = start.elapsed();

            println!("\ncall            : --random-safe-walk");
            println!("navigation mode : goal-oriented");
            println!("elapsed         : {:?}\n", elapsed);
        }
        Some(mode) => {
            if nav.current_route_is_maximal_safe() {
                println!("\n{} is maximal safe\n", nav.route);
                return;
            }

            println!("\nsolving...\n");
            let start = Instant::now();

            let mut i = 0;
            while !nav.current_route_is_maximal_safe() {
                println!("step {:?}", i);
                let mut rng = rand::thread_rng();
                filter(&mode, nav, nav.current_facets.clone().as_ref())
                    .choose(&mut rng)
                    .map(|s| nav.activate(&[s.to_string()], mode_))
                    .expect("random step failed.");
                i += 1;
            }

            let elapsed = start.elapsed();

            println!("call            : --random-safe-walk");
            println!("navigation mode : {}", mode);
            println!("elapsed         : {:?}\n", elapsed);
        }
    }
}

// FIX
pub fn q_zoom_higher_than(
    mode: &(impl GoalOrientedNavigation + Display),
    navigator: &mut Navigator,
    mut input: Input,
) {
    let bound = if let Some(f) = input.next() {
        f.parse::<f32>().expect("parsing bound failed.")
    } else {
        return println!("\n[ERROR] no bound provided\n");
    };

    println!("\nsolving...");
    let start = Instant::now();

    mode.find_with_zh(navigator, bound)
        .map(|f| println!("\n{}\n", f))
        .unwrap_or_else(|| println!("\nno result\n"));

    let elapsed = start.elapsed();

    println!("call            : ?-zoom-higher-than {:?}", bound);
    println!("navigation mode : {}", mode);
    println!("elapsed         : {:?}\n", elapsed);
}

// FIX
pub fn q_zoom_lower_than(
    mode: &(impl GoalOrientedNavigation + Display),
    navigator: &mut Navigator,
    mut input: Input,
) {
    let bound = if let Some(f) = input.next() {
        f.parse::<f32>().expect("parsing bound failed.")
    } else {
        return println!("\n[ERROR] no bound provide\n");
    };

    println!("\nsolving...");
    let start = Instant::now();

    mode.find_with_zl(navigator, bound)
        .map(|f| println!("\n{}\n", f))
        .unwrap_or_else(|| println!("\nno result\n"));

    let elapsed = start.elapsed();

    println!("call            : ?-zoom-lower-than {:?}", bound);
    println!("navigation mode : {}", mode);
    println!("elapsed         : {:?}\n", elapsed);
}

pub fn find_facet_with_zoom_higher_than_and_activate(
    mode: &(impl GoalOrientedNavigation + Display),
    mode_: &Mode,
    navigator: &mut Navigator,
    mut input: Input,
) {
    let bound = if let Some(f) = input.next() {
        f.parse::<f32>().expect("parsing bound failed.")
    } else {
        return println!("\n[ERROR] no bound provided\n");
    };

    println!("\nsolving...");
    let start = Instant::now();

    mode.find_with_zh(navigator, bound)
        .map(|f| navigator.activate(&[f], mode_))
        .unwrap_or_else(|| println!("\nno result"));

    let elapsed = start.elapsed();

    println!(
        "\ncall            : --find-facet-with-zoom-higher-than-and-activate {:?}",
        bound
    );
    println!("navigation mode : {}", mode);
    println!("elapsed         : {:?}\n", elapsed);
}

pub fn find_facet_with_zoom_lower_than_and_activate(
    mode: &(impl GoalOrientedNavigation + Display),
    mode_: &Mode,
    navigator: &mut Navigator,
    mut input: Input,
) {
    let bound = if let Some(f) = input.next() {
        f.parse::<f32>().expect("parsing bound failed.")
    } else {
        return println!("\n[ERROR] no bound provided\n");
    };

    println!("\nsolving...");
    let start = Instant::now();

    mode.find_with_zl(navigator, bound)
        .map(|f| navigator.activate(&[f], mode_))
        .unwrap_or_else(|| println!("\nno result"));

    let elapsed = start.elapsed();

    println!(
        "\ncall            : --find-facet-with-zoom-lower-than-and-activate {:?}",
        bound
    );
    println!("navigation mode : {}", mode);
    println!("elapsed         : {:?}\n", elapsed);
}

pub fn k_greedy_search(navigator: &mut Navigator, mut input: Input) {
    let fst = input.next();
    let sample_size = fst.and_then(|n| n.parse::<usize>().ok());

    let mut ignored_atoms = input
        .map(|s| crate::translator::Atom(s).parse(&[]))
        .flatten() // NOTE: tricky
        .collect::<Vec<_>>();
    if !sample_size.is_some() && fst.is_some() {
        let s = unsafe { fst.unwrap_unchecked() };
        if let Some(sym) = crate::translator::Atom(&s).parse(&[]) {
            ignored_atoms.push(sym)
        }
    }

    println!("\nsolving...\n");
    let start = Instant::now();
    navigator.k_greedy_search_show(ignored_atoms.into_iter(), sample_size);
    let elapsed = start.elapsed();

    println!("\ncall            : --k-greedy-search");
    println!("elapsed         : {:?}\n", elapsed);
}

//pub fn k_greedy_search_io(navigator: &mut Navigator) {
//    navigator.k_greedy_search_show(None);
//}

pub fn components(navigator: &mut Navigator) {
    println!("\nsolving...\n");
    let start = Instant::now();

    navigator.components().0.iter().for_each(|(k, v)| {
        let (kl, vl0, vl1) = (k.len(), v.0.len(), v.1.len());
        print!("({:?}) com: ", vl0);
        v.0.iter().for_each(|s| print!("{} ", s));
        print!("\n({:?}) cov: ", kl);
        k.iter()
            .for_each(|s| print!("{} ", unsafe { s.to_string().unwrap_unchecked() }));
        print!("\n({:?}) con: ", vl1);
        v.1.iter()
            .for_each(|s| print!("{} ", unsafe { s.to_string().unwrap_unchecked() }));
        println!("\n-");
    });

    let elapsed = start.elapsed();

    println!("\ncall            : --connected-components",);
    println!("elapsed         : {:?}\n", elapsed);
}

pub fn components_io(navigator: &mut Navigator) {
    let mut i = 0;
    navigator.components().0.iter().for_each(|(k, v)| {
        let (kl, vl0, vl1) = (k.len(), v.0.len(), v.1.len());
        print!("({:?}) com: ", vl0);
        v.0.iter().for_each(|s| print!("{} ", s));
        print!("\n({:?}) cov: ", kl);
        k.iter()
            .for_each(|s| print!("{} ", unsafe { s.to_string().unwrap_unchecked() }));
        print!("\n({:?}) con: ", vl1);
        v.1.iter()
            .for_each(|s| print!("{} ", unsafe { s.to_string().unwrap_unchecked() }));
        println!("\n-");
        i += 1;
    });
    println!("{:?}", i)
}

/*
pub fn cores_in_io(navigator: &mut Navigator) {
    navigator.cores_in();
}

pub fn find_perfect_core(navigator: &mut Navigator) {
    println!("{:?}", navigator.find_perfect_core())
}

pub fn find_cores_encoding(navigator: &mut Navigator) {
    navigator.show_find_cores_encoding()
}
*/

pub fn related_components(navigator: &mut Navigator) {
    println!("\nsolving...\n");
    let start = Instant::now();

    navigator.related_components().0.iter().for_each(|(k, v)| {
        let (kl, vl0, vl1) = (k.len(), v.0.len(), v.1.len());
        print!("({:?}) com: ", vl0);
        v.0.iter().for_each(|s| print!("{} ", s));
        print!("\n({:?}) con: ", kl);
        k.iter()
            .for_each(|s| print!("{} ", unsafe { s.to_string().unwrap_unchecked() }));
        print!("\n({:?}) cov: ", vl1);
        v.1.iter()
            .for_each(|s| print!("{} ", unsafe { s.to_string().unwrap_unchecked() }));
        println!("\n-");
    });

    let elapsed = start.elapsed();

    println!("\ncall            : --related-components",);
    println!("elapsed         : {:?}\n", elapsed);
}

pub fn activate_from_file(mode: &Mode, navigator: &mut Navigator, file_path: &str) {
    let facets = std::fs::read_to_string(file_path)
        .unwrap() // TODO
        .lines()
        .map(|s| s.to_owned())
        .collect::<Vec<_>>();

    navigator.activate(&facets, mode);
}

pub fn cc(navigator: &mut Navigator, input: Input) {
    if let Some(cc) = navigator.consequences(
        crate::navigator::EnumMode::Cautious,
        &navigator
            .parse_input_to_literals(&input.collect::<Vec<_>>())
            .collect::<Vec<_>>(),
    ) {
        println!("{:?}", crate::soe::stringify(&cc));
    }
}

pub fn hole(navigator: &mut Navigator, input: Input) {
    let facets = input.map(|s| s.to_owned()).collect::<Vec<_>>();
    if let Some(cc) = navigator.consequences(
        crate::navigator::EnumMode::Cautious,
        &navigator
            .parse_input_to_literals(&facets)
            .collect::<Vec<_>>(),
    ) {
        println!(
            "{:?}",
            crate::soe::stringify(&cc)
                .iter()
                .filter(|s| !facets.contains(&s))
                .collect::<Vec<_>>()
        );
    }
}

pub fn naive_approach_representative_sample(navigator: &mut Navigator, input: Input) {
    let ignored_atoms = input
        .map(|s| crate::translator::Atom(s).parse(&[]))
        .flatten() // NOTE: tricky
        .collect::<Vec<_>>();

    println!("\nsolving...\n");
    let start = Instant::now();
    navigator.naive_approach_representative_search_show(ignored_atoms.into_iter());
    let elapsed = start.elapsed();

    println!("\ncall            : --naive-repr-search");
    println!("elapsed         : {:?}\n", elapsed);
}

pub fn perfect_sample(navigator: &mut Navigator, mut input: Input) {
    let mut h = "";
    let mut heuristic = match input.next() {
        Some("ediv") => {
            h = "ediv";
            crate::soe::Heuristic::Ediv
        }
        Some("erep") => {
            h = "erep";
            crate::soe::Heuristic::Erep
        }
        _ => unimplemented!(),
    };

    let ignored_atoms = input
        .map(|s| crate::translator::Atom(s).parse(&[]))
        .flatten() // NOTE: tricky
        .collect::<Vec<_>>();

    println!("\nsolving...\n");
    let start = Instant::now();
    heuristic.collect_show(
        navigator,
        &[],
        &ignored_atoms,
        std::collections::HashSet::new(),
        &navigator.current_facets.0.clone(),
    );
    let elapsed = start.elapsed();

    println!("\ncall            : --{}", h);
    println!("elapsed         : {:?}\n", elapsed);
}

pub fn uncertainty_true(navigator: &mut Navigator, mut input: Input) {
    let (of, target) = (
        &input
            .next()
            .map(|s| vec![s.to_owned()])
            .unwrap_or_else(|| vec![]),
        &input.map(|s| s.to_owned()).collect::<Vec<_>>(),
    );
    println!("{:.2}", navigator.uncertainty_true(of, target))
}

pub fn uncertainty_false(navigator: &mut Navigator, mut input: Input) {
    let (of, target) = (
        &input
            .next()
            .map(|s| vec![s.to_owned()])
            .unwrap_or_else(|| vec![]),
        &input.map(|s| s.to_owned()).collect::<Vec<_>>(),
    );
    println!("{:.2}", navigator.uncertainty_false(of, target))
}

pub fn gini(navigator: &mut Navigator, mut input: Input) {
    let (of, target) = (
        &input
            .next()
            .map(|s| vec![s.to_owned()])
            .unwrap_or_else(|| vec![]),
        &input.map(|s| s.to_owned()).collect::<Vec<_>>(),
    );
    match target.is_empty() {
        true => {
            for a in navigator.current_facets.clone().iter() {
                let s = &a.repr();
                println!("{:.2} {}", navigator.gini(of, Some(&[s.to_owned()])), s)
            }
        }
        _ => println!("{:.2}", navigator.gini(of, Some(target))),
    }
}

pub fn seperates_best(navigator: &mut Navigator, mut input: Input) {
    let of = &input
        .next()
        .map(|s| vec![s.to_owned()])
        .unwrap_or_else(|| vec![]);

    let mut hm = HashMap::new();
    navigator.current_facets.clone().iter().for_each(|a| {
        if a.repr() != of[0] {
            hm.insert(
                a.repr(),
                (100f64 * navigator.gini(of, Some(&[a.repr()]))) as usize,
            );
        }
    });
    let max = unsafe { hm.values().min().unwrap_unchecked() };
    hm.iter().for_each(|(k, v)| {
        if v == max {
            println!("{:.2} {}", *v as f64 / 100f64, k);
        }
    })
}

pub fn seperates_worst(navigator: &mut Navigator, mut input: Input) {
    let of = &input
        .next()
        .map(|s| vec![s.to_owned()])
        .unwrap_or_else(|| vec![]);

    let mut hm = HashMap::new();
    navigator.current_facets.clone().iter().for_each(|a| {
        hm.insert(
            a.repr(),
            (100f64 * navigator.gini(of, Some(&[a.repr()]))) as usize,
        );
    });
    let min = unsafe { hm.values().max().unwrap_unchecked() };
    hm.iter().for_each(|(k, v)| {
        if v == min {
            println!("{:.2} {}", *v as f64 / 100f64, k);
        }
    })
}
