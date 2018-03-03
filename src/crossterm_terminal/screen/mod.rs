use std::io::{self, Write};
use std::ops;
use std::any::Any;

use crossterm_state::commands::{shared_commands,win_commands,ICommand, CommandType};
use crossterm_state::commands::IContextCommand;
use shared::traits::Construct;

use std::fmt;

/// let context = ScreenContext::new();
/// ToMainScreen {}.execute(&mut context);
/// ToAlternateScreen {}.execute(context);
///
///
/// ToMainScreen {}.execute(&mut context);
///
/// context.to_main();
/// let alternate_screen = context.to_alternate(stdout());
///
/// let alternate = AlternateScreen::from(stdout, context);
/// ToMainScreen [} .execute(ScreenContext::new()))
/// ToAlternateScreen {}. execute(ScreenContext::new());
pub struct ToMainScreen;

impl fmt::Display for ToMainScreen
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        get_to_alternate_screen_command().undo();
        Ok(())
    }
}

/// Switch to the alternate screen buffer of the terminal.
pub struct ToAlternateScreen;

impl fmt::Display for ToAlternateScreen
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        get_to_alternate_screen_command().execute();
        Ok(())
    }
}

pub struct AlternateScreen<W: Write> {
    /// The output target.
    output: W,
}

impl<W: Write> AlternateScreen< W> {
    pub fn from(mut output: W) -> Self {
        write!(output, "{}", ToAlternateScreen);
        AlternateScreen { output: output }
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

fn get_to_alternate_screen_command() -> Box<ICommand>
{
//    let mut does_support = true;
    let mut command: Option<Box<ICommand>> = None;
//
//    let succeeded = false;
//
//    if cfg!(target_os = "windows")
//    {
//        #[cfg(windows)]
//        use kernel::windows_kernel::ansi_support::try_enable_ansi_support;
//
//        // Try to enable ansi on windows if not than use WINAPI.
//        does_support = try_enable_ansi_support();
//
//        println!("does support: {}", does_support);
//        if !does_support
//        {
            command = Some(win_commands::ToAlternateScreenBufferCommand::new());
            command.unwrap()
//        }
//    }
//
//    if does_support
//    {
//        command = Some(shared_commands::ToAlternateScreenBufferCommand::new().0);
//    }
//
//    command.unwrap()
}