
//! This bin folder can be used to try the examples out located in the examples directory.
//!
//! All you need to do is:
//!
//! - Download the crossterm source code.
//! - Run program with: `cargo run --example examples`

extern crate crossterm;

// modules that could be test
mod terminal;
mod color;
mod cursor;
mod some_types;
mod input;

use crossterm::{Screen, Crossterm};
use std::{time, thread};
use crossterm::cursor::cursor;

fn main() {

    thread::sleep(time::Duration::from_millis(2000));

}
