//! # Utils

pub use self::{
    command::{Command, ExecutableCommand, QueueableCommand},
    error::{ErrorKind, Result},
};

mod command;
mod error;
#[cfg(windows)]
pub(crate) mod functions;
pub(crate) mod macros;
pub(crate) mod sys;
