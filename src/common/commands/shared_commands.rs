//! This module contains the commands that can be used for both unix and windows 10 systems because they support ANSI escape codes

use super::{IAlternateScreenCommand, TerminalOutput};

use std::sync::Mutex;
use std::io::{ Result, stdout, Write};

/// This command is used for switching to alternate screen and back to main screen.
pub struct ToAlternateScreenCommand;

impl ToAlternateScreenCommand {
    pub fn new() -> ToAlternateScreenCommand {
        return ToAlternateScreenCommand;
    }
}

impl IAlternateScreenCommand for ToAlternateScreenCommand {

    /// enable alternate screen.
    fn enable(&self, screen_manager: &mut TerminalOutput) -> Result<()> {
        screen_manager.write_str(csi!("?1049h"));
        Ok(())
    }

    /// disable alternate screen.
    fn disable(&self, screen_manager: &TerminalOutput) -> Result<()> {
        screen_manager.write_str(csi!("?1049l"));
        Ok(())
    }
}
