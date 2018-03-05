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
//
//// Add the usings for the crossterms modules to play with crossterm
//use self::crossterm::crossterm_style::{paint, Color };
use self::crossterm::crossterm_cursor::cursor;
use self::crossterm::crossterm_terminal;
//
//// Import the example modules.
//pub mod color;
//pub mod cursor;
//pub mod terminal;
use std::io::{self, Error, ErrorKind, Write, stdout, stdin, BufRead};

//use std::{time, thread};
//
use crossterm::crossterm_terminal::screen::{AlternateScreen, ToMainScreen, ToAlternateScreen};
use crossterm::crossterm_terminal::IntoRawMode;

use crossterm::Context;


use std::{time, thread};

fn main() {
    let mut context = Context::new();
    let mut screen = stdout();
    write!(screen, "{}", ToAlternateScreen);
    write!(screen, "Welcome to the alternate screen.\n\nPlease wait patiently until we arrive back at the main screen in a about three seconds.").unwrap();
    //screen.flush().unwrap();

    thread::sleep(time::Duration::from_secs(3));
}