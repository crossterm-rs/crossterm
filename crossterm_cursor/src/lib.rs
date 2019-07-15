#[macro_use]
extern crate crossterm_utils;

#[cfg(windows)]
extern crate winapi;

mod cursor;
pub mod sys;

pub use self::crossterm_utils::{execute, schedule, Command, ErrorKind, Output, Result, QueueableCommand, ExecutableCommand};
pub use self::cursor::{
    cursor, BlinkOff, BlinkOn, Down, Goto, Hide, Left, ResetPos, Right, SavePos, Show,
    TerminalCursor, Up,
};
