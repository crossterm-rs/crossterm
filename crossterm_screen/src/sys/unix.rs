use libc;
use std::fs;
use std::io::{self, Error, ErrorKind, Read, Result, Write};
use std::os::unix::io::AsRawFd;

/// This command is used for enabling and disabling raw mode for the terminal.
pub struct RawModeCommand;

impl RawModeCommand {
    pub fn new() -> Self {
        RawModeCommand
    }

    /// Enables raw mode.
    pub fn enable(&mut self) -> Result<()> {
        crossterm_utils::sys::unix::into_raw_mode();

        // will be removed in 6.1
        unsafe { crossterm_utils::sys::unix::RAW_MODE_ENABLED_BY_USER = true }
        Ok(())
    }

    /// Disables raw mode.
    pub fn disable(&mut self) -> Result<()> {
        crossterm_utils::sys::unix::disable_raw_mode();

        // will be removed in 6.1
        unsafe { crossterm_utils::sys::unix::RAW_MODE_ENABLED_BY_USER = false }
        Ok(())
    }
}
