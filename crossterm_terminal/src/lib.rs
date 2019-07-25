#[macro_use]
extern crate crossterm_utils;
extern crate crossterm_cursor;

#[cfg(windows)]
extern crate crossterm_winapi;

#[cfg(unix)]
extern crate libc;

mod sys;
mod terminal;

pub use self::terminal::{terminal, Clear, ClearType, ScrollDown, ScrollUp, SetSize, Terminal};

pub use crossterm_utils::{execute, queue, Command, ExecutableCommand, QueueableCommand, Result};
