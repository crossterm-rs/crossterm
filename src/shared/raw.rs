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
use super::super::state::commands::unix_command::EnableRawModeCommand;
#[cfg(windows)]
use state::commands::win_commands::EnableRawModeCommand;

use state::commands::IStateCommand;
use {CommandManager, Context};

use std::io::{self, Write};
use std::rc::Rc;

/// A wrapper for the raw terminal state. Which can be used to write to.
pub struct RawTerminal {
    context: Rc<Context>,
    command_id: u16,
}

impl RawTerminal {
    pub fn new(context: &Rc<Context>) -> RawTerminal {
        let command_id = EnableRawModeCommand::new(&context.state_manager);

        RawTerminal {
            context: context.clone(),
            command_id: command_id,
        }
    }

    pub fn enable(&self) -> bool {
        {
            let mutex = &self.context.screen_manager;
            let mut screen = mutex.lock().unwrap();
            screen.set_is_raw_screen(true);
        }

        CommandManager::execute(self.context.clone(), self.command_id)
    }

    pub fn disable(&self) -> bool {
        {
            let mutex = &self.context.screen_manager;
            let mut screen = mutex.lock().unwrap();
            screen.set_is_raw_screen(false);
        }

        CommandManager::undo(self.context.clone(), self.command_id)
    }
}

/// Trait withs contains a method for switching into raw mode.
pub trait IntoRawMode: Write + Sized {
    fn into_raw_mode(&self, context: Rc<Context>) -> io::Result<RawTerminal>;
}

impl<W: Write> IntoRawMode for W {
    /// Raw mode means that input (stdin) won't be printed it will instead have to be written manually by
    /// the program. The input isn't canonicalised or line buffered (that is, you can
    /// read from input(stdin) one byte of a time).
    fn into_raw_mode(&self, context: Rc<Context>) -> io::Result<RawTerminal> {
        let command_id = EnableRawModeCommand::new(&context.state_manager);

        {
            let mutex = &context.screen_manager;
            let mut screen = mutex.lock().unwrap();
            screen.set_is_raw_screen(true);
        }

        let raw_terminal = RawTerminal {
            context: context.clone(),
            command_id: command_id,
        };

        if raw_terminal.enable()
            {
                return Ok(raw_terminal);
            }
        return Err(io::Error::new(io::ErrorKind::Other, "Could not enter raw mode."))
    }
}

impl Write for RawTerminal {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut screen = self.context.screen_manager.lock().unwrap();
        {
            screen.write(buf)
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        let mut screen = self.context.screen_manager.lock().unwrap();
        {
            screen.flush()
        }
    }
}

/// If an instance of `RawTerminal` will be dropped all terminal changes that are made will be undone.
impl Drop for RawTerminal {
    fn drop(&mut self) {
        let success = CommandManager::undo(self.context.clone(), self.command_id);
    }
}
