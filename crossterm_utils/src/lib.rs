#![deny(unused_imports)]

pub use self::command::{Command, ExecutableCommand, Output, QueueableCommand};
pub use self::error::{ErrorKind, Result};
#[cfg(windows)]
pub use self::functions::supports_ansi;

mod command;
pub mod error;
mod functions;
pub mod macros;
pub mod sys;
