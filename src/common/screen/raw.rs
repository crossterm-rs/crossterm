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

use super::commands::*;
use super::{functions, Screen};

use std::io::{self, Write};

/// A wrapper for the raw terminal state. Which can be used to write to.
pub struct RawScreen
{
    #[cfg(not(target_os = "windows"))]
    command: unix_command::RawModeCommand,
    #[cfg(target_os = "windows")]
    command: win_commands::RawModeCommand,

    pub screen: Screen,
}

impl RawScreen {
    pub fn into_raw_mode(screen: Screen) -> io::Result<RawScreen>
    {
        #[cfg(not(target_os = "windows"))]
        let command = unix_command::RawModeCommand::new();
        #[cfg(target_os = "windows")]
        let command = win_commands::RawModeCommand::new();

        Ok(RawScreen {  command: command, screen})
    }

    pub fn disable_raw_modes(&self) -> io::Result<()>
    {
        self.command.disable()?;
        return Ok(())
    }
}

impl Drop for RawScreen
{
    fn drop(&mut self) {
        self.disable_raw_modes();
    }
}

//
/////// Trait withs contains a method for switching into raw mode.
////pub trait IntoRawMode: Write + Sized {
////    fn into_raw_mode(&self, context: Rc<Context>) -> io::Result<RawTerminal>;
////}
