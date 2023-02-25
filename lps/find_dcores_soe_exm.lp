2 {c(X) : X=0..4}.


% {d}
cov(b) :- c(0).
cov(d) :- c(0).
con(b,0). con(d,0).

% {a}
cov(a) :- c(1).
con(a,1). 

% {b}
cov(b) :- c(2).
con(b,2). con(c,2). con(d,2). 

% {f}
cov(c) :- c(3).
cov(f) :- c(3).
con(c,3). con(c,3).

% {c}
cov(c) :- c(4).
con(b,4). con(c,4). con(f,4).

% rest omitted due cover is CC

:- not cov(a).
:- not cov(b).
:- not cov(c).
:- not cov(d).
:- not cov(f).

:- c(X),c(Y),X!=Y,con(I,X),con(I,Y).

#show c/1.
