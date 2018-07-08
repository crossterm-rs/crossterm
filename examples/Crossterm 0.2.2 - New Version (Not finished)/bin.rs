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

mod color;
mod cursor;
mod program_examples;
mod terminal;

use crossterm::Context;
use program_examples::first_depth_search;

fn main() {
    first_depth_search::run();
//    println!("End")
}
