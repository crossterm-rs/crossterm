//! This module contains the commands that can be used for both unix and windows systems. Or else said terminals that support ansi codes.
use super::{IAlternateScreenCommand, ScreenManager};

use std::io::Result;

/// This command is used for switching to alternate screen and back to main screen.
pub struct ToAlternateScreenBufferCommand;

impl  ToAlternateScreenBufferCommand {
    pub fn new() -> Box<ToAlternateScreenBufferCommand> {
      return Box::new(ToAlternateScreenBufferCommand {});
    }
}

impl IAlternateScreenCommand for ToAlternateScreenBufferCommand {
    fn to_alternate_screen(&self, screen_manager: &mut ScreenManager) -> Result<()> {
        screen_manager.write_str(csi!("?1049h"));
        Ok(())
    }

    fn to_main_screen(&self, screen_manager: &mut ScreenManager) -> Result<()> {
        screen_manager.write_str(csi!("?1049l"));
        Ok(())
    }
}
