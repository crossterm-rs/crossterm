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

extern crate crossterm;

mod terminal;
mod cursor;
mod color;

use terminal::alternate_screen;
use terminal::raw_mode;
use terminal::terminal as term;

use self::crossterm::Context;
use self::crossterm::terminal::ClearType;
use crossterm::raw;
use crossterm::screen;
use crossterm::raw::IntoRawMode;
use std::io::Write;
use std::{time, thread};

use std::process::exit;

fn main() {
   let context = Context::new();

   use crossterm::terminal::terminal;

   let curs = terminal(context.clone());
   curs.exit();
    thread::sleep(time::Duration::from_secs(3));

}