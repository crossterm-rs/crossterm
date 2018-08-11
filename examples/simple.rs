
//! This bin folder can be used to try the examples out located in the examples directory.
//!
//! All you need to do is:
//!
//! - Download the crossterm source code.
//!
//! - Add this in the Cargo.toml file:
//!   ``` [[bin]]
//!        name = "example_bin"
//!        path = "./examples/bin.rs"
//!   ```
//!   
//! - Run program with: `cargo run`
extern crate crossterm;

use crossterm::style::Color;
use crossterm::Crossterm;
use crossterm::write::Stdout;
use crossterm::common::screen::Screen;
use std::io::Write;
// mod terminal;
// mod color;
// mod cursor;
// mod crossterm_type;
 mod input;

//use input::keyboard::{async_input, input as stdin};

use std::{thread, time, };
use std::sync::mpsc;

fn main() {
   input::keyboard::async_input::async_reading_on_alternate_screen();
}
