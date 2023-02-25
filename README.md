`fasb` is short for **faceted answer set browser**, which is a tool for
navigating solutions of a problem encoded as a logic program by means of answer
set programming (ASP).

# About
The essential concept is that users may explore sub-spaces by iteratively
enforcing so called _facets_ to be contained in each solution or no solution. A
facet is an atom that is contained in at least one, but not every solution. A
part from navigation, fasb supports quantitative reasoning regarding specific
quantities such as the number of answer sets or the number of facets within a
sub-space. The amount by which a facet decreases a quantity is called the
_weight_ of the facet. For more details on facets and weights see [this
paper](https://ojs.aaai.org/index.php/AAAI/article/view/20506) or [these
slides](https://easychair.org/smart-slide/slide/KcTv#).

![](https://github.com/drwadu/fasb/blob/master/.gif)

# Usage
To give a concise demo of fasb and answer set navigation, consider the
following toy problem.


## Example
Suppose we have three cups labeled by their content. Either cup contains, two
blue balls (bb), two red balls (rr) or one red and one blue ball (rb) . No cup
label matches what's in the cup. Suppose cup 1 is labeled by bb, cup 2 by rr
and cup 3 by rb. Fix the labels by blindly pulling a ball out of some cup.

### Demo
First, we start up fasb with the logic program encoding our problem as input.
We inspect the encoding of the toy problem by query `?src`, which reveals
that the program encodes the environment, but not yet the given labeling. 

Let's step into the sub-space encoding our knowledge based on the given
labeling and the problem specification. Cup 1 is labeled by bb, cup 2 by rr and
cup 3 by rb. We know that therefore cup i cannot contain xx, so we activate the
correspoding facet by command `:a ~in(1,b,b) ~in(2,r,r) ~in(3,r,b)`. By query
`?n` we enumerate all the remaining solutions. There are 8 possible solutions.

The default navigation mode is counting facets, by means of which we can check
by which amount the "event" of having seen a ball in a cup (e.g.: `saw(1,b)`
standing for a blue ball in cup 1) reduces uncertainty. Such an event is
essentially a facet `saw(X,Y)`saying we say a ball with color Y in cup X. Which
facets of these kind remain in the current sub-space? Query `?fs` reveals the
options. Inspecting their facet-counting weights by query `?w`, we can observe
that each facet reduces uncertainty more than others. In particular, we see
that reaching into cup 3 is the best guess, as for either outcome, we reduce
uncertainty by 100%. Guessing indeterministically could lead to, for instance,
seeing a red ball in cup 1, which according to the weight of `saw(r,1)` is not
conclusive. The reason as to why, may be figured out yourself. Anyways. So,
let's reach into cup 3. Say, we see a blue ball. Navigate towards `saw(3,b)`.
We reached a unique solution `in(3,b,b) in(2,r,b) in(1,r,r) saw(3,b)`.

In case of interest, finally one can also check what the probability for each
event was by using weights that refer to the amount of answer sets covered by a
facet (relative frequency). To do so, we switch to any kind of mode using
absolute weights, e.g., `:sm --sgo --abs`, which means we are in strictly
goal-oriented mode with absolute weights. To inspect the weights, again, we use
`?w`.


