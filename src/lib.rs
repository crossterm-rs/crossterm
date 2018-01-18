#[macro_use]
mod shared;
mod kernel;
pub mod crossterm_cursor;
pub mod crossterm_style;
pub mod crossterm_terminal;

use shared::traits::{Construct};

#[cfg(windows)]
extern crate winapi;
#[cfg(unix)]
extern crate libc;