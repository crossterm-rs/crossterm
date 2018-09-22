//! This module contains the commands that can be used for unix systems.

use super::{ IStateCommand};
use kernel::unix_kernel::terminal;
use termios::{tcsetattr, Termios, CREAD, ECHO, ICANON, TCSAFLUSH, BRKINT, ICRNL, INPCK, ISTRIP, IXON, OPOST, CS8, IEXTEN, ISIG,VTIME, VMIN};
use libc::STDIN_FILENO;
use std::sync::{Once, ONCE_INIT};
static TERMINAL_MODE: Once = ONCE_INIT;


use std::io::{Error, ErrorKind, Result};

/// This command is used for switching to NoncanonicalMode.
#[derive(Copy, Clone)]
pub struct NoncanonicalModeCommand;

impl NoncanonicalModeCommand {
    pub fn new() -> NoncanonicalModeCommand {
        NoncanonicalModeCommand {}
    }
}
static mut ORIGINAL: Option<Termios> = None;

impl NoncanonicalModeCommand {
    pub fn enable(&mut self) -> Result<()> {
        // Set noncanonical mode
        if let Ok(orig) = Termios::from_fd(STDIN_FILENO) {
            TERMINAL_MODE.call_once(|| {
               unsafe { ORIGINAL = Some(orig.clone()); }
            });

            let mut noncan = orig.clone();
            noncan.c_iflag &= !(BRKINT | ICRNL | INPCK | ISTRIP | IXON);
            noncan.c_oflag &= !(OPOST);
            noncan.c_cflag |= (CS8);
            noncan.c_lflag &= !(ECHO | ICANON | IEXTEN | ISIG);
            noncan.c_cc[VMIN] = 0;
            noncan.c_cc[VTIME] = 1;

            tcsetattr(STDIN_FILENO, TCSAFLUSH, &noncan)?;
        } else {
            return Err(Error::new(
                ErrorKind::Other,
                "Could not set console mode when enabling raw mode",
            ));
        }

        unsafe { terminal::RAW_MODE_ENABLED_BY_USER = true }
        Ok(())
    }

    pub fn disable(&self) -> Result<()> {

        unsafe {
            if let Some(original) = ORIGINAL
            {
                tcsetattr(STDIN_FILENO, TCSAFLUSH, &original)?;
            }
        }

        unsafe { terminal::RAW_MODE_ENABLED_BY_USER = false }
        Ok(())
    }
}

// This command is used for enabling and disabling raw mode for the terminal.
/// This command is used for enabling and disabling raw mode for the terminal.
pub struct RawModeCommand;

impl RawModeCommand {
    pub fn new() -> Self {
        return RawModeCommand {}
    }

    /// Enables raw mode.
    pub fn enable(&mut self) -> Result<()> {
        terminal::into_raw_mode();

        unsafe { terminal::RAW_MODE_ENABLED_BY_USER = true }
        Ok(())
    }

    /// Disables raw mode.
    pub fn disable(&mut self) -> Result<()> {
       terminal::disable_raw_mode();

        unsafe { terminal::RAW_MODE_ENABLED_BY_USER = false }
        Ok(())
    }
}

