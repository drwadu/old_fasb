x(1..3). y(1..3).
{ t(Y,X) : y(Y) } = 1 :- x(X).
:- t(Y,X1), t(Y,X2), X1!=X2.

#show t/2.

