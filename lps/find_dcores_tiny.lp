2 {c(X) : X=0..3}.


% {~a,b}
cov(b) :- c(0).
con(b,0). con(c,0). con(d,0).

% {a,~b}
cov(a) :- c(1).
con(a,1). 

% {d}
cov(b) :- c(2).
cov(d) :- c(2).
con(b,2). con(d,2).

% {c}
cov(b) :- c(3).
cov(c) :- c(3).
con(b,3). con(c,3).

% rest omitted due cover is CC

:- not cov(a).
:- not cov(b).
:- not cov(c).
:- not cov(d).

%:- c(X),c(Y),X!=Y,con(I,X),con(I,Y).

#show c/1.

