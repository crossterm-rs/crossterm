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

use crossterm::Terminal;

// mod terminal;
// mod color;
// mod cursor;
// mod crossterm_type;
// mod input;

//use input::keyboard::{async_input, input as stdin};

use std::{thread, time};

fn main() {
}
