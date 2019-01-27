extern crate crossterm_utils;

mod input;
mod sys;

pub use self::input::{input, AsyncReader, KeyEvent, TerminalInput};
