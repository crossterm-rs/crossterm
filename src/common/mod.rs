//! This module contains some code that can be used for all module in this library.

#[macro_use]
pub mod macros;
pub mod commands;
pub mod functions;
pub mod screen;
pub mod traits;

mod crossterm;

pub use self::crossterm::Crossterm;
use super::manager::ScreenManager;
