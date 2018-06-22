//! This module contains all the logic for switching between alternate screen and main screen.

use Context;
use state::commands::*;

use std::io::{self, Write};
use std::rc::Rc;

pub struct AlternateScreen {
    context: Rc<Context>,
    command_id: u16
}

impl AlternateScreen {
    /// Get the alternate screen from the context.
    /// By calling this method the current screen will be changed to the alternate screen.
    /// And you get back an handle for that screen.
    pub fn from(context: Rc<Context>) -> Self {
        let command_id = get_to_alternate_screen_command(context.clone());

        let screen = AlternateScreen { context: context, command_id: command_id };
        screen.to_alternate();
        return screen;
    }

    /// Change the current screen to the mainscreen.
    pub fn to_main(&self)
    {
        let mut mutex = &self.context.state_manager;
        {
            let mut state_manager = mutex.lock().unwrap();

            let mut mx = &state_manager.get(self.command_id);
            {
                let mut command = mx.lock().unwrap();
                command.undo();
            }
        }
    }

    /// Change the current screen to alternate screen.
    pub fn to_alternate(&self)
    {
        let mut mutex = &self.context.state_manager;
        {
            let mut state_manager = mutex.lock().unwrap();

            let mut mx = &state_manager.get(self.command_id);
            {
                let mut command = mx.lock().unwrap();
                command.execute();
            }
        }
    }
}

impl Write for AlternateScreen {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut screen = self.context.screen_manager.lock().unwrap();
        {
            screen.write(buf)
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        let mut screen = self.context.screen_manager.lock().unwrap();
        {
            screen.flush()
        }
    }
}

impl Drop for AlternateScreen
{
    fn drop(&mut self)
    {
        let mut mutex = &self.context.state_manager;
        {
            let mut state_manager = mutex.lock().unwrap();

            let mut mx = &state_manager.get(self.command_id);
            {
                let mut command = mx.lock().unwrap();
                command.undo();
            }
        }
    }
}

// Get the alternate screen command to enable and disable alternate screen based on the current platform
fn get_to_alternate_screen_command(context: Rc<Context>) -> u16
{
//    #[cfg(target_os = "windows")]
//    let command = functions::get_module::<Box<ICommand>>(win_commands::ToAlternateScreenBufferCommand::new(), shared_commands::ToAlternateScreenBufferCommand::new(), context).unwrap();

//    #[cfg(not(target_os = "windows"))]
     win_commands::ToAlternateScreenBufferCommand::new(context)
}