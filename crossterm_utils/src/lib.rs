#[cfg(windows)]
extern crate crossterm_winapi;
#[cfg(unix)]
extern crate libc;
#[cfg(windows)]
extern crate winapi;

mod command;
pub mod error;
mod functions;
pub mod macros;
pub mod sys;

pub use self::command::{Command, Output, QueueableCommand, ExecutableCommand};

pub use self::error::{ErrorKind, Result};
#[cfg(windows)]
pub use self::functions::supports_ansi;
