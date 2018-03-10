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

use terminal::alternate_screen;
use crossterm::Context;
use std::io::{Write, stdout};

use crossterm::cursor;

fn main() {
    alternate_screen::manually_switch_to_alternate_screen();
//    cursor::cursor().goto(10,10).print("@");

}