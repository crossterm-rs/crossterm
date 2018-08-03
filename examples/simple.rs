
//! This bin folder can be used to try the examples out located in the examples directory.
//!
//! All you need to do is:
//!
//! - Download the crossterm source code.
//!
//! - Add this in the Cargo.toml file:
//!   ``` [[bin]]
//!        name = "example_bin"
//!        path = "./examples/bin.rs"
//!   ```
//!   
//! - Run program with: `cargo run`
extern crate crossterm;

use crossterm::style::Color;
use crossterm::Crossterm;

// mod terminal;
// mod color;
// mod cursor;
// mod crossterm_type;
// mod input;

//use input::keyboard::{async_input, input as stdin};

use std::{thread, time};

fn main() {
    do_something();
}

fn do_something()
{
    let mut crossterm = Crossterm::new();

    {
        let mut cursor = crossterm.cursor(); // <- Immutable borrow occurs here ( cursor(&self) ) end lives until the end of this function call.
        cursor.goto(10, 10);
    }
    crossterm.to_alternate_screen(); // <- mutable borrow occurs here  ( to_alternate_screen(&mut self) ) but because we already have borrowed immutable we can not mutate it.
}



