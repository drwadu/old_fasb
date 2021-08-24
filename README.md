# 1. About
`fasb` implements the _weighted faceted answer set navigation_ framework on top of
the [`clingo`](https://github.com/potassco/clingo) solver, which allows for
manipulating the size of the solution space, i.e., the number of answer sets,
during navigation. During weighted-faceted navigation, navigation steps are
characterised w.r.t. the extent to which they affect the size of the solution
space, thereby we can navigate towards solutions at a configurable `pace` ([ n%
]) of navigation, which we consider to be the extent to which the current
`route` (< >) zooms into the solution space. The kind of parameter that allows
for configuration is called the _weight_ of a facet. Weights of facets enable
users to inspect effects of facets at any stage of navigation (< > [ n% ] ~>),
which allows for navigating more interactively in a systematic way. 

## 1.1 Example

# 2. Usage
`fasb` expects an answer set program that uses clingo syntax as input.
To invoke the fasb tool on a program (**path**) use:
    
    fasb path [mode] [weight] [n=]

Users can provide arguments to specify the navigation mode (**mode**) and the facet weight (**weight**) to use during navigation at startup. Both
can be changed during runtime. Furthermore, the number of solutions to enumerate with certain commands (**n**) can be specifed. The
value cannot be changed during runtime. Currently `fasb` supports the following combinations of weights and modes:

* absolute goal-oriented (--go --abs)
* absolute strictly-goal-oriented (--sgo --abs)
* absolute explore (--expl --abs)
* facet-counting goal-oriented (--go --fc)
* facet-counting strictly-goal-oriented (--sgo --fc)
* facet-counting explore (--expl --fc)

Use the `--help` flag to inspect the following command line options:
* [REQUIRED] **path**: path of the logic program (.lp file) to read from
* [OPTIONAL] **mode**: [--goal-oriented | --go] | [--strictly-goal-oriented | --sgo] | [--explore | --expl]
* [OPTIONAL] **weight**: [--absolute | --abs] | [--facet-counting | --fc]
* [OPTIONAL] **n**: u64

The basic call 

    fasb path

defaults to

    fasb path --go --fc --n=3

`fasb` provides functionality that distinguishes between commands (`--`, `:`) and queries (`?-`, `?`). A command is a call that mutates objects such as the route or the solution space. A query does not mutate objects, but rather solely returns answers.

To inspect an overview of commands and queries with short descriptions during runtime use the `?-manual` or `?man` query with no argument. For a more detailed manual w.r.t. to a certain command or query call `?-manual` or `?man` and provide the command or query in question; `?man` does not describe itself.

## 2.1 Commands
`fasb` provides the following commands:
* `--activate`
    * **short**: `:a`
    * **description**: activates n provided whitespace separated facets `f0 f1 ... fn`.
    * **parameters**: 
        * facets `f0 f1 ... fn`
    * **errors**: returns error for invalid input. For n-ary facets with n >= 2 use `some_atom(x0,x1)` instead of `some_atom(x0, x1)`.
    * **syntax**: `:a f0 f1 ... fn`
* `--deactivate`
    * **short**: `:d`
    * **description**: activates n provided whitespace separated facets `f0 f1 ... fn`; if a facet is activated multiple times, any occurence will be deactivated.
    * **parameters**: 
        * facets `f0 f1 ... fn`
    * **errors**: no op for invalid input; for n-ary facets with n >= 2 use `some_atom(x0,x1)` instead of `some_atom(x0, x1)`
    * **syntax**: `:d f0 f1 ... fn`
* `--clear-route`
    * **short**: `:cr`
    * **description**: clears the current route, i.e., sets empty route.
    * **parameters**: 
    * **errors**: 
    * **syntax**: `:cr`
* `--find-facet-with-zoom-higher-than-and-activate`
    * **short**: `:zha`
    * **description**: searches for first facet with zoom in effect higher than or equal to the provided bound.
    * **parameters**: 
        * bound in `[0;1]`
    * **errors**: no op, if no bound is provided
    * **syntax**: `:zha f32`
* `--find-facet-with-zoom-lower-than-and-activate`
    * **short**: `:zla`
    * **description**: searches for first facet with zoom in effect lower than or equal to the provided bound.
    * **parameters**: 
        * bound `[0;1]`
    * **errors**: no op, if no bound is provided
    * **syntax**: `:zla f32`
* `--random-safe-steps`, `:rss`
* `--random-safe-walk`, `:rsw`
* `--step`, `:s`
* `--step-n`, `:sn`
* `--switch-mode`, `:sm`
 