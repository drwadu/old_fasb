% a;e;g.
a :- not e, not g.
e :- not a, not g.
g :- not a, not e.

% a b c
% a h i
% b;h :- a.
b :- a, not h.
h :- a, not b.
c :- a,b.
i :- a,h.

% b d e
% e f i
% d;f :- e.
d :- e, not f.
f :- e, not d.
b :- e,d.
i :- e,f.


% c f g
% d g h
%c;d :- g.
c :- g, not d.
d :- g, not c.
f :- g, c.
h :- g, d.