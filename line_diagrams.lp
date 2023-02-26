element(1..4).
0 { leq(X,Y) : element(X), element(Y), X!=Y } 4-1.
:- leq(X,Y), leq(Y,X).
:- leq(X,Y), leq(Y,X).
#show leq/2.
