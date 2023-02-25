% a b c
% b d e
% c f g
% d g h
% a h i
% e f i

a;e;g.

% a b c
% a h i
b;h :- a.
c :- a,b.
i :- a,h.

% b d e
% e f i
d;f :- e.
b :- e,d.
i :- e,f.


% c f g
% d g h
c;d :- g.
f :- g, c.
h :- g, d.

