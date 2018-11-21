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
mod some_types;
mod terminal;

use crossterm::input::{TerminalInput, KeyEvent};

fn main() {
    println!("Press 'x' to quit...");
    TerminalInput::wait_until(KeyEvent::OnKeyPress(b'x'));
}