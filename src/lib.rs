#[macro_use]
mod shared;
mod kernel;
mod crossterm_state;

pub mod crossterm_cursor;
pub mod crossterm_style;
pub mod crossterm_terminal;

use shared::traits::{Construct};
pub use crossterm_state::{ Context};

#[cfg(windows)]
extern crate winapi;
#[cfg(unix)]
extern crate libc;
#[cfg(unix)]
extern crate termios;

extern crate rand;


// private mod
//
// public mod
//
// own usings
//
// std usings
//
// extern crate