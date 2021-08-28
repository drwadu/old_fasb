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
```
$ fasb grid.lp --sgo --fc

fasb version 0.1.0 [clingo version 5.4.0]

reading from grid.lp

< > [ 0% ] ~> ?src

cell(1..9).
obj(1..9).
{set_obj_cell(X,C) : obj(X)} = 1 :- cell(C).
:- set_obj_cell(X,C1), set_obj_cell(X,C2), C1!=C2.

#show set_obj_cell/2.

< > [ 0% ] ~> ?fs

set_obj_cell(1,1) ~set_obj_cell(1,1) set_obj_cell(1,2) ~set_obj_cell(1,2) set_obj_cell(1,3) ~set_obj_cell(1,3) set_obj_cell(1,4) ~set_obj_cell(1,4) set_obj_cell(1,5) ~set_obj_cell(1,5) set_obj_cell(1,6) ~set_obj_cell(1,6) 
set_obj_cell(1,7) ~set_obj_cell(1,7) set_obj_cell(1,8) ~set_obj_cell(1,8) set_obj_cell(1,9) ~set_obj_cell(1,9) set_obj_cell(2,1) ~set_obj_cell(2,1) set_obj_cell(2,2) ~set_obj_cell(2,2) set_obj_cell(2,3) ~set_obj_cell(2,3) 
set_obj_cell(2,4) ~set_obj_cell(2,4) set_obj_cell(2,5) ~set_obj_cell(2,5) set_obj_cell(2,6) ~set_obj_cell(2,6) set_obj_cell(2,7) ~set_obj_cell(2,7) set_obj_cell(2,8) ~set_obj_cell(2,8) set_obj_cell(2,9) ~set_obj_cell(2,9) 
set_obj_cell(3,1) ~set_obj_cell(3,1) set_obj_cell(3,2) ~set_obj_cell(3,2) set_obj_cell(3,3) ~set_obj_cell(3,3) set_obj_cell(3,4) ~set_obj_cell(3,4) set_obj_cell(3,5) ~set_obj_cell(3,5) set_obj_cell(3,6) ~set_obj_cell(3,6) 
set_obj_cell(3,7) ~set_obj_cell(3,7) set_obj_cell(3,8) ~set_obj_cell(3,8) set_obj_cell(3,9) ~set_obj_cell(3,9) set_obj_cell(4,1) ~set_obj_cell(4,1) set_obj_cell(4,2) ~set_obj_cell(4,2) set_obj_cell(4,3) ~set_obj_cell(4,3) 
set_obj_cell(4,4) ~set_obj_cell(4,4) set_obj_cell(4,5) ~set_obj_cell(4,5) set_obj_cell(4,6) ~set_obj_cell(4,6) set_obj_cell(4,7) ~set_obj_cell(4,7) set_obj_cell(4,8) ~set_obj_cell(4,8) set_obj_cell(4,9) ~set_obj_cell(4,9) 
set_obj_cell(5,1) ~set_obj_cell(5,1) set_obj_cell(5,2) ~set_obj_cell(5,2) set_obj_cell(5,3) ~set_obj_cell(5,3) set_obj_cell(5,4) ~set_obj_cell(5,4) set_obj_cell(5,5) ~set_obj_cell(5,5) set_obj_cell(5,6) ~set_obj_cell(5,6) 
set_obj_cell(5,7) ~set_obj_cell(5,7) set_obj_cell(5,8) ~set_obj_cell(5,8) set_obj_cell(5,9) ~set_obj_cell(5,9) set_obj_cell(6,1) ~set_obj_cell(6,1) set_obj_cell(6,2) ~set_obj_cell(6,2) set_obj_cell(6,3) ~set_obj_cell(6,3) 
set_obj_cell(6,4) ~set_obj_cell(6,4) set_obj_cell(6,5) ~set_obj_cell(6,5) set_obj_cell(6,6) ~set_obj_cell(6,6) set_obj_cell(6,7) ~set_obj_cell(6,7) set_obj_cell(6,8) ~set_obj_cell(6,8) set_obj_cell(6,9) ~set_obj_cell(6,9) 
set_obj_cell(7,1) ~set_obj_cell(7,1) set_obj_cell(7,2) ~set_obj_cell(7,2) set_obj_cell(7,3) ~set_obj_cell(7,3) set_obj_cell(7,4) ~set_obj_cell(7,4) set_obj_cell(7,5) ~set_obj_cell(7,5) set_obj_cell(7,6) ~set_obj_cell(7,6) 
set_obj_cell(7,7) ~set_obj_cell(7,7) set_obj_cell(7,8) ~set_obj_cell(7,8) set_obj_cell(7,9) ~set_obj_cell(7,9) set_obj_cell(8,1) ~set_obj_cell(8,1) set_obj_cell(8,2) ~set_obj_cell(8,2) set_obj_cell(8,3) ~set_obj_cell(8,3) 
set_obj_cell(8,4) ~set_obj_cell(8,4) set_obj_cell(8,5) ~set_obj_cell(8,5) set_obj_cell(8,6) ~set_obj_cell(8,6) set_obj_cell(8,7) ~set_obj_cell(8,7) set_obj_cell(8,8) ~set_obj_cell(8,8) set_obj_cell(8,9) ~set_obj_cell(8,9) 
set_obj_cell(9,1) ~set_obj_cell(9,1) set_obj_cell(9,2) ~set_obj_cell(9,2) set_obj_cell(9,3) ~set_obj_cell(9,3) set_obj_cell(9,4) ~set_obj_cell(9,4) set_obj_cell(9,5) ~set_obj_cell(9,5) set_obj_cell(9,6) ~set_obj_cell(9,6) 
set_obj_cell(9,7) ~set_obj_cell(9,7) set_obj_cell(9,8) ~set_obj_cell(9,8) set_obj_cell(9,9) ~set_obj_cell(9,9) 

< > [ 0% ] ~> ?z set_obj_cell(1,1)

solving...

set_obj_cell(1,1) : 20.9877%

call    : ?-zoom set_obj_cell(1,1)
weight  : facet-counting
elapsed : 12.254999ms

< > [ 0% ] ~> :a set_obj_cell(1,1)
< set_obj_cell(1,1) > [ 21% ] ~> :a set_obj_cell(2,1) set_obj_cell(3,3)
< set_obj_cell(1,1) set_obj_cell(2,1) set_obj_cell(3,3) > [ UNSAT ] ~> ?rs < set_obj_cell(1,1) set_obj_cell(3,3)

solving...

true

call    : ?-route-safe < set_obj_cell(1,1) set_obj_cell(3,3) >
elapsed : 1.232581ms

< set_obj_cell(1,1) set_obj_cell(2,1) set_obj_cell(3,3) > [ UNSAT ] ~> :d set_obj_cell(2,1)
< set_obj_cell(1,1) set_obj_cell(3,3) > [ 40% ] ~> :zha 0.2

solving...

no result

call            : --find-facet-with-zoom-higher-than-and-activate 0.2
navigation mode : facet-counting strictly-goal-oriented mode
elapsed         : 220.960587ms

< set_obj_cell(1,1) set_obj_cell(3,3) > [ 40% ] ~> :zha 0.1

solving...

call            : --find-facet-with-zoom-higher-than-and-activate 0.1
navigation mode : facet-counting strictly-goal-oriented mode
elapsed         : 11.656472ms

< set_obj_cell(1,1) set_obj_cell(3,3) set_obj_cell(9,4) > [ 56% ] ~> :sn

call            : --step-n
navigation mode : facet-counting strictly-goal-oriented mode
filtered        : 36/72
elapsed         : 143.427385ms

set_obj_cell(8,6) set_obj_cell(5,5) set_obj_cell(6,5) set_obj_cell(4,8) set_obj_cell(6,9) set_obj_cell(7,9) set_obj_cell(8,8) set_obj_cell(5,6) set_obj_cell(4,6) set_obj_cell(6,8) set_obj_cell(2,2) set_obj_cell(4,9) set_obj_cell(5,9) set_obj_cell(6,2) set_obj_cell(2,8) set_obj_cell(7,8) set_obj_cell(2,9) set_obj_cell(8,7) set_obj_cell(7,2) set_obj_cell(5,2) set_obj_cell(2,5) set_obj_cell(6,6) set_obj_cell(2,7) set_obj_cell(4,7) set_obj_cell(2,6) set_obj_cell(7,6) set_obj_cell(8,5) set_obj_cell(4,2) set_obj_cell(4,5) set_obj_cell(7,7) set_obj_cell(8,2) set_obj_cell(5,8) set_obj_cell(5,7) set_obj_cell(7,5) set_obj_cell(8,9) set_obj_cell(6,7) 

activate: set_obj_cell(4,8)

solving...

Answer 1: 
set_obj_cell(1,1) set_obj_cell(2,5) set_obj_cell(3,3) set_obj_cell(4,8) set_obj_cell(5,6) set_obj_cell(6,2) set_obj_cell(7,7) set_obj_cell(8,9) set_obj_cell(9,4) 
Answer 2: 
set_obj_cell(1,1) set_obj_cell(2,5) set_obj_cell(3,3) set_obj_cell(4,8) set_obj_cell(5,9) set_obj_cell(6,2) set_obj_cell(7,7) set_obj_cell(8,6) set_obj_cell(9,4) 
Answer 3: 
set_obj_cell(1,1) set_obj_cell(2,6) set_obj_cell(3,3) set_obj_cell(4,8) set_obj_cell(5,5) set_obj_cell(6,2) set_obj_cell(7,7) set_obj_cell(8,9) set_obj_cell(9,4) 
SATISFIABLE

call    : ?-navigate-n 3
elapsed : 1.304382ms

< set_obj_cell(1,1) set_obj_cell(3,3) set_obj_cell(9,4) set_obj_cell(4,8) > [ 69% ] ~> :rss 1 --expl --fc

solving...

step 0
navigation mode : facet-counting explore mode
filtered        : 25/50
elapsed         : 85.788606ms

call            : --random-safe-steps 1
navigation mode : facet-counting explore mode
elapsed         : 93.316118ms

< set_obj_cell(1,1) set_obj_cell(3,3) set_obj_cell(9,4) set_obj_cell(4,8) ~set_obj_cell(8,2) > [ 70% ] ~> ?rms + set_obj_cell(2,2)

solving...

false

call    : ?-route-maximal-safe < set_obj_cell(1,1) set_obj_cell(3,3) set_obj_cell(9,4) set_obj_cell(4,8) ~set_obj_cell(8,2) set_obj_cell(2,2) >
elapsed : 9.542704ms

< set_obj_cell(1,1) set_obj_cell(3,3) set_obj_cell(9,4) set_obj_cell(4,8) ~set_obj_cell(8,2) > [ 70% ] ~> :rsw

solving...

step 0
step 1
step 2

call            : --random-safe-walk
navigation mode : goal-oriented
elapsed         : 15.096645ms

< set_obj_cell(1,1) set_obj_cell(3,3) set_obj_cell(9,4) set_obj_cell(4,8) ~set_obj_cell(8,2) set_obj_cell(7,9) set_obj_cell(6,6) set_obj_cell(5,5) > [ 100% ] ~> ?n

solving...

Answer 1: 
set_obj_cell(1,1) set_obj_cell(2,2) set_obj_cell(3,3) set_obj_cell(4,8) set_obj_cell(5,5) set_obj_cell(6,6) set_obj_cell(7,9) set_obj_cell(8,7) set_obj_cell(9,4) 
SATISFIABLE

call    : ?-navigate
elapsed : 1.300003ms

< set_obj_cell(1,1) set_obj_cell(3,3) set_obj_cell(9,4) set_obj_cell(4,8) ~set_obj_cell(8,2) set_obj_cell(7,9) set_obj_cell(6,6) set_obj_cell(5,5) > [ 100% ] ~> :q
```

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
    * **description**: activates n provided whitespace separated facets
    * **parameters**: 
        * [REQUIRED] facets `f0 f1 ... fn`
    * **errors**: no op for invalid input with error message. For n-ary facets with n >= 2 use `some_atom(x0,x1)` instead of `some_atom(x0, x1)`
    * **syntax**: `:a f0 f1 ... fn`
* `--deactivate`
    * **short**: `:d`
    * **description**: deactivates n provided whitespace separated facets; if a facet is activated multiple times, any occurence will be deactivated
    * **parameters**: 
        * [REQUIRED] facets `f0 f1 ... fn`
    * **errors**: no op for invalid input with error message; for n-ary facets with n >= 2 use `some_atom(x0,x1)` instead of `some_atom(x0, x1)`
    * **syntax**: `:d f0 f1 ... fn`
* `--clear-route`
    * **short**: `:cr`
    * **description**: clears the current route, i.e., sets empty route as current route
    * **parameters**: 
    * **errors**:  no op for route = < > 
    * **syntax**: `:cr`
* `--zoom-higher-than-and-activate`
    * **short**: `:zha`
    * **description**: activates first facet found with zoom in effect higher than or equal to the provided bound
    * **parameters**: 
        * [REQUIRED] bound `f32`
    * **errors**: no op, if no bound is provided with error message 
    * **syntax**: `:zha f32`
* `--zoom-lower-than-and-activate`
    * **short**: `:zla`
    * **description**: activates first facet found with zoom in effect lower than or equal to the provided bound
    * **parameters**: 
        * [REQUIRED] bound `f32` 
    * **errors**: no op, if no bound is provided with error message
    * **syntax**: `:zla f32`
* `--random-safe-steps`
    * **short**: `:rss`
    * **description**: actitvates n random facets w.r.t. the specified combination of mode and weight
    * **parameters**: 
        * n `u64`; if not provided, as many steps as needed to reach unique solution will be taken
        * mode; by default --go
        * weight; by default --fc
    * **errors**: no op for invalid combination of mode and weight or pace = 100% with error message
    * **syntax**: `:rss n mode weight`, `:rss`
* `--random-safe-walk`
    * **short**: `:rsw`
    * **description**: actitvates random facets in facet-counting goal-oriented mode until a unique solution reached
    * **parameters**: 
    * **errors**: no op, if pace = 100% 
    * **syntax**: `:rsw`
* `--step`
    * **short**: `:s`
    * **description**: filter facets w.r.t. to currently used combination of mode and weight, prompts user to activate a filtered facet and calls `?-navigate`
    * **parameters**: 
    * **errors**: no op, if pace = 100% 
    * **syntax**: `:s`
* `--step-n`
    * **short**: `:sn`
    * **description**: filter facets w.r.t. to currently used combination of mode and weight, prompts user to activate a filtered facet and calls `?-navigate-n`
    * **parameters**: 
    * **errors**: no op, if pace = 100% 
    * **syntax**: `:sn`
* `--switch-mode`
    * **short**: `:sm`
    * **description**: switches current combination of mode and weight to specified combination of mode and weight
    * **parameters**: 
        * [REQUIRED] mode
        * [REQUIRED] weight 
    * **errors**: no op for invalid combination of mode and weight with error message
    * **syntax**: `:sm`
* `--quit`
    * **short**: `:q`
    * **description**: exits
    * **parameters**: 
    * **errors**: 
    * **syntax**: `:q`

## 2.2 Queries
`fasb` provides the following queries:
* `?-facets-count`
    * **short**: `?fc`
    * **description**: returns the number of current facets
    * **parameters**: 
    * **errors**: 
    * **syntax**: `?fc`
* `?-facets`
    * **short**: `?fs`
    * **description**: returns the current facets
    * **parameters**: 
    * **errors**: 
    * **syntax**: `?fs`
* `?-initial-facets-count`
    * **short**: `?ifc`
    * **description**: returns the number of initial facets
    * **parameters**: 
    * **errors**: 
    * **syntax**: `?ifc`
* `?-initial-facets`
    * **short**: `?ifs`
    * **description**: returns the initial facets
    * **parameters**: 
    * **errors**: 
    * **syntax**: `?ifs`
* `?-mode`
    * **short**: `?m`
    * **description**: returns the currently used combination of mode and weight
    * **parameters**: 
    * **errors**: 
    * **syntax**: `?m`
* `?-navigate`
    * **short**: `?n`
    * **description**: solves program on current route and outputs all solutions
    * **parameters**: 
    * **errors**: 
    * **syntax**: `?n`
* `?-navigate-n`
    * **short**: `?nn`
    * **description**: solves program on current route and outputs all solutions
    * **parameters**: 
        * n `u64`; if not provided n is as specified at startup
    * **errors**: 
    * **syntax**: `?nn`
* `?-route-safe`
    * **short**: `?m`
    * **description**: returns true, if provided route is safe, false otherwise; there a several ways to provide a route:
        * route: `< f0 f1 ... fn >` checks, whether `< f0 f1 ... fn >` is safe
        * peek on route: `+ f0 f1 ... fn` checks, whether current route + `f0 f1 ... fn` is safe
        * current route: no argument checks, wether current route is safe
    * **parameters**: 
        * route
    * **errors**: no op for invalid syntax or invalid facets
    * **syntax**: `?rs < f0 f2 ... fn >`, `?rs + f0 f1 ... fn`, `?rs `
* `?-route-maximal-safe`
    * **short**: `?m`
    * **description**: returns true, if provided route is maximal safe, false otherwise; there a several ways to provide a route:
        * route: `< f0 f1 ... fn >` checks, whether `< f0 f1 ... fn >` is maximal safe
        * peek on route: `+ f0 f1 ... fn` checks, whether current route + `f0 f1 ... fn` is maximal safe
        * current route: no argument checks, wether current route is maximal safe
    * **parameters**: 
        * route
    * **errors**: no op for invalid syntax or invalid facets
    * **syntax**: `?rms < f0 f2 ... fn >`, `?rms + f0 f1 ... fn`, `?rms `
* `?-source`
    * **short**: `?src`
    * **description**: returns the logic program source code, fasb is reading from
    * **parameters**: 
    * **errors**: 
    * **syntax**: `?src`
* `?-weight`
    * **short**: `?w`
    * **description**: returns the currently used weight value of the provided facet; returns weight of all current facets, if no facet is provided
    * **parameters**: 
        * facet `f`
    * **errors**:  no op for invalid input with error message.
    * **syntax**: `?w f`, `?w `
* `?-zoom`
    * **short**: `?z`
    * **description**: returns the zoom in effect percentage of the provided facet; returns zoom in effects of all current facets, if no facet is provided
    * **parameters**: 
        * facet `f`
    * **errors**:  no op for invalid facet with error message.
    * **syntax**: `?z f`, `?z `
* `?-zoom-higher-than`
    * **short**: `?zh`
    * **description**: returns true if zoom in effect of provided facet is higher or equal to provided bound, otherwise false
    * **parameters**: 
        * [REQUIRED] facet `f` 
        * [REQUIRED] bound  `f32`
    * **errors**:  no op for invalid input or bound with error message.
    * **syntax**: `?zh f f32`
* `?-zoom-lower-than`
    * **short**: `?zl`
    * **description**: returns true if zoom in effect of provided facet is lower or equal to provided bound, otherwise false
    * **parameters**: 
        * [REQUIRED] facet `f` 
        * [REQUIRED] bound  `f32` 
    * **errors**:  no op for invalid input or bound with error message.
    * **syntax**: `?zl f f32`
