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
mod terminal;
mod crossterm_type;

fn main() {
    use crossterm_type::create_alternate_screen_from_crossterm;
    create_alternate_screen_from_crossterm();
}
