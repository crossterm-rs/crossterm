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
use self::crossterm::crossterm_style;
use self::crossterm::crossterm_cursor;
use self::crossterm::crossterm_terminal;

// Import the example modules.
pub mod color;
pub mod cursor;
pub mod terminal;

fn main() {
  
}
