//! This module contains some code that can be used for all module in this library.

#[macro_use]
pub mod macros;
pub mod environment;
pub mod functions;
pub mod screen;
pub mod traits;

#[cfg(target_os = "unix")]
pub mod raw;
