paper_id(1..100).
reviewer(1..10).
bid(1,6).
bid(1,86).
bid(2,46).
bid(2,4).
bid(2,53).
bid(3,84).
bid(4,35).
bid(4,22).
bid(4,19).
bid(5,45).
bid(5,80).
bid(5,79).
bid(6,48).
bid(7,67).
bid(7,25).
bid(8,1).
bid(8,91).
bid(9,81).
bid(9,25).
bid(10,83).
bid(10,31).
0 { n(X,Y) : reviewer(X) } 1 :- not bid(_,Y), paper_id(Y).
has_review(X) :- n(_,X).
:- not has_review(X), not bid(_,X), paper_id(X).
#show n/2.
#show has_review/1.
% rcd wrt has_review/1