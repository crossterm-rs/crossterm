#[cfg(windows)]
extern crate crossterm_winapi;
#[cfg(windows)]
extern crate winapi;

pub mod macros;
pub mod error;
pub mod sys;

mod functions;
mod output;

pub use self::error::{ErrorKind, Result};

#[cfg(windows)]
pub use self::functions::supports_ansi;
