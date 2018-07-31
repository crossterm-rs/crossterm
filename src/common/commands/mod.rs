//! This module contains some commands that could be executed for specific task.

use super::super::manager::ScreenManager;
use std::io::Result;

pub mod shared_commands;

#[cfg(not(target_os = "windows"))]
pub mod unix_command;

#[cfg(target_os = "windows")]
pub mod win_commands;

/// This trait provides a way to execute some state changing commands.
pub trait IStateCommand {
    fn execute(&mut self) -> Result<()>;
    fn undo(&mut self) -> Result<()>;
}

pub trait IEnableAnsiCommand {
    fn enable(&mut self) -> bool;
    fn disable(&mut self) -> bool;
}

// This trait provides an interface for switching to alternate screen and back.
pub trait IAlternateScreenCommand: Send {
    fn enable(&self, screen_manager: &mut ScreenManager) -> Result<()>;
    fn disable(&self, screen_manager: &mut ScreenManager) -> Result<()>;
}

// This trait provides an interface for switching to raw mode and back.
/*pub trait IRawScreenCommand: Send{
    fn enable(&mut self) -> Result<()>;
    fn disable(&mut self) -> Result<()>;
}*/
