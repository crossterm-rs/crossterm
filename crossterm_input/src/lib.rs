#![deny(unused_imports)]

pub use crossterm_screen::{IntoRawMode, RawScreen};

pub use self::input::{
    input, AsyncReader, InputEvent, KeyEvent, MouseButton, MouseEvent, SyncReader, TerminalInput,
};

mod input;
mod sys;
