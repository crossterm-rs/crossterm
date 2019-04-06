extern crate crossterm_utils;
extern crate crossterm_screen;

#[cfg(unix)]
extern crate libc;

mod input;
mod sys;

pub use self::input::{
    input, AsyncReader, InputEvent, KeyEvent, MouseButton, MouseEvent, SyncReader, TerminalInput,
};

pub use self::crossterm_screen::Screen;
