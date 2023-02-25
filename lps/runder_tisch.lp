person(1..6). platz(1..6).

praeferenz_platz(1,1). 
praeferenz_nachbarIn(1,3). praeferenz_nachbarIn(2,4). praeferenz_nachbarIn(4,6).

{ setze(X,Y) : person(X) } = 1 :- platz(Y).
:- setze(X,Y), setze(X,Z), Y!=Z. 

:- praeferenz_platz(X,Y), not setze(X,Y).
setze(X,(Y+1)\6); setze(X,(Y-1)\6) :- praeferenz_nachbarIn(X,Z), setze(Z,Y).

#show setze/2.