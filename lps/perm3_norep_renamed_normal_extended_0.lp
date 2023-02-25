a :- not e, not g.
e :- not a, not g.
g :- not a, not e.
b :- a, not h.
h :- a, not b.
c :- a, b.
i :- a, h.
d :- e, not f.
f :- e, not d.
b :- e, d.
i :- e, f.
c :- g, not d.
d :- g, not c.
f :- g, c.
h :- g, d.

%%% added knowledge
k :- not l, a.
k :- not l, f.
l :- not k, a.
l :- not k, f.