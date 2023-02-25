paper_id(1..10).
reviewer(1..3).
bid(1,5).
bid(2,5).
bid(2,3).
bid(3,8).
bid(3,1).
%1 { r(X,Y) : reviewer(X)  } 1 :- paper_id(Y).
%0 { r(X,Y) : reviewer(X)  } 3 :- paper_id(Y).
0 { r(X,Y) : reviewer(X)  } 1 :- paper_id(Y).
a(X,Y) :- r(X,Y), bid(X,Y).
n(X,Y) :- r(X,Y), not bid(X,Y).
#show a/2.
#show n/2.

% task: find rcd wrt a/2.
%   - every bid fulfilled
%   - no overlaps
% take rcd (with r wrt a/2) as solution, in which every reviewer will review 
% at least on paper and for sure each paper they bid for