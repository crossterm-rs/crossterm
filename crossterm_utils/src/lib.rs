#[cfg(windows)]
extern crate crossterm_winapi;
#[cfg(windows)]
extern crate winapi;

pub mod error;
pub mod macros;
pub mod sys;

mod functions;
mod output;

pub use self::error::{ErrorKind, Result};
pub use self::output::TerminalOutput;

#[cfg(windows)]
pub use self::functions::supports_ansi;
