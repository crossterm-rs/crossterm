//! This module is used for managing the state changes of the terminal.
//!
//! If `crossterm` changes some core state of the terminal like: enabling ANSI or enabling raw mode it should be reverted when the current process ends.
//! It would be a little lame to let the terminal in raw mode after the the current process ends for the user of this library.

mod context;
pub mod commands;

pub use self::context::{Context};