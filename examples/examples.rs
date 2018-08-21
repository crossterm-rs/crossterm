
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

fn main()
{
//    use crossterm::screen::RawScreen;
//    use crossterm::Screen;
//
//    let mut screen = Screen::new(true);
//
//    write!(screen, "text \n\r");
//    let a = screen.enable_alternate_modes(true).unwrap();
//
//    write!(a, "text \n\r");


}
