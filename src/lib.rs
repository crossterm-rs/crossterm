//! Crossterm provides the same core functionalities for both windows and unix systems.
//! Crossterm aims to be simple and easy to call in code.
//! True the simplicity of Crossterm you do not have to worry about the platform your working with.
//! You can just call the action you want to perform and under water it will check what to do based on the current platform.

#[macro_use]
mod common;

mod kernel;
mod modules;

pub use modules::cursor;
pub use modules::input;
pub use modules::output;
pub use modules::style;
pub use modules::terminal;

pub use common::screen::{Screen, AlternateScreen};
pub use common::Crossterm;
pub use output::TerminalOutput;
pub use self::cursor::*;
pub use self::input::*;
pub use self::output::*;
pub use self::style::*;

#[cfg(unix)]
extern crate libc;
#[cfg(unix)]
extern crate termios;

#[cfg(windows)]
extern crate winapi;
