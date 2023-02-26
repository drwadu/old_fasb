a :- not not b.
b :- not not a.
a :- not not c.
c :- not d.
d :- not c.
:- a, b, not c.
%:- not a.
%:- not b.
%:- c.
