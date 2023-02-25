
cov(a) :- c(0).
:- not cov(a).
con(a,0) :- c(0).
cov(c) :- c(1).
:- not cov(c).
con(c,1) :- c(1).
con(f,1) :- c(1).
con(b,1) :- c(1).
cov(f) :- c(2).
:- not cov(f).
cov(c) :- c(2).
:- not cov(c).
con(c,2) :- c(2).
con(f,2) :- c(2).
cov(b) :- c(3).
:- not cov(b).
con(b,3) :- c(3).
con(c,3) :- c(3).
con(d,3) :- c(3).
cov(b) :- c(4).
:- not cov(b).
cov(d) :- c(4).
:- not cov(d).
con(b,4) :- c(4).
con(d,4) :- c(4).
2 {c(X) : X=0..4}.
:- c(X),c(Y),X!=Y,con(I,X),con(I,Y).
#show c/1.