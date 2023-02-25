box(1..3). 
label(b,b). label(b,r). label(r,r).

{in(X,Y,Z) : label(Y,Z)} = 1 :- box(X). 
:- in(X,Y,Z), in(X',Y,Z), X!=X'.

{saw(X,Y) : box(X), label(Y,_)} = 1.

:- not in(X,Y,_), saw(X,Y'), Y=Y'. 
:- not in(X,_,Z), saw(X,Z'), Z=Z'. 

#show in/3.
#show saw/2.
