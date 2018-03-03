//! This bin folder can be used to try the examples out located in the examples directory.
//!
//! All you need to do is:
//! 
//! - Download the crossterm source code.
//! - Add this in the Cargo.toml file:
//!   ``` [[bin]]
//!        name = "example_bin"
//!        path = "./examples/bin.rs"
//!   ```
//!   
//! - Run program with: `cargo run`

// Import crossterm crate.
extern crate crossterm;

// Add the usings for the crossterms modules to play with crossterm
use self::crossterm::crossterm_style::{paint, Color };
use self::crossterm::crossterm_cursor;
use self::crossterm::crossterm_terminal;

// Import the example modules.
pub mod color;
pub mod cursor;
pub mod terminal;
use std::io::{Error, ErrorKind, Write};
use std::io;
use std::{time, thread};

use self::crossterm_terminal::screen::AlternateScreen;
use crossterm::crossterm_terminal::IntoRawMode;

use crossterm::Context;

fn main() {
    let mut context = Context::new();
//
    let mut screen = io::stdout().into_raw_mode(&mut context).unwrap();
    {
//        let mut screen = io::stdout();
        crossterm_cursor::cursor().goto(10, 10);

        let mut curs = crossterm::crossterm_cursor::cursor();
        curs.move_up(1);
//        print!("1");
        write!(screen, "{}", "1");
        curs.move_right(1);
//        print!("2");
        write!(screen, "{}", "2");
        curs.move_down(1);
//        print!("3");
        write!(screen, "{}", "3");
        curs.move_left(1);
//        write!()!("4");
        write!(screen, "{}", "4");
    }
}