//! This module contains all the logic for switching between alternate screen and main screen.

use Context;
use state::commands::*;

use std::io::{self, Write};

pub struct AlternateScreen<'context> {
    context: &'context Context
}

impl<'context> AlternateScreen<'context> {
    /// Get the alternate screen from the context.
    /// By calling this method the current screen will be changed to the alternate screen.
    /// And you get back an handle for that screen.
    pub fn from(context: &'context Context) -> Self {
        get_to_alternate_screen_command().execute(&context);
        AlternateScreen { context: context }
    }

    /// Change the current screen to the mainscreen.
    pub fn to_main(&self)
    {
        get_to_alternate_screen_command().undo(&self.context);
    }

    /// Change the current screen to alternate screen.
    pub fn to_alternate(&self)
    {
        get_to_alternate_screen_command().execute(&self.context);
    }
}

impl<'context> Write for AlternateScreen<'context> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut screen = self.context.screen_manager.lock().unwrap();
        {
            screen.stdout().write(buf)
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        let mut screen = self.context.screen_manager.lock().unwrap();
        {
            screen.stdout().flush()
        }
    }
}

impl<'context> Drop for AlternateScreen<'context>
{
    fn drop(&mut self)
    {
        get_to_alternate_screen_command().undo(&self.context);
    }
}

// Get the alternate screen command to enable and disable alternate screen based on the current platform
fn get_to_alternate_screen_command() -> Box<ICommand>
{
    #[cfg(target_os = "windows")]
    let command = functions::get_module::<Box<ICommand>>(win_commands::ToAlternateScreenBufferCommand::new(), shared_commands::ToAlternateScreenBufferCommand::new()).unwrap();

    #[cfg(not(target_os = "windows"))]
    let command = shared_commands::ToAlternateScreenBufferCommand::new();

    command
}