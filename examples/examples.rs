
//! This bin folder can be used to try the examples out located in the examples directory.
//!
//! All you need to do is:
//!
//! - Download the crossterm source code.
//! - Run program with: `cargo run --example examples`

extern crate crossterm;

// modules that could be test
//mod terminal;
mod color;
mod cursor;
mod some_types;
mod input;

use std::io::Write;
use std::{thread,time};

use crossterm::style::{style, Color, DisplayableObject};
use crossterm::terminal::terminal;
use crossterm::Screen;

use crossterm::output::TerminalOutput;
use crossterm::cursor::TerminalCursor;

fn main()
{
    let screen = Screen::default();
    let cursor = TerminalCursor::new(&screen.stdout);

    cursor.goto(5, 5);
    let (x, y) = cursor.pos();

    assert_eq!(x, 5);
    assert_eq!(y, 5);

    println!("x: {} y: {}", x,y);
}
