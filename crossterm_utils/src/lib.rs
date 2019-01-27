#[cfg(windows)]
extern crate crossterm_winapi;
#[cfg(windows)]
extern crate winapi;

#[cfg(unix)]
extern crate termios;

pub mod commands;
pub mod error;
pub mod macros;
pub mod sys;

mod functions;
mod output;

pub use self::error::{ErrorKind, Result};
pub use self::output::TerminalOutput;

#[cfg(windows)]
pub use self::functions::get_module;
pub use self::functions::{write, write_str};
