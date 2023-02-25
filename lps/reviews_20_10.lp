paper_id(1..20).
reviewer(1..10).
bid(1,1).
bid(1,4).
bid(1,8).
bid(2,17).
bid(2,7).
bid(3,9).
bid(3,19).
bid(3,20).
bid(4,14).
bid(4,1).
bid(4,18).
bid(5,15).
bid(5,11).
bid(5,12).
bid(6,2).
bid(6,19).
bid(7,9).
bid(7,13).
bid(7,17).
bid(8,15).
bid(8,2).
bid(9,4).
bid(10,6).
bid(10,18).
bid(10,1).
0 { n(X,Y) : reviewer(X) } 1 :- not bid(_,Y), paper_id(Y).
has_review(X) :- n(_,X).
:- not has_review(X), not bid(_,X), paper_id(X).
#show n/2.
#show has_review/1.


%< > [ 0% ] ~> :kg
%
%solving...
%
%Answer 1: 
%n(6,3) n(8,5) n(8,10) n(3,16) has_review(3) has_review(5) has_review(10) has_review(16) 
%Answer 2: 
%n(4,3) n(4,5) n(10,10) n(6,16) has_review(3) has_review(5) has_review(10) has_review(16) 
%Answer 3: 
%n(2,3) n(3,5) n(2,10) n(2,16) has_review(3) has_review(5) has_review(10) has_review(16) 
%Answer 4: 
%n(3,3) n(9,5) n(7,10) n(4,16) has_review(3) has_review(5) has_review(10) has_review(16) 
%Answer 5: 
%n(9,3) n(2,5) n(4,10) n(1,16) has_review(3) has_review(5) has_review(10) has_review(16) 
%Answer 6: 
%n(5,3) n(1,5) n(6,10) n(8,16) has_review(3) has_review(5) has_review(10) has_review(16) 
%Answer 7: 
%n(7,3) n(7,5) n(5,10) n(9,16) has_review(3) has_review(5) has_review(10) has_review(16) 
%Answer 8: 
%n(8,3) n(5,5) n(1,10) n(7,16) has_review(3) has_review(5) has_review(10) has_review(16) 
%Answer 9: 
%n(1,3) n(6,5) n(3,10) n(10,16) has_review(3) has_review(5) has_review(10) has_review(16) 
%Answer 10: 
%n(10,3) n(10,5) n(9,10) n(5,16) has_review(3) has_review(5) has_review(10) has_review(16) 
%
%call            : --k-greedy-search
%elapsed         : 11.761612ms
%
%< > [ 0% ] ~> ?fc
%
%80
%
%< > [ 0% ] ~> 
