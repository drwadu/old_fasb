#![allow(dead_code)]

pub struct Config<'a> {
    pub name: &'a str,
    pub authors: &'a str,
    pub version: &'a str,
    pub help: [&'a str; 8],
    pub manual: [&'a str; 22],
}

const NAME: &str = env!("CARGO_PKG_NAME");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP: [&str; 8] = [
    "PROGRAM",
    "\t[REQUIRED] path to the .lp file to read",
    "NAVIGATION_MODE    [--goal-oriented | --go] | [--strictly-goal-oriented | --sgo] | [--explore | --expl]", 
    "\t[OPTIONAL] specifies the navigation mode; by default --goal-oriented", 
    "WEIGHT_TYPE        [--absolute | --abs] | [--facet-counting | --fc]", 
    "\t[OPTIONAL] specifies the weight type; by default --facet-counting", 
    "DEFAULT_N  --n=int",
    "\t[OPTIONAL] specifies the number of solutions to output by --navigate-n-models or :nn; by default --n=3", 
];

const MANUAL: [&str; 22] = [
    "--manual                                            :man\treturns manual", 
    "--logic-program                                     :lp\treturns source code of logic program, wfasb reads from", 
    "--facets                                            :fs\treturns current facets",
    "--facets-count                                       :fc\treturns count of current facets",
    "--initial-facets                                    :ifs\treturns initial facets",
    "--initial-facets-count                              :ifc\treturns count of initial facets",
    "--activate                                          :a\t`:a f0 f1 ... fn` activates facets f0, f1, ..., fn",
    "--deactivate                                        :d\t`:d f0 f1 ... fn` deactivates facets f0, f1, ..., fn",
    "--step                                              :s\t`:s NAVIGATION_MODE WEIGHT_TYPE` performs navigation step w.r.t. specified mode and returns DEFAULT_N solutions, by default mode is current mode",
    "--step-n                                            :sn\t`:sn n=int NAVIGATION_MODE WEIGHT_TYPE` performs navigation step w.r.t. specified mode and returns n solutions, by default mode is current mode, n is DEFAULT_N",
    "--navigate                                          :n\tsolves program w.r.t. current route and outputs all solutions",
    "--navigate-n-models                                 :nn \t`:nn n=int` solves program w.r.t. current route and outputs n (by defaut n=DEFAULT_N) solutions",
    "--find-facet-with-zoom-higher-than-and-activate     :zha\t`:zha bound=float` activates first facet found s.t. weight of f is higher than or equal to bound",
    "--find-facet-with-zoom-lower-than-and-activate      :zla\t`:zla bound=float` activates first facet found s.t. weight of f lower than or equal to bound",
    "?-weight                                            ?w\t`?w f` returns weight of f; returns weight of all current facets if no f is provided",
    "?-zoom                                              ?z\t`?z f` returns zoom-in effect of f; returns zoom-in effect of all current facets if no f is provided",
    "?-route-safe                                        ?rs\t`?rs r=< f0 f1 ... fn>` checks whether r is safe, r is by default current route",
    "?-route-maximal-safe                                ?rms\t`?rms r=< f0 f1 ... fn>` checks whether r is maximal safe, r is by default current route",
    "?-zoom-higher-than                                  ?zh\t`?zh bound=float` returns first facet found s.t. weight of f is higher than or equal to bound",
    "?-zoom-lower-than                                   ?zl\t`?zl bound=float` returns first facet found s.t. weight of f is lower than or equal to bound",
    "--mode                                              :m\treturns current navigation mode",
    "--quit                                              :q\texits",
];

pub const CONFIG: Config<'static> = Config {
    name: NAME,
    authors: AUTHORS,
    version: VERSION,
    help: HELP,
    manual: MANUAL,
};
