
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


fn main() {
  // call some test module function

// terminal::terminal::resize_terminal();
 input::keyboard::async_input::read_async_demo();
// use crossterm::screen::RawScreen;
// RawScreen::into_raw_mode();
// RawScreen::disable_raw_modes();
}
