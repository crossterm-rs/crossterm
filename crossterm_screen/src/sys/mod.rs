#[cfg(unix)]
pub mod unix;

#[cfg(windows)]
pub mod winapi;

use std::io::Write;
use crossterm_utils::Result;

/// This command is used for switching to alternate screen and back to main screen.
pub struct ToAlternateScreenCommand;

impl ToAlternateScreenCommand {
    pub fn new() -> ToAlternateScreenCommand {
        ToAlternateScreenCommand
    }
}

impl IAlternateScreenCommand for ToAlternateScreenCommand {
    /// enable alternate screen.
    fn enable(&self) -> Result<()> {
        write_cout!(csi!("?1049h"))?;
        Ok(())
    }

    /// disable alternate screen.
    fn disable(&self) -> Result<()> {
        write_cout!(csi!("?1049l"))?;
        Ok(())
    }
}

// This trait provides an interface for switching to alternate screen and back.
pub trait IAlternateScreenCommand: Sync + Send {
    fn enable(&self) -> Result<()>;
    fn disable(&self) -> Result<()>;
}
