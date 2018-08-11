//! Crossterm provides the same core functionalities for both windows and unix systems.
//! Crossterm aims to be simple and easy to call in code.
//! True the simplicity of Crossterm you do not have to worry about the platform your working with.
//! You can just call the action you want to perform and under water it will check what to do based on the current platform.

#[macro_use]
pub mod common;

mod kernel;
mod modules;

pub use common::screen;
pub use modules::cursor;
pub use modules::input;
pub use modules::write;
pub use modules::style;
 pub use modules::terminal;

pub use common::Crossterm;
pub use write::{IStdout, Stdout};
pub use common::screen::Screen;
#[cfg(unix)]
extern crate libc;
#[cfg(unix)]
extern crate termios;

#[cfg(windows)]
extern crate winapi;
