a :- not b.
b :- not a.
x :- a, not y.
y :- a, not x.
%xx :- x.
%yy :- y.
xi :- b, not yi, not zi.
yi :- b, not xi, not zi.
zi :- b, not xi, not yi.