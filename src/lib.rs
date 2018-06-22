//! Crossterm provides the same core functionalities for both windows and unix systems.
//! Crossterm aims to be simple and easy to call in code.
//! True the simplicity of Crossterm you do not have to worry about the platform your working with.
//! You can just call the action you want to preform and under water it will check what to do based on the current platform.

#[macro_use]
mod shared;
mod kernel;
mod state;

pub mod cursor;
pub mod style;
pub mod terminal;
pub mod manager;

pub use shared::{screen, raw};
pub use state::context::Context;

use state::commands::IStateCommand;
use state::command_manager::CommandManager;
use state::state_manager::StateManager;
use manager::ScreenManager;

#[cfg(windows)]
extern crate winapi;
#[cfg(unix)]
extern crate libc;
#[cfg(unix)]
extern crate termios;