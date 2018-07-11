//! This module contains some code that can be used for all module in this library.

#[macro_use]
pub mod macros;
pub mod crossterm;
pub mod functions;
pub mod traits;

pub mod screen;

#[cfg(target_os = "unix")]
pub mod raw;
