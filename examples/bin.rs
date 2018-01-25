extern crate crossterm;

use self::crossterm::crossterm_style::*;
use self::crossterm::crossterm_cursor;
use self::crossterm::crossterm_terminal::*;

use std::io::{stdin, stdout, Write};

pub mod color;
pub mod cursor;
pub mod terminal;

fn main() {
    terminal::resize_terminal();

    print!(paint("asdf").with(Color::Black).on(Color::Red).black().red());
    println!()
}
