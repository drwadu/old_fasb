%*
#const n=3.
n { o(1..n,1..n) } n.
:- not win.
win :- I=1..n, o(I,J) : J=1..n.
win :- J=1..n, o(I,J) : I=1..n.
win :- o(I,I) : I=1..n.
win :- o(I,n+1-I) : I=1..n.

#show o/2.
*%

#const n=3.

%n { o(1..n,1..n) } n.
%n { x(1..n,1..n) } n.
n { o(1..n,1..n) } n+2.
n { x(1..n,1..n) } n+2.
%{ o(1..n,1..n) } = n.
%{ x(1..n,1..n) } = n.

wino :- I=1..n, o(I,J) : J=1..n.
wino :- J=1..n, o(I,J) : I=1..n.
wino :- o(I,I) : I=1..n.
wino :- o(I,n+1-I) : I=1..n.

winx :- I=1..n, x(I,J) : J=1..n.
winx :- J=1..n, x(I,J) : I=1..n.
winx :- x(I,I) : I=1..n.
winx :- x(I,n+1-I) : I=1..n.

:- o(I,J), x(I,J).
draw :- not wino, not winx.
:- wino, winx.
%wino :- not winx, not draw.
%winx :- not wino, not draw.

%#show o/2.
%#show x/2.
%#show wino/0.
%#show winx/0.
%#show draw/0.
