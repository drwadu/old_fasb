green_dot;blue_dot;orange_dot;yellow_dot;brown_dot;gray_dot.
green_rec;green_tri;blue_rec :- green_dot.
blue_rec :- blue_dot.
blue_tri :- blue_rec.
green_tri :- orange_dot.
orange_rec :- orange_dot.
e :- green_rec.
%
yellow_rec :- yellow_dot.
j;l :- yellow_rec.
o;p :- brown_dot.
q :- p.
r :- o.
p :- gray_dot.
o :- gray_dot.
s :- gray_dot.
u :- gray_dot.
