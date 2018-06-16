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
//!
//! # Example
//!
//! ```rust
//! to be implemented
//!
//! ```

#[cfg(not(windows))]
use super::super::state::commands::unix_command::EnableRawModeCommand;
#[cfg(windows)]
use state::commands::win_commands::EnableRawModeCommand;

use {Context, CommandManager };
use state::commands::IStateCommand;

use std::io::{ self, Write};
use std::rc::Rc;

/// A wrapper for the raw terminal state. Which can be used to write to.
pub struct RawTerminal<'a>
{
    terminal : &'a Context,
    command_id: u16,
}

/// Trait withs contains a method for switching into raw mode.
pub trait IntoRawMode<'a>: Write + Sized
{
    fn into_raw_mode(&self, terminal: &'a Context) -> io::Result<RawTerminal<'a>>;
}

impl <'a, W: Write> IntoRawMode<'a> for W
{
    /// Raw mode means that input (stdin) won't be printed it will instead have to be written manually by
    /// the program. The input isn't canonicalised or line buffered (that is, you can
    /// read from input(stdin) one byte of a time).
    fn into_raw_mode(&self, terminal: &'a Context) -> io::Result<RawTerminal<'a>> {
        let command_id = EnableRawModeCommand::new(&terminal.state_manager);

        let success = CommandManager::execute(terminal, command_id);

        if success
        {
            Ok(RawTerminal { terminal: &terminal, command_id: command_id})
        }else { panic!("cannot move into raw mode") }
    }
}

impl<'a> Write for RawTerminal<'a> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut screen = self.terminal.screen_manager.lock().unwrap();
        {
            screen.stdout().write(buf)
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        let mut screen = self.terminal.screen_manager.lock().unwrap();
        {
            screen.stdout().flush()
        }
    }
}

/// If an instance of `RawTerminal` will be dropped all terminal changes that are made will be undone.
impl <'a> Drop for RawTerminal<'a>
{
    fn drop(&mut self)
    {
        let success = CommandManager::undo(&self.terminal, self.command_id);
    }
}