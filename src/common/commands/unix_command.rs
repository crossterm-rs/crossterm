//! This module contains the commands that can be used for unix systems.

use super::{ IStateCommand, IRawScreenCommand};
use kernel::unix_kernel::terminal;
use termios::{tcsetattr, Termios, CREAD, ECHO, ICANON, TCSAFLUSH};

const FD_STDIN: ::std::os::unix::io::RawFd = 1;

use std::io::{Error, ErrorKind, Result};

/// This command is used for switching to NoncanonicalMode.
#[derive(Copy, Clone)]
pub struct NoncanonicalModeCommand;

impl NoncanonicalModeCommand {
    pub fn new() -> NoncanonicalModeCommand {
        NoncanonicalModeCommand {}
    }
}

impl IStateCommand for NoncanonicalModeCommand {
    fn execute(&mut self) -> Result<()> {
        // Set noncanonical mode
        if let Ok(orig) = Termios::from_fd(FD_STDIN) {
            let mut noncan = orig.clone();
            noncan.c_lflag &= !ICANON;
            noncan.c_lflag &= !ECHO;
            noncan.c_lflag &= !CREAD;
            tcsetattr(FD_STDIN, TCSAFLUSH, &noncan)?;
        } else {
            return Err(Error::new(
                ErrorKind::Other,
                "Could not set console mode when enabling raw mode",
            ));
        }
        Ok(())
    }

    fn undo(&mut self) -> Result<()> {
        // Disable noncanonical mode
        if let Ok(orig) = Termios::from_fd(FD_STDIN) {
            let mut noncan = orig.clone();
            noncan.c_lflag &= ICANON;
            noncan.c_lflag &= ECHO;
            noncan.c_lflag &= CREAD;

            tcsetattr(FD_STDIN, TCSAFLUSH, &noncan)?;
        } else {
            return Err(Error::new(
                ErrorKind::Other,
                "Could not set console mode when enabling raw mode",
            ));
        }
        Ok(())
    }
}

/// This command is used for enabling and disabling raw mode for the terminal.
pub struct RawModeCommand {
    original_mode: Result<Termios>,
}

impl RawModeCommand
{
    pub fn new() -> Self {
        RawModeCommand {
            original_mode: terminal::get_terminal_mode(),
        }
    }
}

impl RawModeCommand {
    /// Enables raw mode.
    fn enable(&mut self) -> Result<()> {
        if let Ok(original_mode) = self.original_mode {
            let mut new_mode = original_mode;
            terminal::make_raw(&mut new_mode);
            terminal::set_terminal_mode(&new_mode);
        } else {
            return Err(Error::new(
                ErrorKind::Other,
                "Could not set console mode when enabling raw mode",
            ));
        }
        Ok(())
    }

    /// Disables raw mode.
    fn disable(&self) -> Result<()> {
        if let Ok(ref original_mode) = self.original_mode {
            let result = terminal::set_terminal_mode(&original_mode)?;
        } else {
            return Err(Error::new(
                ErrorKind::Other,
                "Could not set console mode when enabling raw mode",
            ));
        }
        Ok(())
    }
}
