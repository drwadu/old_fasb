card(1..50).
player(1..3).
0 {hint(X,Y) : player(X)} 1 :- player(Y).
:- hint(X,X).
0 {trash(X,Y): player(X), card(Y)} 1.
0 {play(X,Y) : player(X), card(Y)} 1.
:- hint(X,_), trash(X,_), play(X,_).
:- hint(X,_), trash(X,_).
:- play(X,_), trash(X,_).
:- play(X,_), hint(X,_).
0{in(X) : card(X)}25.
1{blue;red;green;yellow;white;one;two;three;four;five}1.
%1{blue;red;green;yellow;white;one;two;three;four;five}1.
#show in/1.
#show hint/2.
#show blue/1.
