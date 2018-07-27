//! Crossterm provides the same core functionalities for both windows and unix systems.
//! Crossterm aims to be simple and easy to call in code.
//! True the simplicity of Crossterm you do not have to worry about the platform your working with.
//! You can just call the action you want to perform and under water it will check what to do based on the current platform.

#[macro_use]
mod shared;
mod kernel;
mod state;

pub mod cursor;
pub mod input;
pub mod manager;
//pub mod style;
pub mod terminal;

pub use shared::Terminal::Terminal;
//pub use shared::crossterm::Crossterm;
pub use shared::raw;
pub use shared::screen;
pub use state::context::Context;

use manager::ScreenManager;
use state::command_manager::CommandManager;
use state::commands::IStateCommand;
use state::state_manager::StateManager;

#[cfg(unix)]
extern crate libc;
#[cfg(unix)]
extern crate termios;

#[cfg(windows)]
extern crate winapi;
