use ncurses::*;

const START: [&'static str; 3] = [
    "basic vim support (:w, :q, normal mode, insert mode)",
    "type i\t to edit text",
    "type ESC+:wq\t to write and exit",
];

pub(crate) const ESC: i32 = 27;
pub(crate) const DELETE_ITEM: i32 = 100;
pub(crate) const EXIT: i32 = 101;
pub(crate) const VIM_DOWN: i32 = 106;
pub(crate) const VIM_UP: i32 = 107;
pub(crate) const QUIT: i32 = 113;

pub(crate) fn startup() {
    initscr();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    start_color();
}

pub(crate) fn start_screen() {
    let (mut x, mut y) = (0, 0);
    let w = stdscr();
    getmaxyx(w, &mut y, &mut x);
    START.iter().enumerate().for_each(|(i, s)| {
        let j = s.len() as i32;
        mvprintw((y / 2) + i as i32, (x / 2) - (j / 2), s);
    })
}

pub(crate) fn display_command(c: i32, i: i32) {
    let w = stdscr();
    let y = getmaxy(w);
    mvprintw(y - 1 as i32, 0 + i, &format!("{}", c as u8 as char));
}
