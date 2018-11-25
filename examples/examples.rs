//! This bin folder can be used to try the examples out located in the examples directory.
//!
//! All you need to do is:
//!
//! - Download the crossterm source code.
//! - Run program with: `cargo run --example examples`

extern crate crossterm;

// modules that could be test
//mod color;
//mod cursor;
//mod input;
//mod some_types;
//mod terminal;

use crossterm::style::{style, Color, Attribute};

fn main() {
    let styled_object = style("'Red' text on 'White' background")
        .with(Color::AnsiValue(9))
        .on(Color::AnsiValue(15));

    println!("{}", styled_object);
}