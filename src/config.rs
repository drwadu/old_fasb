pub struct Config<'a> {
    pub name: &'a str,
    pub authors: &'a str,
    pub version: &'a str,
    pub help: [&'a str; 7],
    pub manual: [&'a str; 33],
}

const NAME: &str = env!("CARGO_PKG_NAME");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP: [&str; 7] = [
    "usage             : fasb path [mode] [weight] [n]",
    "default           : fasb path --goal-oriented --facet-counting --n=3\n",
    "[REQUIRED] path   : path to the .lp file to read",
    "[OPTIONAL] mode   : [--goal-oriented | --go] | [--strictly-goal-oriented | --sgo] | [--explore | --expl]",
    "[OPTIONAL] weight : [--absolute | --abs] | [--facet-counting | --fc]",
    "[OPTIONAL] n      : u64",
    "\nuse `?man` to inspect manual during navigation",
];

const MANUAL: [&str; 33] = [
    "fasb supports the following combinations of weights and modes:\n",
    "\t* absolute goal-oriented (--go --abs)",
    "\t* absolute strictly-goal-oriented (--sgo --abs)",
    "\t* absolute explore (--expl --abs)",
    "\t* facet-counting goal-oriented (--go --fc)",
    "\t* facet-counting strictly-goal-oriented (--sgo --fc)",
    "\t* facet-counting explore (--expl --fc)\n\n",
    "commands:\n:a        activates n provided whitespace separated facets",
    ":d        deactivates n provided whitespace separated facets; if a facet is activated multiple times, any occurence will be deactivated",
    ":cr       clears the current route, i.e., sets empty route as current route",
    ":zha      activates first facet found with zoom in effect higher than or equal to the provided bound",
    ":zla      activates first facet found with zoom in effect lower than or equal to the provided bound",
    ":rss      actitvates n random facets w.r.t. the specified combination of mode and weight",
    ":rsw      actitvates random facets in facet-counting goal-oriented mode until a unique solution is reached",
    ":s        filter facets w.r.t. to currently used combination of mode and weight, prompts user to activate a filtered facet and calls `?n`",
    ":sn       filter facets w.r.t. to currently used combination of mode and weight, prompts user to activate a filtered facet and calls `?nn`",
    ":sm       switches current combination of mode and weight to specified combination of mode and weight",
    ":q        exits",
    "\nqueries:\n?fc       returns the number of current facets",
    "?fs       returns the current facets",
    "?ifs      returns the initial facets",
    "?ifc      returns the number of initial facets",
    "?m        returns the currently used combination of mode and weight",
    "?n        solves program on current route and outputs all solutions",
    "?nn       solves program on current route and by default outputs --n solutions",
    "?rs       returns true, if provided route is safe, false otherwise",
    "?rms      returns true, if provided route is maximal safe, false otherwise",
    "?src      returns the logic program source code, fasb is reading from",
    "?w        returns the currently used weight value of the provided facet; returns weight of all current facets, if no facet is provided",
    "?z        returns the zoom in effect percentage of the provided facet; returns zoom in effects of all current facets, if no facet is provided",
    "?zh       returns true if zoom in effect of provided facet is higher or equal to provided bound, otherwise false",
    "?zl       returns true if zoom in effect of provided facet is lower or equal to provided bound, otherwise false",
    "\nfor a more detailed manual w.r.t. a certain command or query use `?man` and provide the functionality in question",
];

#[cfg(not(tarpaulin_include))]
pub fn manual_command_or_query(input: &str) {
    match input {
        ":a" | "--activate" => println!("
        `--activate`
            short: `:a`
            description: activates n provided whitespace separated facets
            parameters: 
                [REQUIRED] facets `f0 f1 ... fn`
            errors: no op for invalid input with error message. For n-ary facets with n >= 2 use `some_atom(x0,x1)` instead of `some_atom(x0, x1)`
            syntax: `:a f0 f1 ... fn`
        "),
        ":d" | "--deactivate" => println!("
        `--deactivate`
            short: `:d`
            description: deactivates n provided whitespace separated facets; if a facet is activated multiple times, any occurence will be deactivated
            parameters: 
                [REQUIRED] facets `f0 f1 ... fn`
            errors: no op for invalid input with error message; for n-ary facets with n >= 2 use `some_atom(x0,x1)` instead of `some_atom(x0, x1)`
            syntax: `:d f0 f1 ... fn`
        "),
        ":cr" | "--clear-route" => println!("
        `--clear-route`
            short: `:cr`
            description: clears the current route, i.e., sets empty route as current route
            parameters: 
            errors:  no op for route = < > 
            syntax: `:cr`
        "),
        ":zha" | "--zoom-higher-than-and-activate" => println!("
        `--zoom-higher-than-and-activate`
            short: `:zha`
            description: activates first facet found with zoom in effect higher than or equal to the provided bound
            parameters: 
                [REQUIRED] bound f32
            errors: no op, if no bound is provided with error message 
            syntax: `:zha f32`
        "),
        ":zla" | "--zoom-lower-than-and-activate" => println!("
        --zoom-lower-than-and-activate`
            short: `:zla`
            description: activates first facet found with zoom in effect lower than or equal to the provided bound
            parameters: 
                [REQUIRED] bound f32
            errors: no op, if no bound is provided with error message
            syntax: `:zla f32`
        "),
        ":rss" | "--random-safe-steps" => println!("
        `--random-safe-steps`
            short: `:rss`
            description: actitvates n random facets w.r.t. the specified combination of mode and weight
            parameters: 
                n `u64`; if not provided, as many steps as needed to reach unique solution will be taken
                mode; by default --go
                weight; by default --fc
            errors: no op for invalid combination of mode and weight or pace = 100% with error message
            syntax: `:rss n mode weight`, `:rss`
        "),
        ":rsw" | "--random-safe-walk" => println!("
        `--random-safe-walk`
            short: `:rsw`
            description: actitvates random facets in facet-counting goal-oriented mode until a unique solution reached
            parameter: 
            errors: no op, if pace = 100% 
            syntax: `:rsw`
        "),
        ":s" | "--step" => println!("
        `--step`
            short: `:s`
            description: filter facets w.r.t. to currently used combination of mode and weight, prompts user to activate a filtered facet and calls `?-navigate`
            parameters: 
            errors: no op, if pace = 100% 
            syntax: `:s`
        "),
        ":sn" | "--step-n" => println!("
        `--step-n`
            short: `:sn`
            description: filter facets w.r.t. to currently used combination of mode and weight, prompts user to activate a filtered facet and calls `?-navigate-n`
            parameters*: 
            errors: no op, if pace = 100% 
            syntax: `:sn`
        "),
        ":sm" | "--switch-mode" => println!("
        `--switch-mode`
            short: `:sm`
            description: switches current combination of mode and weight to specified combination of mode and weight
            parameters: 
                [REQUIRED] mode
                [REQUIRED] weight 
            errors: no op for invalid combination of mode and weight with error message
            syntax: `:sm`
        "),
        ":q" | "--quit" => println!("
        `--quit`
            short: `:q`
            description: exits
            parameters: 
            errors: 
            syntax: `:q`
        "),
        "?fc" | "?-facets-count" => println!("
        `?-facets-count`
            short: `?fc`
            description: returns the number of current facets
            parameters: 
            errors: 
            syntax: `?fc`
        "),
        "?fs" | "?-facets" => println!("
        `?-facets`
            short: `?fs`
            description: returns the current facets
            parameters: 
            errors: 
            syntax: `?fs`
        "),
        "?ifc" | "?-initial-facets-count" => println!("
        `?-initial-facets-count`
            short: `?ifc`
            description: returns the number of initial facets
            parameters: 
            errors: 
            syntax: `?ifc`
        "),
        "?ifs" | "?-initial-facets" => println!("
        `?-initial-facets`
            short: `?ifs`
            description: returns the initial facets
            parameters: 
            errors: 
            syntax: `?ifs`
        "),
        "?m" | "?-mode" => println!("
        `?-mode`
            short: `?m`
            description: returns the currently used combination of mode and weight
            parameters: 
            errors: 
            syntax: `?m`
        "),
        "?n" | "?-navigate" => println!("
        `?-navigate`
            short: `?n`
            description: solves program on current route and outputs all solutions
            parameters: 
            errors: 
            syntax: `?n`
        "),
        "?nn" | "?-navigate-n" => println!("
        `?-navigate-n`
            short: `?nn`
            description: solves program on current route and outputs all solutions
            parameters: 
                n `u64; if not provided n is as specified at startup
            errors: 
            syntax: `?nn`
        "),
        "?rs" | "?-route-safe" => println!("
        `?-route-safe`
            short: `?m`
            description: returns true, if provided route is safe, false otherwise; there a several ways to provide a route:
                route: `< f0 f1 ... fn >` checks, whether `< f0 f1 ... fn >` is safe
                peek on route: `+ f0 f1 ... fn` checks, whether current route + `f0 f1 ... fn` is safe
                current route: no argument checks, wether current route is safe
            parameters: 
                route
            errors: no op for invalid syntax or invalid facets
            syntax: `?rs < f0 f2 ... fn >`, `?rs + f0 f1 ... fn`, `?rs `
        "),
        "?rms" | "?-route-maximal-safe" => println!("
        `?-route-maximal-safe`
            short: `?m`
            description: returns true, if provided route is maximal safe, false otherwise; there a several ways to provide a route:
                route: `< f0 f1 ... fn >` checks, whether `< f0 f1 ... fn >` is maximal safe
                peek on route: `+ f0 f1 ... fn` checks, whether current route + `f0 f1 ... fn` is maximal safe
                current route: no argument checks, wether current route is maximal safe
            parameters: 
                route
            errors: no op for invalid syntax or invalid facets
            syntax: `?rms < f0 f2 ... fn >`, `?rms + f0 f1 ... fn`, `?rms `
        "),
        "?src" | "?-source" => println!("
        `?-source`
            short: `?src`
            description: returns the logic program source code, fasb is reading from
            parameters: 
            errors: 
            syntax: `?src`
        "),
        "?w" | "?-weight" => println!("
        `?-weight`
            short: `?w`
            description: returns the current weight of the provided facet; returns weight of all current facets, if no facet is provided
            parameters: 
                facet `f`
            errors:  no op for invalid input with error message
            syntax: `?w f`, `?w `
        "),
        "?z" | "?-zoom" => println!("
        `?-zoom`
            short: `?z`
            description: returns the zoom in effect percentage of the provided facet; returns zoom in effects of all current facets, if no facet is provided
            parameters: 
                facet `f`
            errors:  no op for invalid facet with error message
            syntax: `?z f`, `?z `
        "),
        "?zh" | "?-zoom-higher-than" => println!("
        `?-zoom-higher-than`
            short: `?zh`
            description: returns true if zoom in effect of provided facet is higher or equal to provided bound, otherwise false
            parameters: 
                [REQUIRED] facet `f` 
                [REQUIRED] bound  f32
            errors:  no op for invalid input or bound with error message
            syntax: `?zh f f32`
        "),
        "?zl" | "?-zoom-lower-than" => println!("
        `?-zoom-lower-than`
            short: `?zl`
            description: returns true if zoom in effect of provided facet is lower or equal to provided bound, otherwise false
            parameters: 
                [REQUIRED] facet `f` 
                [REQUIRED] bound  f32
            errors:  no op for invalid input or bound with error message
            syntax: `?zl f f32`
        "),
       _ => println!("\nunknown command or query: {:?}\n", input), 
    }
}

pub const CONFIG: Config<'static> = Config {
    name: NAME,
    authors: AUTHORS,
    version: VERSION,
    help: HELP,
    manual: MANUAL,
};
