row(1..9). col(1..9). n(1..9).

{set(R,C,N) : n(N)} = 1 :- row(R), col(C).

sub(R1,C1,R2,C2) :- 
	row(R1), row(R2), 
	(R1-1)/3 == (R2-1)/3,
	col(C1), col(C2),
	(C1-1)/3 == (C2-1)/3.

:- set(R1,C,N), set(R2,C,N), R1 != R2.
:- set(R,C1,N), set(R,C2,N), C1 != C2.
:- set(R1,C1,N), set(R2,C2,N), 
	sub(R1,C1,R2,C2),
	R1 != R2, C1 != C2.

set(1,2,2). set(1,4,5). set(1,6,1). set(1,8,9). 
set(2,1,8). set(2,4,2). set(2,6,3). set(2,9,6).
set(3,2,3). set(3,5,6). set(3,8,7). 

% a,b: 8 solutions
% a',b': 1 solution
set(4,3,1).               % a'
% set(4,3,1). set(4,7,6). % a
set(5,1,5). set(5,2,4). set(5,8,1). set(5,9,9).
% set(6,3,2). set(6,7,7). % b
set(6,7,7).               % b'

set(7,2,9). set(7,5,3). set(7,8,8). 
set(8,1,2). set(8,4,8). set(8,6,4). set(8,9,7).
set(9,2,1). set(9,4,9). set(9,6,7). set(9,8,6). 

#show set/3.