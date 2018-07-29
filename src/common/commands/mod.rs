//! This module contains some commands that could be executed for specific task.

use super::super::manager::ScreenManager;
use std::io::Result;

pub mod shared_commands;

#[cfg(target_os = "unix")]
pub mod unix_command;

#[cfg(target_os = "windows")]
pub mod win_commands;

#[cfg(target_os = "windows")]
pub use self::win_commands::*;
#[cfg(target_os = "unix")]
pub use self::unix_commands::*;

pub use self::shared_commands::*;

/// This trait provides a way to execute some state changing commands.
pub trait IStateCommand {
    fn execute(&mut self) -> bool;
    fn undo(&mut self) -> bool;
}

/// This trait provides an interface for switching to alternate screen and back.
pub trait IAlternateScreenCommand
{
    fn enable(&self,screen_manager: &mut ScreenManager) -> Result<()>;
    fn disable(&self, screen_manager: &mut ScreenManager) -> Result<()>;
}

/// This trait provides an interface for switching to raw mode and back.
pub trait IRawScreenCommand
{
    fn enable(&mut self) -> Result<()>;
    fn disable(&mut self) -> Result<()>;
}