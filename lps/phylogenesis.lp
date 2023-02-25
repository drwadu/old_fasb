#const n=17.
#const k=7.


%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
% Package Name			:	OnlineMethod3_v1
% Package Info			:	Computing n k-similar/diverse phylogenies by representing
%							the problem in ASP. Online Method 3 mentioned in [1]
% Authors				:	Esra Erdem [http://people.sabanciuniv.edu/esraerdem] 
%							Halit Erdogan [http://students.sabanciuniv.edu/halit]
% Version				:	1.00
% File Name				:	Indo-European
% Functions				:	Input file for Indo-European language phylogenies.
% Notes					:	- See claspnk2.pdf for this package.
%
%
% RELATED PUBLICATIONS
% --------------------		
% [1] T. Eiter, E. Erdem, H. Erdogan, and M.Fink. Finding Similar or Diverse Solutions in Answer Set Programming. 
%	  In Proc. of ICLP'09, pp. 342--356. 
% 
% Indo-European languages
%    dataset, assembled by Don Ringe and Ann Taylor, with the advice
%    of other specialist colleagues, is described in 
%       "Indo-European and computational cladistics," 
%       (Transactions of the Philological Society, 100:1:59-129,2002)
%    by Donald Ringe and Tandy Warnow and Ann Taylor .
%
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

% 7 language groups and Albanian



leaf(7).  leaf(6).  leaf(5).  leaf(4).  
leaf(3).  leaf(2).  leaf(1).  leaf(0).  

% proto-Italo-Celtic

f(7,340,2).  f(7,280,2).  f(7,299,14).  f(7,235,5).  f(7,202,9).
f(7,161,11).  f(7,100,1).  f(7,146,7).  f(7,147,7).  f(7,133,1).
f(7,107,2).  f(7,73,3).  f(7,39,6).  f(7,59,7).  f(7,93,7).  f(7,2,1).
f(7,3,1).  f(7,27,1).  

% proto-Anatolian

f(6,235,1).  f(6,133,1).  f(6,107,1).  f(6,73,1).  f(6,39,8).
f(6,2,1).  f(6,3,1).  f(6,27,1).  

% proto-Tocharian

f(5,328,5).  f(5,340,11).  f(5,280,4).  f(5,297,3).  f(5,299,3).
f(5,230,5).  f(5,161,2).  f(5,146,5).  f(5,147,5).  f(5,107,1).
f(5,73,1).  f(5,39,3).  f(5,59,2).  f(5,2,1).  f(5,3,1).  f(5,27,1).  

% Albanian

f(4,328,4).  f(4,340,3).  f(4,280,3).  f(4,297,4).  f(4,299,4).
f(4,235,1).  f(4,230,4).  f(4,202,4).  f(4,161,2).  f(4,100,6).
f(4,146,4).  f(4,147,4).  f(4,133,4).  f(4,107,3).  f(4,73,3).
f(4,39,4).  f(4,59,4).  f(4,93,3).  f(4,2,1).  f(4,3,1).  f(4,27,4).  

% proto-Germanic

f(3,328,6).  f(3,340,2).  f(3,280,3).  f(3,297,14).  f(3,299,14).
f(3,235,5).  f(3,230,8).  f(3,202,9).  f(3,161,11).  f(3,100,1).
f(3,146,7).  f(3,147,7).  f(3,133,7).  f(3,107,4).  f(3,73,6).
f(3,39,6).  f(3,59,7).  f(3,93,7).  f(3,2,1).  f(3,3,1).  f(3,27,2).  

% proto-Greco-Armenian

f(2,328,3).  f(2,340,2).  f(2,280,2).  f(2,297,3).  f(2,299,3).
f(2,235,1).  f(2,202,3).  f(2,161,2).  f(2,100,5).  f(2,107,2).
f(2,73,3).  f(2,39,3).  f(2,59,2).  f(2,93,2).  f(2,2,1).  f(2,3,1).
f(2,27,2).  

% proto-Balto-Slavic

f(1,328,6).  f(1,340,11).  f(1,280,2).  f(1,297,14).  f(1,299,6).
f(1,235,5).  f(1,230,8).  f(1,161,11).  f(1,100,5).  f(1,146,6).
f(1,147,6).  f(1,133,8).  f(1,107,2).  f(1,73,3).  f(1,39,5).
f(1,93,2).  f(1,2,2).  f(1,3,2).  

% proto-Indo-Iranian

f(0,328,3).  f(0,340,11).  f(0,280,2).  f(0,297,3).  f(0,299,5).
f(0,235,1).  f(0,230,5).  f(0,202,3).  f(0,146,6).  f(0,147,6).
f(0,133,7).  f(0,107,2).  f(0,73,3).  f(0,39,5).  f(0,59,2).
f(0,93,5).  f(0,2,2).  f(0,3,2).  f(0,27,2).  

% characters and states

character(I) :- f(X,I,S).
state(S) :- f(X,I,S).

% informative characters

informative_character(39).  informative_character(235).
informative_character(100).  informative_character(280).
informative_character(73).  informative_character(328).
informative_character(27).  informative_character(3).
informative_character(2).  informative_character(107).
informative_character(340).  informative_character(161).
informative_character(297).  informative_character(299).
informative_character(230).  informative_character(202).
informative_character(147).  informative_character(146).
informative_character(93).  informative_character(59).
informative_character(133).  

% essential states

essential_state(39,5).  essential_state(100,5).
essential_state(235,5).  essential_state(230,5).
essential_state(73,3).  essential_state(328,3).
essential_state(297,3).  essential_state(202,3).
essential_state(280,3).  essential_state(299,3).
essential_state(39,3).  essential_state(146,6).
essential_state(147,6).  essential_state(328,6).
essential_state(39,6).  essential_state(280,2).
essential_state(107,2).  essential_state(2,2).  essential_state(3,2).
essential_state(59,2).  essential_state(27,2).  essential_state(93,2).
essential_state(340,2).  essential_state(161,2).
essential_state(340,11).  essential_state(161,11).
essential_state(297,14).  essential_state(299,14).
essential_state(235,1).  essential_state(2,1).  essential_state(3,1).
essential_state(107,1).  essential_state(73,1).
essential_state(27,1).  essential_state(100,1).
essential_state(133,1).  essential_state(230,8).
essential_state(202,9).  essential_state(133,7).
essential_state(146,7).  essential_state(147,7).
essential_state(59,7).  essential_state(93,7).





% domain specific information

:- not edge(14,6).
:- not edge(14,13).
:- not edge(13,5).

% AL cannot be a sister of IIR, BS, Anatolian
sister(X,Y) :- edge(Z,X), edge(Z,Y), X<Y, vertex(X), vertex(Y), vertex(Z).
:- sister(0,4).
:- sister(1,4).

subtree(1) :- edge(12,4), edge(12,11), edge(11,7).
subtree(2) :- edge(12,7), edge(12,11).
subtree(3) :- vertex(V), edge(12,V), edge(V,7), edge(V,4).

:- 2 {subtree(I):I=1..3}.
:-  {subtree(I):I=1..3} 0.

:- incompatible(27).

% redundant constraints
:- not edge(13,12).
:- not edge(12,11).


%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
% Package Name			:	OnlineMethod3_v1
% Package Info			:	Computing n k-similar/diverse phylogenies by representing
%							the problem in ASP. Online Method 3 mentioned in [1]
% Authors				:	Esra Erdem [http://people.sabanciuniv.edu/esraerdem] 
%							Halit Erdogan [http://students.sabanciuniv.edu/halit]
% Version				:	1.00
% File Name				:	phylogeny-improved.lp
% Functions				:	ASP representation of phylogeny reconstruction problem, see [2] for details.
% Notes					:	- See claspnk2.pdf for this package.
%
%
% RELATED PUBLICATIONS
% --------------------		
% [1] T. Eiter, E. Erdem, H. Erdogan, and M.Fink. Finding Similar or Diverse Solutions in Answer Set Programming. 
%	  In Proc. of ICLP'09, pp. 342--356. 
% [2] D. Brooks, E. Erdem, S. Erdogan, J. Minett, and D. Ringe. Inferring phylogenetic trees using answer set 
%     programming. In Journal of Automated Reasoning (JAR) , Vol. 39(4), pp. 471-511, 2007.
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%





vertex(0..2*k).

internal(X) :- vertex(X), not leaf(X).

2 {edge(X,Y) : vertex(Y) , X > Y} 2 :- internal(X).

root(2*k).	

reachable(X,Y) :- edge(X,Y), vertex(X), vertex(Y), X > Y.
reachable(X,Y) :- edge(X,Z), reachable(Z,Y), X > Z, vertex(X), vertex(Y), vertex(Z).

:- vertex(Y), not reachable(X,Y), root(X), Y != X.

:- reachable(X,X), vertex(X).

maxY(X,Y) :- edge(X,Y), edge(X,Y1), Y > Y1, vertex(Y1), vertex(Y), internal(X).
:- maxY(X,Y), maxY(X1,Y1), Y > Y1, X < X1, vertex(Y1), vertex(Y), internal(X), internal(X1).





%%%%%%%%%%%%%%% B6
% maximum compatibility

g0(X,I,S) :- f(X,I,S), informative_character(I), essential_state(I,S).
g0(Y,I,S) :- g0(X,I,S), g0(X1,I,S), edge(Y,X), edge(Y,X1), 
X>X1, internal(Y), vertex(X1), vertex(X), informative_character(I), 
essential_state(I,S).

marked(X,I) :- g0(X,I,S), informative_character(I), 
vertex(X), essential_state(I,S).

g(X,I,S) :- g0(X,I,S), informative_character(I),
vertex(X), essential_state(I,S).

{g(X,I,S): essential_state(I,S)} 1 :- internal(X), 
not marked(X,I), informative_character(I).

{root_is(X,I,S)} :- g(X,I,S), vertex(X), informative_character(I), 
essential_state(I,S).

:- root_is(X,I,S), root_is(Y,I,S),
vertex(X), vertex(Y), X < Y, informative_character(I), essential_state(I,S).

% we need to consider every antecedent of x below
:- root_is(X,I,S), g(Y,I,S), reachable(Y,X), Y > X,
vertex(X), vertex(Y), informative_character(I), essential_state(I,S).
														  
reachable_is(X,I,S) :- root_is(X,I,S), 
vertex(X), informative_character(I), essential_state(I,S).

reachable_is(X,I,S) :- g(X,I,S), reachable_is(Z,I,S), 
edge(Z,X), Z > X, vertex(X), vertex(Z), informative_character(I), essential_state(I,S). 

incompatible(I) :- g(X,I,S), not reachable_is(X,I,S), 
vertex(X), informative_character(I), essential_state(I,S).

:- n+1 {incompatible(I) : informative_character(I)}.

#show edge/2.