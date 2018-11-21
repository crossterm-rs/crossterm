//! This module contains the commands that can be used for unix systems.
use kernel::unix_kernel::terminal;

use std::io::Result;

// This command is used for enabling and disabling raw mode for the terminal.
/// This command is used for enabling and disabling raw mode for the terminal.
pub struct RawModeCommand;

impl RawModeCommand {
    pub fn new() -> Self {
        RawModeCommand
    }

    /// Enables raw mode.
    pub fn enable(&mut self) -> Result<()> {
        terminal::into_raw_mode()?;

        unsafe { terminal::RAW_MODE_ENABLED_BY_USER = true }
        Ok(())
    }

    /// Disables raw mode.
    pub fn disable(&mut self) -> Result<()> {
        terminal::disable_raw_mode()?;

        unsafe { terminal::RAW_MODE_ENABLED_BY_USER = false }
        Ok(())
    }
}
