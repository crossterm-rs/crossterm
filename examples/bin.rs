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

    terminal::get().clear(ClearType::UntilNewLine);

    cursor::get().goto(0,30);
}
