use super::IAlternateScreenCommand;
use crossterm_utils::TerminalOutput;
use crossterm_winapi::{ConsoleMode, Handle, ScreenBuffer};
use std::io;
use winapi::shared::minwindef::DWORD;
use winapi::um::wincon;

/// This command is used for enabling and disabling raw mode for windows systems.
/// For more info check: https://docs.microsoft.com/en-us/windows/console/high-level-console-modes.
#[derive(Clone, Copy)]
pub struct RawModeCommand {
    mask: DWORD,
}
use self::wincon::{ENABLE_LINE_INPUT, ENABLE_WRAP_AT_EOL_OUTPUT};
impl RawModeCommand {
    pub fn new() -> Self {
        RawModeCommand {
            mask: ENABLE_WRAP_AT_EOL_OUTPUT | ENABLE_LINE_INPUT,
        }
    }
}

impl RawModeCommand {
    /// Enables raw mode.
    pub fn enable(&mut self) -> io::Result<()> {
        let console_mode = ConsoleMode::new()?;

        let dw_mode = console_mode.mode()?;

        let new_mode = dw_mode & !self.mask;

        console_mode.set_mode(new_mode)?;

        Ok(())
    }

    /// Disables raw mode.
    pub fn disable(&self) -> io::Result<()> {
        let console_mode = ConsoleMode::new()?;

        let dw_mode = console_mode.mode()?;

        let new_mode = dw_mode | self.mask;

        console_mode.set_mode(new_mode)?;

        return Ok(());
    }
}

/// This command is used for switching to alternate screen and back to main screen.
/// check https://docs.microsoft.com/en-us/windows/console/reading-and-writing-blocks-of-characters-and-attributes for more info
pub struct ToAlternateScreenCommand;

impl ToAlternateScreenCommand {
    pub fn new() -> ToAlternateScreenCommand {
        return ToAlternateScreenCommand {};
    }
}

impl IAlternateScreenCommand for ToAlternateScreenCommand {
    fn enable(&self, _stdout: &mut TerminalOutput) -> io::Result<()> {
        let alternate_screen = ScreenBuffer::create();
        alternate_screen.show()?;
        Ok(())
    }

    fn disable(&self, _stdout: &TerminalOutput) -> io::Result<()> {
        let screen_buffer = ScreenBuffer::from(Handle::output_handle()?);
        screen_buffer.show()?;
        Ok(())
    }
}
