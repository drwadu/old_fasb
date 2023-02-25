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

# Usage
To give a concise demo of fasb and answer set navigation, consider the
following toy problem, which will be used in the usage section. Suppose we have
three cups labeled by their content. Either cup contains, two blue balls, two
red balls or one blue and one red ball. No cup label matches what's in the cup.
Fix the labels by blindly pulling a ball out of some cup.

First, start up fasb with the logic program encoding our problem as input. We
inspect the encoding of the toy problem by query `?src`. 

The default navigation mode is counting facets, by means of which we can check
which event of having seen a ball in a cup (e.g.: `saw(b,1)` standing for a
blue ball in cup 1) reduces uncertainty the most. Inspecting the facet-counting
facets by query `?w`, we can observe that each event reduces uncertainty by
47%. Had we seen that reaching into one cup reduces uncertainty more than into
another one, we could have gone for this one. However, we need to guess
indeterministically. Say, we reach into cup 1 and see a blue ball. We encode
this by acitvating facet `saw(b,1)` with `:a saw(b,1)`.


