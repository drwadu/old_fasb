% author: dominik.rusovac@tu-dresden.de
% generated on 2023-02-06
#const n_tests=6.
#const n_modules=3.

test(0,0). test(0,1). 
test(1,2). test(1,3). 
test(2,4). test(2,5). 

1 { run_module(N) : N=0..n_modules-1 }.
test_runs(N) :- run_module(M), test(M,N).
n_tests_running(N) :- N = #count { 1:test_runs(M) }.
1 { run_test(T,N) : T=0..K } 1 :- n_tests_running(K), test_runs(N).

:- run_module(0), run_module(1).
:- run_test(T,1), run_test(T',2), T>=T'.
:- run_test(T,2), run_test(T,4).

#show run_module/1.
#show run_test/2.
