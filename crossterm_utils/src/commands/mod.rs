//! This module contains some commands that could be executed for a specific task. A `Command` is just a little wrapper.

use crate::output::TerminalOutput;
use std::io;

//pub mod shared_commands;
//
//#[cfg(not(target_os = "windows"))]
//pub mod unix_command;

#[cfg(target_os = "windows")]
pub mod win_commands;

/// This trait provides a way to execute some state changing commands.
pub trait IStateCommand {
    fn execute(&mut self) -> io::Result<()>;
    fn undo(&mut self) -> io::Result<()>;
}

// This trait provides an interface for switching to alternate screen and back.
pub trait IAlternateScreenCommand: Sync + Send {
    fn enable(&self, stdout: &mut TerminalOutput) -> io::Result<()>;
    fn disable(&self, stdout: &TerminalOutput) -> io::Result<()>;
}

// This trait provides an interface for switching to raw mode and back.
pub trait IRawScreenCommand: Sync + Send {
    fn enable(&mut self) -> io::Result<()>;
    fn disable(&self) -> io::Result<()>;
}
