
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

use crossterm::style::{style, Color, DisplayableObject};
use crossterm::terminal::{terminal, ClearType};
use crossterm::Screen;

use crossterm::output::TerminalOutput;
use crossterm::cursor::{TerminalCursor, cursor};

use crossterm::terminal::Terminal;
use std::{thread,time};

fn main()
{
    let mut screen = Screen::default();
    terminal(&screen).clear(ClearType::All);
    cursor(&screen).goto(0, 0);
    screen.write_buf(b"https://www.google.com").expect("");
    screen.flush_buf().expect("");
    cursor(&screen).goto(4, 0);
    screen.write_buf(b"FFF").expect("");
    screen.flush_buf().expect("");
}
