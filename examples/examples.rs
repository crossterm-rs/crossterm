//! This bin folder can be used to try the examples out located in the examples directory.
//!
//! All you need to do is:
//!
//! - Download the crossterm source code.
//! - Run program with: `cargo run --example examples`

extern crate crossterm;

// modules that could be test
mod color;
mod cursor;
mod input;
//mod some_types;
mod terminal;

fn main() {
    let cursor = crossterm::cursor();
    cursor.goto(5, 5);

    let integer = 10;
    let float: f32 = integert as f32;

    println!("5.515151");
}
