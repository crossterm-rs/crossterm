//! This module is used for managing the state changes of the terminal.
//!
//! If `crossterm` changes some core state of the terminal like: enabling ANSI or enabling raw mode it should be reverted when the current process ends.
//! It would be a little lame to let the terminal in raw mode after the the current process ends for the user of this library.

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

/// This command is used for complex commands whits change the terminal state.
/// By passing an `Context` instance this command will register it self to notify the terminal state change.
pub trait IStateCommand {
    fn execute(&mut self) -> bool;
    fn undo(&mut self) -> bool;
}

pub trait IAlternateScreenCommand
{
    fn to_alternate_screen(&self,screen_manager: &mut ScreenManager) -> Result<()>;
    fn to_main_screen(&self, screen_manager: &mut ScreenManager) -> Result<()>;
}

pub trait IRawScreenCommand
{
    fn enable(&mut self) -> Result<()>;
    fn disable(&mut self) -> Result<()>;
}