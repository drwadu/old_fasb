schicht(t). schicht(n).
tag(mo). tag(di). tag(mi). tag(do). tag(fr).
angestellt(0..9). 

slot(X,Y) :- tag(X), schicht(Y).
{ p(X,Y,Z) : slot(Y,Z) } = 1 :- angestellt(X).
:- p(X0,Y,Z), p(X1,Y,Z), X0!=X1.

:- p(0,_,Z), Z=n.  
:- p(2,_,Z), Z=n.  
:- p(4,_,Z), Z=n.
:- p(6,_,Z), Z=t.  
:- p(9,_,Z), Z=t.  
:- p(1,Y,_), Y=mo.  
:- p(1,Y,_), Y=di.  
:- p(3,Y,_), Y=mo.  
:- p(3,Y,_), Y=mi.  
:- p(8,Y,_), Y=fr.

#show p/3.
