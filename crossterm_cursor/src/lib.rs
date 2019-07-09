#[macro_use]
extern crate crossterm_utils;

#[cfg(windows)]
extern crate winapi;

mod cursor;
pub mod sys;

pub use self::crossterm_utils::{schedule, Command, ErrorKind, Output, Result};
pub use self::cursor::{cursor, Goto, TerminalCursor};
