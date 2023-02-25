import clingo
from typing import Any, Dict, List, Iterable, Set, Tuple


def facets(ctl: clingo.Control, route: Iterable[int], f: Any = id) -> Iterable[int]:
    ctl.configuration.solve.enum_mode = "brave"
    with ctl.solve(yield_=True, assumptions=route) as h:
        bc = set([model.symbols(atoms=True) for model in h][-1])

    ctl.configuration.solve.enum_mode = "cautious"
    with ctl.solve(yield_=True, assumptions=route) as h:
        cc = set([model.symbols(atoms=True) for model in h][-1])

    ctl.configuration.solve.enum_mode = "auto"

    fs = bc.difference(cc)

    return map(f, fs)

def ccs(ctl: clingo.Control, route: Iterable[int], f: Any = id) -> Iterable[int]:
    ctl.configuration.solve.enum_mode = "cautious"
    with ctl.solve(yield_=True, assumptions=route) as h:
        cc = set([model.symbols(atoms=True) for model in h][-1])

    ctl.configuration.solve.enum_mode = "auto"

    return map(f, cc)

def remap(facet: Any) -> int:
    facet = str(facet)
    if facet.startswith("-"):
        return -MAPS[facet[1:]]
    else:
        return MAPS[facet]

def com_cov(ctl: clingo.Control, fs: List[int]) -> None:
    xs = dict()
    for f in fs:
        cc = frozenset(ccs(ctl, [s.literal for s in ctl.symbolic_atoms if str(s.symbol) == f],str))
        if not xs.get(cc, None):
            xs[cc] = [f]
        else:
            xs[cc].append(f)
    for k,v in xs.items():
        print(*map(remap,v))
        print(*map(remap,k))
        print()



if __name__ == "__main__": 
    import sys


    lp = open(sys.argv[1],"r").read()
    MAPS = {x[1]:x[0] for x in map(lambda l: l.strip().split(" ")[1:], filter(lambda l: l.startswith("c "), open(sys.argv[2],"r").readlines()))}
    
    ctl = clingo.Control("0")
    ctl.add("base", [], lp)
    ctl.ground([("base", [])])

    fs = list(facets(ctl, [],str))
    print(fs)
    com_cov(ctl, fs)


    
