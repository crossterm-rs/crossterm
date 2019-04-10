extern crate crossterm_screen;
extern crate crossterm_utils;

#[cfg(unix)]
extern crate libc;

mod input;
mod sys;

pub use self::input::{
    input, AsyncReader, InputEvent, KeyEvent, MouseButton, MouseEvent, SyncReader, TerminalInput,
};

pub use self::crossterm_screen::{IntoRawMode, RawScreen};
