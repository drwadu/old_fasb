% core, if contents of components cover all BC

2 {c(X) : X=0..4}.


% {d}
b :- c(0).
d :- c(0).

% {a}
a :- c(1).

% {b}
b :- c(2).

% {f}
c :- c(3).
f :- c(3).

% {c}
c :- c(4).

% rest omitted due cover is CC

:- not a.
:- not b.
:- not c.
:- not d.
:- not f.

#show c/1.
