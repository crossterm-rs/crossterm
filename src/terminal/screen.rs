//! This module contains all the logic for switching between alternate screen and main screen.

use shared::functions;
use Context;
use state::commands::*;

use std::{ fmt, ops };
use std::io::{self, Write};

pub struct ToMainScreen;

impl fmt::Display for ToMainScreen
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        get_to_alternate_screen_command().undo();
        Ok(())
    }
}

/// Struct that switches to alternate screen buffer on display.
pub struct ToAlternateScreen;

impl fmt::Display for ToAlternateScreen
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        get_to_alternate_screen_command().execute();
        Ok(())
    }
}

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
pub struct AlternateScreen<W: Write> {
    /// The output target.
    output: W,
    context: Context
}

impl<W: Write>  AlternateScreen<W> {
    pub fn from(mut output: W) -> Self {
        write!(output, "{}", ToAlternateScreen);
        AlternateScreen { output: output, context: Context::new()}
    }
}

impl<W: Write> ops::Deref for AlternateScreen<W> {
    type Target = W;

    fn deref(&self) -> &W {
        &self.output
    }
}

impl<W: Write> ops::DerefMut for AlternateScreen<W> {
    fn deref_mut(&mut self) -> &mut W {
        &mut self.output
    }
}

impl<W: Write> Write for AlternateScreen<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.output.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.output.flush()
    }
}

impl<W: Write> Drop for AlternateScreen<W>
{
    fn drop(&mut self)
    {
        write!(self, "{}", ToMainScreen).expect("switch to main screen");
    }
}

/// Get the alternate screen command to enable and disable alternate screen based on the current platform
fn get_to_alternate_screen_command() -> Box<ICommand>
{
    #[cfg(target_os = "windows")]
    let command = functions::get_module::<Box<ICommand>>(win_commands::ToAlternateScreenBufferCommand::new(), shared_commands::ToAlternateScreenBufferCommand::new()).unwrap();

    #[cfg(not(target_os = "windows"))]
    let command = shared_commands::ToAlternateScreenBufferCommand::new();
    
    command
}