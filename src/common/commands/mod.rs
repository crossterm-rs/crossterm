//! This module contains some commands that could be executed for specific task.

use super::super::output::TerminalOutput;
use std::io;
use std::sync::Mutex;

pub mod shared_commands;
use common::screen::Screen;

#[cfg(not(target_os = "windows"))]
pub mod unix_command;

#[cfg(target_os = "windows")]
pub mod win_commands;

/// This trait provides a way to execute some state changing commands.
pub trait IStateCommand {
    fn execute(&mut self) -> io::Result<()>;
    fn undo(&mut self) -> io::Result<()>;
}

pub trait IEnableAnsiCommand {
    fn enable(&self) -> bool;
    fn disable(&self) -> bool;
}

// This trait provides an interface for switching to alternate screen and back.
pub trait IAlternateScreenCommand: Send {
    fn enable(&self, screen_manager: &mut TerminalOutput) -> io::Result<()>;
    fn disable(&self, screen_manager: &TerminalOutput) -> io::Result<()>;
}

// This trait provides an interface for switching to raw mode and back.
pub trait IRawScreenCommand: Send{
    fn enable(&mut self) -> io::Result<()>;
    fn disable(&self) -> io::Result<()>;
}
