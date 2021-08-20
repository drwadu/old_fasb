use clingo::{Literal, Symbol};
use rand::seq::SliceRandom;

use crate::config::CONFIG;
use crate::navigator::{filter, zoom, GoalOrientedNavigation, Mode, Navigator, Weight};
use crate::utils::{Repr, Route, ToSymbol};

pub type Input<'a> = std::str::SplitWhitespace<'a>;

pub fn parse_mode(input: (Option<&str>, Option<&str>)) -> Option<Mode> {
    let mode = match input {
        (Some("--goal-oriented"), None)
        | (Some("--go"), None)
        | (None, Some("--goal-oriented"))
        | (None, Some("--go"))
        | (Some("--fc"), None)
        | (Some("--facet-counting"), None)
        | (None, Some("--facet-counting"))
        | (None, Some("--fc"))
        | (Some("--goal-oriented"), Some("--facet--counting"))
        | (None, None)
        | (Some("--go"), Some("--fc")) => Mode::GoalOriented(Weight::FacetCounting),
        (Some("--goal-oriented"), Some("--absolute"))
        | (Some("--goal-oriented"), Some("--abs"))
        | (Some("--go"), Some("--absolute"))
        | (Some("--go"), Some("--abs")) => Mode::GoalOriented(Weight::Absolute),
        (Some("--strictly-goal-oriented"), Some("--absolute"))
        | (Some("--strictly-goal-oriented"), Some("--abs"))
        | (Some("--sgo"), Some("--absolute"))
        | (Some("--sgo"), Some("--abs")) => Mode::StrictlyGoalOriented(Weight::Absolute),
        (Some("--strictly-goal-oriented"), Some("--facet-counting"))
        | (Some("--strictly-goal-oriented"), Some("--fc"))
        | (Some("--sgo"), Some("--facet-counting"))
        | (Some("--sgo"), Some("--fc")) => Mode::StrictlyGoalOriented(Weight::FacetCounting),
        (Some("--explore"), Some("--absolute"))
        | (Some("--explore"), Some("--abs"))
        | (Some("--expl"), Some("--absolute"))
        | (Some("--expl"), Some("--abs")) => Mode::Explore(Weight::Absolute),
        (Some("--explore"), Some("--facet-counting"))
        | (Some("--explore"), Some("--fc"))
        | (Some("--expl"), Some("--facet-counting"))
        | (Some("--expl"), Some("--fc")) => Mode::Explore(Weight::FacetCounting),
        _ => panic!("unknown navigation mode."),
    };

    Some(mode)
}

pub fn parse_args(args: std::env::Args) -> Option<(Mode, usize)> {
    let (n_p, xs_p): (Vec<String>, Vec<String>) = args.partition(|s| s[2..].starts_with('n'));
    let n = n_p
        .get(0)
        .map(|s| s[4..].parse::<usize>().ok())
        .flatten()
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

pub fn activate(navigator: &mut Navigator, input: Input) {
    navigator.activate(&input.map(|s| s.to_owned()).collect::<Vec<String>>());
}

pub fn deactivate(navigator: &mut Navigator, input: Input) {
    navigator.deactivate_any(
        &input
            .map(|s| s.to_owned().symbol())
            .collect::<Vec<Symbol>>(),
    );
}

pub fn clear_route(navigator: &mut Navigator) {
    navigator.route = Route(vec![]);
    navigator.active_facets = vec![];
    navigator.update();
}

pub fn navigate(navigator: &mut Navigator) {
    navigator.navigate()
}

pub fn navigate_n(navigator: &mut Navigator, mut input: Input) {
    let n = input.next().map(|n| n.parse::<usize>().ok()).flatten();

    navigator.navigate_n(n);
}

pub fn q_zoom(mode: &impl GoalOrientedNavigation, navigator: &mut Navigator, mut input: Input) {
    println!("\nsolving...\n");

    match input.next() {
        Some(f) => {
            println!("{}: {:?}", f, mode.zoom(navigator, f));
        }
        _ => {
            navigator.current_facets.clone().iter().for_each(|f| {
                let repr = f.repr();
                let neg_repr = format!("~{}", repr.clone());

                println!("{} : {:.2}%", &repr, zoom(mode, navigator, &repr) * 100f32);
                println!(
                    "{} : {:.2}%",
                    &neg_repr,
                    zoom(mode, navigator, &neg_repr) * 100f32
                );
            });
        }
    };

    println!();
}

pub fn q_route_safe(navigator: &mut Navigator, mut input: Input) {
    match input.next() {
        Some(arg) => match arg.chars().next() {
            // some route
            Some('<') => {
                println!("\nsolving...\n");

                let facets = input
                    .chain(vec![&*arg])
                    .map(|s| s.replace("<", "").replace(">", ""))
                    .filter(|s| !s.is_empty())
                    .collect::<Vec<String>>();

                let assumptions = navigator
                    .parse_input_to_literals(&facets)
                    .collect::<Vec<Literal>>();

                println!("{:?}", navigator.satisfiable(&assumptions))
            }
            // peeking on current route
            Some('+') => {
                println!("\nsolving...\n");

                let route = navigator.route.peek_steps(input);
                let assumptions = navigator
                    .parse_input_to_literals(&route.0)
                    .collect::<Vec<Literal>>();

                println!("{:?}", navigator.satisfiable(&assumptions))
            }
            _ => println!(
                "\ninvalid input: {:?}\n\nsee manual (--manual, :man) for syntax",
                arg
            ),
        },
        // current route
        _ => {
            println!("\nsolving...\n");

            let assumptions = navigator.active_facets.clone();

            println!("{:?}", navigator.satisfiable(&assumptions))
        }
    }

    println!();
}

pub fn q_route_maximal_safe(navigator: &mut Navigator, mut input: Input) {
    println!("\nsolving...\n");

    match input.next() {
        Some(s) => match s.chars().next() {
            // some route
            Some('<') => {
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
            }
            // peeking on current route
            Some('+') => {
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
            }
            _ => println!("invalid input: {:?}", s),
        },
        // current route
        _ => println!("{:?}", navigator.current_route_is_maximal_safe()),
    }

    println!();
}

pub fn step(
    mode: &impl GoalOrientedNavigation,
    navigator: &mut Navigator,
    current_facets: &[Symbol],
) {
    println!("\nsolving...\n");

    filter(mode, navigator, current_facets)
        .iter()
        .for_each(|s| print!("{} ", s));
    print!("\n\nactivate: ");

    activate(navigator, navigator.user_input().split_whitespace());

    navigate(navigator);
}

pub fn step_n(
    mode: &impl GoalOrientedNavigation,
    navigator: &mut Navigator,
    current_facets: &[Symbol],
    input: Input,
) {
    println!("\nsolving...\n");

    filter(mode, navigator, current_facets)
        .iter()
        .for_each(|s| print!("{} ", s));
    print!("\n\nactivate: ");

    activate(navigator, navigator.user_input().split_whitespace());

    navigate_n(navigator, input);
}

pub fn random_safe_steps(nav: &mut Navigator, mut input: Input) {
    match input
        .next()
        .map(|n| n.parse::<usize>())
        .expect("\n\nprovide number of steps to take\n")
    {
        Ok(n) => {
            let t = (input.next(), input.next());

            let mut m = 0;
            match parse_mode(t) {
                Some(Mode::GoalOriented(_)) | None => {
                    println!("\nsolving...\n");

                    while !nav.current_route_is_maximal_safe() && m != n {
                        let mut rng = rand::thread_rng();
                        nav.current_facets
                            .clone()
                            .0
                            .choose(&mut rng)
                            .map(|s| nav.activate(&[s.repr()]))
                            .expect("random step failed.");
                        m += 1;
                    }
                }
                Some(mode) => {
                    println!("\nsolving...\n");

                    while !nav.current_route_is_maximal_safe() && m != n {
                        let mut rng = rand::thread_rng();
                        filter(&mode, nav, nav.current_facets.clone().as_ref())
                            .choose(&mut rng)
                            .map(|s| nav.activate(&[s.to_string()]))
                            .expect("random step failed.");
                        m += 1;
                    }
                }
            }
        }
        _ => random_safe_steps(nav, input), // :rsw --go --fc
    }
}

pub fn random_safe_walk(nav: &mut Navigator, mut input: Input) {
    match parse_mode((input.next(), input.next())) {
        Some(Mode::GoalOriented(_)) | None => {
            println!("\nsolving...\n");

            while !nav.current_route_is_maximal_safe() {
                let mut rng = rand::thread_rng();
                nav.current_facets
                    .clone()
                    .0
                    .choose(&mut rng)
                    .map(|s| nav.activate(&[s.repr()]))
                    .expect("random step failed.");
            }
        }
        Some(mode) => {
            println!("\nsolving...\n");

            while !nav.current_route_is_maximal_safe() {
                let mut rng = rand::thread_rng();
                filter(&mode, nav, nav.current_facets.clone().as_ref())
                    .choose(&mut rng)
                    .map(|s| nav.activate(&[s.to_string()]))
                    .expect("random step failed.");
            }
        }
    }
}

pub fn q_weight(mode: &impl GoalOrientedNavigation, navigator: &mut Navigator, mut input: Input) {
    println!("\nsolving...\n");

    match input.next() {
        Some(f) => {
            mode.show(navigator, f);
        }
        _ => {
            let start = std::time::Instant::now();
            navigator.current_facets.clone().iter().for_each(|f| {
                let repr = f.repr();
                let neg_repr = format!("~{}", repr.clone());

                mode.show(navigator, &repr);
                mode.show(navigator, &neg_repr);
            });

            println!("elapsed: {:?}", start.elapsed());
        }
    };

    println!();
}

pub fn q_zoom_higher_than(
    mode: &impl GoalOrientedNavigation,
    navigator: &mut Navigator,
    mut input: Input,
) {
    let bound = if let Some(f) = input.next() {
        f.parse::<f32>().expect("parsing bound failed.")
    } else {
        return println!("\nno bound provided\n");
    };

    println!("\nsolving...");

    let fs = navigator.current_facets.clone();

    match fs
        .iter()
        .find(|f| zoom(mode, navigator, &f.repr()) >= bound)
    {
        Some(f) => println!("\n{}\n", f.repr()),
        _ => fs
            .iter()
            .map(|f| format!("~{}", f.repr()))
            .find(|f| zoom(mode, navigator, f) >= bound)
            .map(|f| println!("\n{}\n", f))
            .unwrap_or_else(|| println!("\nno result\n")),
    }
}

pub fn q_zoom_lower_than(
    mode: &impl GoalOrientedNavigation,
    navigator: &mut Navigator,
    mut input: Input,
) {
    let bound = if let Some(f) = input.next() {
        f.parse::<f32>().expect("parsing bound failed.")
    } else {
        return println!("\nno bound provided\n");
    };

    println!("\nsolving...");

    let fs = navigator.current_facets.clone();

    match fs
        .iter()
        .map(|f| format!("~{}", f.repr()))
        .find(|f| zoom(mode, navigator, f) <= bound)
    {
        Some(f) => println!("\n{}\n", f),
        _ => fs
            .iter()
            .find(|f| zoom(mode, navigator, &f.repr()) <= bound)
            .map(|f| println!("\n{}\n", f.repr()))
            .unwrap_or_else(|| println!("\nno result")),
    }
}

pub fn find_facet_with_zoom_higher_than_and_activate(
    mode: &impl GoalOrientedNavigation,
    navigator: &mut Navigator,
    mut input: Input,
) {
    let bound = if let Some(f) = input.next() {
        f.parse::<f32>().expect("parsing bound failed.")
    } else {
        return println!("\nno bound provided\n");
    };

    println!("\nsolving...");

    let fs = navigator.current_facets.clone();

    match fs
        .iter()
        .find(|f| zoom(mode, navigator, &f.repr()) >= bound)
    {
        Some(f) => {
            let repr = f.repr();

            navigator.activate(&[repr.clone()]);
        }
        _ => fs
            .iter()
            .map(|f| format!("~{}", f.repr()))
            .find(|f| zoom(mode, navigator, f) >= bound)
            .map(|f| {
                navigator.activate(&[f.clone()]);
            })
            .unwrap_or_else(|| println!("\nno result")),
    }

    println!();
}

pub fn find_facet_with_zoom_lower_than_and_activate(
    mode: &impl GoalOrientedNavigation,
    navigator: &mut Navigator,
    mut input: Input,
) {
    let bound = if let Some(f) = input.next() {
        f.parse::<f32>().expect("parsing bound failed.")
    } else {
        return println!("\nno bound provided\n");
    };

    println!("\nsolving...");

    let fs = navigator.current_facets.clone();

    match fs
        .iter()
        .map(|f| format!("~{}", f.repr()))
        .find(|f| zoom(mode, navigator, f) <= bound)
    {
        Some(f) => {
            navigator.activate(&[f]);
        }
        _ => fs
            .iter()
            .find(|f| zoom(mode, navigator, &f.repr()) <= bound)
            .map(|f| {
                navigator.activate(&[f.repr()]);
            })
            .unwrap_or_else(|| println!("\nno result")),
    }

    println!();
}
