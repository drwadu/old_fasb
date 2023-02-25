color(blue). color(green). color(red).

{frame_color(X) : color(X)} = 1.
:- frame_color(red).
{handlebars_color(X) : color(X)} = 1.
{saddle_color(X) : color(X)} = 1.

#show frame_color/1.
#show handlebars_color/1.
#show saddle_color/1.

% 2*3*3