//! This module contains all the logic for switching between alternate screen and main screen.

use Terminal;
use state::commands::*;

use std::io::{self, Write};

//pub struct ToMainScreen;
//
//impl fmt::Display for ToMainScreen
//{
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        get_to_alternate_screen_command().undo();
//        Ok(())
//    }
//}
//
///// Struct that switches to alternate screen buffer on display.
//pub struct ToAlternateScreen;
//
//impl fmt::Display for ToAlternateScreen
//{
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        get_to_alternate_screen_command().execute();
//        Ok(())
//    }
//}

/// Struct that can be used for writing to an alternate screen.
///
/// #Example
///
/// ```rust
/// extern crate crossterm;
/// use self::crossterm::terminal::screen;
/// use std::{time, thread};
///    ...
///
///    // Initialize and switch to the alternate screen from an std output handle.
///    // Now you can write to this screen.
///    let mut screen = screen::AlternateScreen::from(stdout());
///    // Write some text to the alternate screen.
///    write!(screen, "Welcome to the alternate screen. Wait 4 seconds to switch back").unwrap();
///    thread::sleep(time::Duration::from_secs(4));
///    // switch back to main screen.
///    write!(screen, "{}", screen::ToMainScreen);
///    write!(screen, "{}", "We are back again at the main screen");
///
///    ...
///
/// ```
pub struct AlternateScreen<'term> {
    term: &'term Terminal
}

impl<'term> AlternateScreen<'term> {
    pub fn from(output: &'term Terminal) -> Self {
        get_to_alternate_screen_command().execute(&output);
//
//        let mut screen = output.screen_manager.lock().unwrap();
//        {
//            screen.register_output(Box::from(o), true);
//        }
        AlternateScreen { term: output }
    }

    pub fn to_main(&self)
    {
        get_to_alternate_screen_command().undo(&self.term);
    }

    pub fn to_alternate(&self)
    {
        get_to_alternate_screen_command().execute(&self.term);
    }
}

impl<'term> Write for AlternateScreen<'term> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut screen = self.term.screen_manager.lock().unwrap();
        {
            screen.stdout().write(buf)
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        let mut screen = self.term.screen_manager.lock().unwrap();
        {
            screen.stdout().flush()
        }
    }
}

impl<'term> Drop for AlternateScreen<'term>
{
    fn drop(&mut self)
    {
        get_to_alternate_screen_command().undo(&self.term);
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