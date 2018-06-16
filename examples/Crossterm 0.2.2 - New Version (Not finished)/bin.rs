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

fn main() {
//   alternate_screen::switch_between_main_and_alternate_screen();
   let context = Context::new();
   raw_mode::print_wait_screen_on_alternate_window();
}