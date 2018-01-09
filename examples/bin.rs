extern crate crossterm;

use self::crossterm::terminal_style::*;
use self::crossterm::terminal_cursor::*;
use self::crossterm::terminal::*;
use std::io::{stdin, stdout, Write};

fn main() {
    terminal::get().clear(ClearType::All);

    for y in 0..21 {
        for x in 0..21 {
            if (x == 0 || y == 0) || (x == 20 || y == 20) {
                print!("{}", paint("■").with(Color::Red));
            } else {
                print!("{}", paint(" ").with(Color::Blue).on(Color::Blue));
            }
        }
        println!();
    }

    let mut curs = cursor::get();
    {
        curs.goto(4, 1).print("@");

    }

    let mut terminal = terminal::get();

    // clear all cells in terminal.
    terminal.clear(ClearType::All);
    // clear all cells after the cursor position in terminal.
    terminal.clear(ClearType::AfterCursor);
    // clear all cells before cursor in terminal.
    terminal.clear(ClearType::BeforeCursor);
    // clear current line cells in terminal.
    terminal.clear(ClearType::CurrentLine);
    // clear all cells until new line in terminal.
    terminal.clear(ClearType::UntilNewLine);

    let size = terminal.terminal_size();
    println!("{:?}", size);

    // scrolling in terminal
    terminal.scroll_up(1);
    terminal.scroll_down();

    cursor::get().goto(0,30);
}
