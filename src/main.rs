#![deny(clippy::all)]

mod asnc;
mod cache;
mod commands;
mod config;
mod navigator;
mod soe;
mod translator;
mod utils;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::fs::read_to_string;
use std::path::Path;
use std::time::Instant;

use commands::*;
use config::{manual_command_or_query, CONFIG};
use navigator::*;

type Result<T> = std::result::Result<T, NavigatorError>;

#[cfg(not(tarpaulin_include))]
fn clingo_version_str() -> String {
    let (major, minor, revision) = clingo::version();
    format!("{:?}.{:?}.{:?}", major, minor, revision)
}

#[cfg(not(tarpaulin_include))]
fn main() -> Result<()> {
    let mut args = std::env::args();
    let arg = match args.nth(1) {
        Some(s) => s,
        _ => {
            println!("\nNo program path provided. Use --help or -h for help.\n");
            return Ok(());
        }
    };

    if ["--help", "--h"].iter().any(|s| *s == arg) {
        println!(
            "\n{} version {} [clingo version {}]\n",
            CONFIG.name,
            CONFIG.version,
            clingo_version_str()
        );

        CONFIG.help.iter().for_each(|s| println!("{}", s));

        println!();

        return Ok(());
    }

    let (mut mode, n) = parse_args(args).ok_or(NavigatorError::None)?;

    let path = Path::new(&arg).to_str().ok_or(NavigatorError::None)?;

    let start = Instant::now();
    let mut navigator = read_to_string(path).map(|s| Navigator::new(s, n))??;
    let end = start.elapsed();

    let io = match mode {
        Mode::Io(2) => {
            k_greedy_search_io(&mut navigator);
            Some(())
        }
        Mode::Io(3) => {
            cores_in_io(&mut navigator);
            Some(())
        }
        Mode::Io(4) => {
            components_io(&mut navigator);
            Some(())
        }
        _ => None,
    };
    if io.is_some() {
        return Ok(());
    }

    println!(
        "\n{} version {} [clingo version {}]",
        CONFIG.name,
        CONFIG.version,
        clingo_version_str()
    );

    println!("\nelapsed : {:?}", end);

    println!("reading from {}\n", arg);
    println!("type `?man` for help\n\n");

    let mut quit = false;

    while !quit {
        navigator.info();

        let input = navigator.user_input();

        if input.is_empty() {
            continue;
        }

        let mut input_iter = input.split_whitespace();
        let command = input_iter.next().expect("unknown error.");

        match command {
            "?-manual" | "?man" => match input_iter.next() {
                Some(s) => manual_command_or_query(s),
                _ => manual(),
            },
            "?-source" | "?src" => source(&navigator),
            "?-facets" | "?fs" => facets(&navigator),
            "?-facets-count" | "?fc" => facets_count(&navigator),
            "?-initial-facets" | "?ifs" => initial_facets(&navigator),
            "?-initial-facets-count" | "?ifc" => initial_facets_count(&navigator),
            "--activate" | ":a" => activate(&mode, &mut navigator, input_iter),
            "--deactivate" | ":d" => deactivate(&mode, &mut navigator, input_iter),
            "--clear-route" | ":cr" => clear_route(&mode, &mut navigator),
            "--random-safe-steps" | ":rss" => random_safe_steps(&mode, &mut navigator, input_iter),
            "--random-safe-walk" | ":rsw" => random_safe_walk(&mode, &mut navigator, input_iter),
            "--step" | ":s" => {
                let fs = navigator.clone().current_facets;

                match (input_iter.next(), input_iter.next()) {
                    (None, None) => step(&mode, &mode, &mut navigator, fs.as_ref()),
                    t => match parse_mode(t) {
                        Some(m) => step(&mode, &m, &mut navigator, fs.as_ref()),
                        _ => step(&mode, &mode, &mut navigator, fs.as_ref()),
                    },
                }
            }
            "--step-n" | ":sn" => {
                let fs = navigator.current_facets.clone();
                step_n(&mode, &mode, &mut navigator, fs.as_ref(), input_iter);
            }
            "?-navigate" | "?n" => navigate(&mut navigator),
            "?-navigate-n" | "?nn" => navigate_n(&mut navigator, input_iter),
            "--find-facet-with-zoom-higher-than-and-activate" | ":zha" => {
                find_facet_with_zoom_higher_than_and_activate(
                    &mode,
                    &mode,
                    &mut navigator,
                    input_iter,
                )
            }
            "--find-facet-with-zoom-lower-than-and-activate" | ":zla" => {
                find_facet_with_zoom_lower_than_and_activate(
                    &mode,
                    &mode,
                    &mut navigator,
                    input_iter,
                )
            }
            "--switch-mode" | ":sm" => match parse_mode((input_iter.next(), input_iter.next())) {
                Some(m) => {
                    mode = m;
                    navigator.update(&mode)
                }
                _ => println!("\n[ERROR] unknown mode.\n"),
            },
            "?-weight" | "?w" => q_weight(&mode, &mut navigator, input_iter),
            "?-weight-n" | "?wn" => q_weight_n(&mode, &mut navigator, input_iter),
            "?-zoom" | "?z" => q_zoom(&mode, &mut navigator, input_iter),
            "?-zoom-n" | "?zn" => q_zoom_n(&mode, &mut navigator, input_iter),
            "?-route-safe" | "?rs" => q_route_safe(&mut navigator, input_iter),
            "?-route-maximal-safe" | "?rms" => q_route_maximal_safe(&mut navigator, input_iter),
            "?-zoom-higher-than" | "?zh" => q_zoom_higher_than(&mode, &mut navigator, input_iter),
            "?-zoom-lower-than" | "?zl" => q_zoom_lower_than(&mode, &mut navigator, input_iter),
            "?-mode" | "?m" => println!("\n{}\n", mode),
            "?com" => components(&mut navigator),
            "?cor" => cores_in_io(&mut navigator), // TODO
            "?rcom" => related_components(&mut navigator),
            ":aw" => activate_where(&mode, &mut navigator, input_iter),
            ":aa" => activate_all_of(&mode, &mut navigator, input_iter),
            ":dw" => deactivate_where(&mode, &mut navigator, input_iter),
            ":kg" => k_greedy_search(&mut navigator, input_iter),
            "--quit" | ":q" => quit = true,
            _ => println!(
                "\nunknown command or query: {:?}\nuse `?man` to inspect manual\n",
                input
            ),
        }
    }

    Ok(())
}
