use super::commands::{IRawScreenCommand, IAlternateScreenCommand};

use super::screen::RawScreen;
use super::screen::AlternateScreen;

use super::super::manager;
use super::super::cursor;
use super::super::input;
use super::super::terminal;
use super::super::style;

use std::fmt::{Display};
use std::io::Write;

use std::io::Result;

pub struct Crossterm {
    pub active_screen: manager::ScreenManager,
    raw_terminal: Option<Box<IRawScreenCommand>>,
    // Would be cool to figure out a way to have multiple screens instead of just only the main and alternate screen.
    // For windows this would be easy but for unix I have no idea.
    alternate_screen: Option<Box<IAlternateScreenCommand>>
}

impl<'crossterm> Crossterm
{
    pub fn new() -> Crossterm
    {
        Crossterm
        {
            active_screen:  manager::ScreenManager::new(),
            raw_terminal: None,
            alternate_screen: None,
        }
    }

    pub fn enable_raw_mode(&mut self) -> Result<()> {
        match self.raw_terminal
            {
                None => {
                    self.raw_terminal = Some(RawScreen::new());
                    return self.enable_raw_mode();
                },
                Some(ref mut raw_terminal) => {
                    raw_terminal.enable()?;
                    self.active_screen.set_is_raw_screen(true);
                },
            }

        return Ok(())
    }

    pub fn disable_raw_mode(&mut self) -> Result<()>
    {
        match self.raw_terminal
            {
                None => {
                    self.raw_terminal = Some(RawScreen::new());
                    return self.disable_raw_mode();
                },
                Some(ref mut raw_terminal) => {
                    raw_terminal.disable()?;
                    self.active_screen.set_is_raw_screen(false);
                },
            }

        return Ok(())
    }

    pub fn to_alternate_screen(&mut self) -> Result<()>
    {
        match self.alternate_screen
            {
                None => {
                    self.alternate_screen = Some(AlternateScreen::new());
                    return self.to_alternate_screen();
                },
                Some(ref mut alternate_screen) => {
                    alternate_screen.enable(&mut self.active_screen)?;
                    self.active_screen.set_is_alternate_screen(true);
                },
            }

        return Ok(())
    }

    pub fn to_main_screen(&mut self) -> Result<()>
    {
        match self.alternate_screen
            {
                None => {
                    self.alternate_screen = Some(AlternateScreen::new());
                    return self.to_main_screen();
                },
                Some(ref mut alternate_screen) => {
                    alternate_screen.disable(&mut self.active_screen)?;
                    self.active_screen.set_is_alternate_screen(false);
                },
            }

        return Ok(())
    }

    pub fn cursor(&self) -> cursor::TerminalCursor {
        cursor::TerminalCursor::new(&self.active_screen)
    }

    pub fn input(&self) -> input::TerminalInput {
        return input::TerminalInput::new(&self.active_screen)
    }

    pub fn terminal(&self) -> terminal::Terminal {
        return terminal::Terminal::new(&self.active_screen)
    }

    pub fn color(&self) -> style::TerminalColor {
        return style::TerminalColor::new(&self.active_screen)
    }

    // Wraps an displayable object so it can be formatted with colors and attributes.
    //
    // Check `/examples/color` in the libary for more spesific examples.
    //
    pub fn paint<D>(&self, val: D) -> style::StyledObject<D>
    where
        D: Display,
    {
        style::ObjectStyle::new().apply_to(val, &self.active_screen)
    }
}

impl Write for Crossterm
{
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.active_screen.write_buf(buf)
    }

    fn flush(&mut self) -> Result<()> {
        self.active_screen.flush()
    }
}

impl Drop for Crossterm
{
    fn drop(&mut self) {
        if let Some(ref mut screen) = self.alternate_screen
        {
            screen.disable(&mut self.active_screen);
        }
        if let Some(ref mut raw_terminal) = self.raw_terminal
        {
            raw_terminal.disable();
        }
    }
}



