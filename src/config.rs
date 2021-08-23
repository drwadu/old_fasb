pub struct Config<'a> {
    pub name: &'a str,
    pub authors: &'a str,
    pub version: &'a str,
    pub help: [&'a str; 7],
    pub manual: [&'a str; 23],
}

const NAME: &str = env!("CARGO_PKG_NAME");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP: [&str; 7] = [
    "usage               : fasb path [--mode] [--weight] [--n=int]",
    "default             : fasb path --goal-oriented --facet-counting --n=3\n",
    "[REQUIRED] --path   : path to the .lp file to read",
    "[OPTIONAL] --mode   : [--goal-oriented | --go] | [--strictly-goal-oriented | --sgo] | [--explore | --expl]",
    "[OPTIONAL] --weight : [--absolute | --abs] | [--facet-counting | --fc]",
    "[OPTIONAL] --n      : int",
    "\nuse `:man` to inspect manual during navigation",
];

const MANUAL: [&str; 23] = [
    "--source, :src                                          returns source code of logic program, fasb reads from",
    "--facets, :fs                                           returns current facets",
    "--facets-count, :fc                                     returns count of current facets",
    "--initial-facets, :ifs                                  returns initial facets",
    "--initial-facets-count, :ifc                            returns count of initial facets",
    "--step, :s                                              performs navigation step and returns all solutions",
    "--step-n, :s                                            performs navigation step returns --n solutions",
    "--navigate, :n                                          solves program on current route and outputs all solutions",
    "--activate, :a                                          activates facets",
    "--deactivate, :d                                        deactivates facets",
    "--navigate-n-models, :nn                                solves program on current route and by default outputs --n solutions",
    "--find-facet-with-zoom-higher-than-and-activate, :zha   activates first facet found with zoom in effect higher than or equal to bound",
    "--find-facet-with-zoom-lower-than-and-activate, :zla    activates first facet found with zoom in effect lower than or equal to bound",
    "--switch-mode, :sm                                      switches navigation mode",
    "?-weight, ?w                                            returns weight of facet; returns weight of all current facets if no facet is provided",
    "?-zoom, ?z                                              returns zoom in effect of facet; returns zoom-in effect of all current facets if no facet is provided",
    "?-route-safe, ?rs                                       checks whether route is safe; route is by default current route",
    "?-route-maximal-safe, ?rms                              checks whether route is maximal safe; route is by default current route",
    "?-zoom-higher-than, ?zh                                 returns first facet found with zoom in effect higher than or equal to bound",
    "?-zoom-lower-than, ?zl                                  returns first facet found with zoom in effect lower than or equal to bound",
    "--mode, :m                                              returns current navigation mode",
    "--quit, :q                                              exits",
    "\nfor more detailed manual w.r.t. certain command or query use `:man command` or `:man query`",
];

pub const CONFIG: Config<'static> = Config {
    name: NAME,
    authors: AUTHORS,
    version: VERSION,
    help: HELP,
    manual: MANUAL,
};
