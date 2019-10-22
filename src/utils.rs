//! # Utils

pub use self::command::{Command, ExecutableCommand, Output, QueueableCommand};
pub use self::error::{ErrorKind, Result};
#[cfg(windows)]
pub use self::functions::supports_ansi;

mod command;
mod error;
mod functions;
pub(crate) mod macros;
pub(crate) mod sys;
