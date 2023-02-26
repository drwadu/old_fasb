row(1..4). col(1..4). n(1..4).

{set(R,C,N) : n(N)} = 1 :- row(R), col(C).
:- set(R,C,N), set(R',C,N), R!=R'.
:- set(R,C,N), set(R,C',N), C!=C'.
:- set(R,C,N), set(R',C',N), 
   (R-1)/2 == (R'-1)/2,
   (C-1)/2 == (C'-1)/2,
   sub(R,C,R',C'), R!= R', C!=C'.

#show set/3.

#dasc_start 
	% list comprehension 
	ns <- [# set(ROW,COL,_)] 
	* ns
#dasc_end