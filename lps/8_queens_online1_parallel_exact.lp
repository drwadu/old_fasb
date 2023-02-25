{ num(1..8) } = 1.
solution(1..N) :- num(N).

{q(S,I ,1..8)} == 1 :- I = 1..8, solution(S).
{q(S,1..8, J)} == 1 :- J = 1..8, solution(S).
:- {q(S,D-J, J)} >= 2, D = 2..2*8, solution(S).
:- {q(S,D+J, J)} >= 2, D = 1-8..8-1, solution(S).

:- q(S1,X,Y), q(S2,X,Y), S1!=S2, solution(S1), solution(S2).
#maximize { N : num(N) }. 
