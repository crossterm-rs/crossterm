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
use state::commands::unix_command::EnableRawModeCommand;
#[cfg(windows)]
use state::commands::win_commands::EnableRawModeCommand;

use Context;
use state::commands::IContextCommand;

use std::io::{ self, Write};

/// A wrapper for the raw terminal state. Which can be used to write to.
pub struct RawTerminal<'a, W: Write>
{
    output: W,
    context : &'a mut Context
}

/// Trait withs contains a method for switching into raw mode.
pub trait IntoRawMode: Write + Sized
{
    fn into_raw_mode<'a>(self, context: &'a mut Context) -> io::Result<RawTerminal<Self>>;
}

impl<W: Write> IntoRawMode for W
{
    /// Switch to raw mode.
    ///
    /// Raw mode means that input (stdin) won't be printed it will instead have to be written manually by
    /// the program. The input isn't canonicalised or line buffered (that is, you can
    /// read from input(stdin) one byte of a time).
    fn into_raw_mode<'a>(self, context: &'a mut Context) -> io::Result<RawTerminal<Self>>
    {
        let (mut command, _) = EnableRawModeCommand::new(context);
        let success = command.execute();

        if success
        {
            Ok(RawTerminal { output: self, context: context})

        }else { panic!("cannot move into raw mode") }
    }
}

impl<'a, W: Write> Write for RawTerminal<'a, W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.output.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.output.flush()
    }
}

/// If an instance of `RawTerminal` will be dropped all terminal changes that are made will be undone.
impl <'a, W: Write> Drop for RawTerminal<'a, W>
{
    fn drop(&mut self)
    {
        self.context.restore_changes();
    }
}