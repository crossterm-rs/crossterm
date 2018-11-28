//! Ever got disappointed when a terminal library for rust was only written for UNIX systems?
//! Crossterm provides the same core functionalities for both Windows and UNIX systems.
//!
//! Crossterm aims to be simple and easy to call in code.
//! Through the simplicity of Crossterm, you do not have to worry about the platform you are working with.
//!
//! This crate supports all UNIX and Windows terminals down to windows 7 (not all terminals are tested see [Tested Terminals] in the README.

#[macro_use]
mod common;

mod kernel;
mod modules;

pub use modules::terminal;
pub use modules::cursor;
pub use modules::input;
pub use modules::output;
pub use modules::style;

pub use self::style::{color, style, Color, ColorType, Attribute, TerminalColor, ObjectStyle, StyledObject, DisplayableObject};
pub use self::cursor::{cursor, TerminalCursor};
pub use self::input::{input, TerminalInput, AsyncReader, KeyEvent};
pub use self::terminal::{terminal, Terminal};
pub use self::output::TerminalOutput;
pub use common::screen::{AlternateScreen, Screen};
pub use common::Crossterm;

#[cfg(unix)]
extern crate libc;
#[cfg(unix)]
extern crate termios;

#[cfg(windows)]
extern crate winapi;
