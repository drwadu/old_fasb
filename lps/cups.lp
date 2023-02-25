box(1..3). 
label(b,b). label(r,b). label(r,r).
{in(X,Y,Z) : label(Y,Z)} = 1 :- box(X). 
:- in(X,Y,Z), in(X',Y,Z), X!=X'.
{saw(X,Y) : box(X), label(Y,_)} = 1.
:- not in(X,Y,_), not in(X,_,Y), saw(X,Y). 
#show in/3.
#show saw/2.