sitzplatz(1..6).
person(1..6).

praferenz_sitzplatz(1,1). 

praferenz_sitznachbar(1,3).
%praferenz_sitznachbar(2,6).
%praferenz_sitznachbar(4,9).
praferenz_sitznachbar(2,4).
praferenz_sitznachbar(4,6).

{setze_person_an_platz(X,Y) : person(X)} = 1 :- sitzplatz(Y).
:- setze_person_an_platz(X,Y), setze_person_an_platz(X,Z), Y!=Z.

:- praferenz_sitzplatz(X,Y), not setze_person_an_platz(X,Y).
setze_person_an_platz(X,(Y+1) \ 6); setze_person_an_platz(X, (Y-1) \ 6) :- 
	praferenz_sitznachbar(X,Z), setze_person_an_platz(Z,Y).

#show setze_person_an_platz/2.
