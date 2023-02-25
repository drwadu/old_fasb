cell(1..9).
obj(1..9).
{set_obj_cell(X,C) : obj(X)} = 1 :- cell(C).
:- set_obj_cell(X,C1), set_obj_cell(X,C2), C1!=C2.

#show set_obj_cell/2.
