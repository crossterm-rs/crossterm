
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
use crossterm::terminal::terminal;
use crossterm::Screen;

use crossterm::output::TerminalOutput;
use crossterm::cursor::TerminalCursor;

use crossterm::terminal::Terminal;
use std::{thread,time};

fn main()
{
    let screen = Screen::new(false);
    let terminal = Terminal::new(&screen.stdout);

    // get terminal size
    let (x, y) = terminal.terminal_size();

    // set size to 30, 50
    terminal.set_size(30,50);

    // if we uncomment the line below the code will work perfectly fine and we will get the new dimensions.
    // if we comment this line the terminal dimensions gotten from terminal_size() are equal to the old dimensions.

    // thread::sleep(time::Duration::from_millis(20));

    // get new dimensions
    let (x_new, y_new) = terminal.terminal_size();

    println!("old width: {} old height: {}", x, y);
    println!("new width: {} new height: {}", x_new, y_new);
}
