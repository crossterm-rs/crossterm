
//! This bin folder can be used to try the examples out located in the examples directory.
//!
//! All you need to do is:
//!
//! - Download the crossterm source code.
//! - Run program with: `cargo run --example examples`

extern crate crossterm;

// modules that could be test
//mod terminal;
mod color;
mod cursor;
mod some_types;
mod input;

use std::io::Write;
use std::{thread,time};

use crossterm::style::{style, Color, DisplayableObject};
use crossterm::terminal::terminal;
use crossterm::Screen;

fn main()
{
    let screen = Screen::default();

    println!("\nExample:\n\n\taws --profile {} s3 ls\n", DisplayableObject::new(&screen, &style("test").with(Color::Yellow)));
}
