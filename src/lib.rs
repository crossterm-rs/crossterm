#[macro_use]
pub mod shared;
pub mod terminal_cursor;
pub mod terminal_style;
pub mod terminal;
mod kernel;

use shared::traits::{Construct, Empty};
pub use terminal_cursor::cursor;
pub use terminal_style::paint;

#[cfg(windows)]
extern crate kernel32;
#[cfg(windows)]
extern crate winapi;
