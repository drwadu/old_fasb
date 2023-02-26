x(1..3).
y(1..3).
{t(X,C) : y(X)} = 1 :- x(C).
:- t(X,Y1), t(X,Y2), Y1!=Y2.

#show t/2.



% example: permutations of set with n(=3) objects with no repetition
    % we know that # x(X) = # y(Y); define n=# x(X) 
    % thus solutions correspond to symmetric group S_n over target predicate t
    % which means that in total we have n! solutions, which is expressed in the
    % following line (as n=3)
    %
    % to count under assumptions we need to find out how we can use (count) facets
    % to determine the number of solutions. in other words: which facets do we need 
    % to count and how can we use the count to determine the solution count?
    % in this case:
    %   1. we know that each facet is in t (due to #show t/2)
    %   2. t associates each x(C) with exactly one y(X) (see program line 3)
    %   3. therefore restricting t to its first parameter (t(X,_)) 
    %      and counting the resulting number m of objects ( m <- # t(X,_) ) in t(X,_)
    %      tells us how many objects m in the permutation (cycle) are not yet fixed
    %   4. the result is then the number of permutations of m objects, thus, !m

    % the last line is the so called return, which is a required expression that
    % may not contain declarations; it will be used to count under assumptions
    % as in each step m will be determined and then !m will be calculated
    % ! $ # t(X,_) would have also worked
#dasc_start
	m  <-  # t(X,_)
    ! m
#dasc_end