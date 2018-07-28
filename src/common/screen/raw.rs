//! This module is used for enabling and disabling raw mode for the terminal.
//!
//! What exactly is raw state:
//! - No line buffering.
//!    Normally the terminals uses line buffering. This means that the input will be send to the terminal line by line.
//!    With raw mode the input will be send one byte at a time.
//! - Input
//!   All input has to be written manually by the programmer.
//! - Characters
//!   The characters are not processed by the terminal driver, but are sent straight through.
//!   Special character have no meaning, like backspace will not be interpret as backspace but instead will be directly send to the terminal.
//!
//! With these modes you can easier design the terminal screen.

#[cfg(not(windows))]
use common::commands::unix_command::EnableRawModeCommand;

#[cfg(windows)]
use common::commands::EnableRawModeCommand;

use super::{functions, ScreenManager};
use super::commands;

use std::io::{self, Write};

/// A wrapper for the raw terminal state. Which can be used to write to.
pub struct RawScreen;

impl RawScreen {
    pub fn new() -> Box<commands::IRawScreenCommand> {
        Box::from(EnableRawModeCommand::new())
    }
}

///// Trait withs contains a method for switching into raw mode.
//pub trait IntoRawMode: Write + Sized {
//    fn into_raw_mode(&self, context: Rc<Context>) -> io::Result<RawTerminal>;
//}