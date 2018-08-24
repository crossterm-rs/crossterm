
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

use std::io::Write;
use std::{thread,time};
fn main()
{
   ::crossterm::terminal::terminal(&::crossterm::Screen::default()).terminal_size();
    thread::sleep(time::Duration::from_millis(2000));
}
