#[cfg(unix)]
extern crate libc;
#[cfg(windows)]
extern crate crossterm_winapi;
#[cfg(windows)]
extern crate winapi;

pub mod error;
pub mod macros;
pub mod sys;

mod functions;

pub use self::error::{ErrorKind, Result};

#[cfg(windows)]
pub use self::functions::supports_ansi;
