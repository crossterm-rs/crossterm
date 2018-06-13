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
use terminal::raw_mode;

use crossterm::cursor;


fn main() {

    raw_mode::print_wait_screen_on_alternate_window();
}