extern crate crossterm_utils;
#[cfg(unix)]
extern crate libc;

mod input;
mod sys;

pub use self::input::{
    input, parse_event, AsyncReader, InputEvent, KeyEvent, MouseButton, MouseEvent, TerminalInput,
};
