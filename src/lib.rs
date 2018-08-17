//! Crossterm provides the same core functionalities for both windows and unix systems.
//! Crossterm aims to be simple and easy to call in code.
//! True the simplicity of Crossterm you do not have to worry about the platform your working with.
//! You can just call the action you want to perform and under water it will check what to do based on the current platform.

#[macro_use]
mod common;

mod kernel;
mod modules;

mod cursor;
mod input;
mod terminal_output;
mod crossterm;
mod color;

pub use self::cursor::*;
pub use self::input::*;
pub use self::terminal_output::*;
pub use self::crossterm::*;
pub use self::color::*;

#[cfg(unix)]
extern crate libc;
#[cfg(unix)]
extern crate termios;

#[cfg(windows)]
extern crate winapi;
