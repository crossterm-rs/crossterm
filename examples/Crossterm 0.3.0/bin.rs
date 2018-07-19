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

use crossterm::Context;


// mod terminal;
// mod color;
// mod cursor;
// mod crossterm_type;

fn main()
{
    let context = Context::new();
    let input = ::crossterm::input::input(&context);
    let line = input.read_line().unwrap();

    println!("input: {}",line);
}


