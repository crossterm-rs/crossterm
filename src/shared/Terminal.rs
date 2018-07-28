use {StateManager, ScreenManager};
use super::super::state::commands::*;
use super::raw::RawTerminal;
use super::screen::AlternateScreen;

use super::super::cursor;
use super::super::input;
//use super::super::terminal;

use std::collections::HashMap;

use std::io::Result;

pub struct Terminal{
    raw_mode: bool,
    alternate_mode: bool,
    active_screen: ScreenManager,
    raw_terminal: Option<Box<IRawScreenCommand>>,
    alternate_screen: Option<Box<IAlternateScreenCommand>>
}

impl Terminal{
    pub fn new() -> Terminal
    {
        Terminal
        {
            raw_mode: false,
            alternate_mode: false,
            active_screen: ScreenManager::new(),
            raw_terminal: None,
            alternate_screen: None,
        }
    }

    pub fn enable_raw_mode(&mut self) -> Result<()> {
        match self.raw_terminal
        {
            None => {
                self.raw_terminal = Some(RawTerminal::new());
                return self.enable_raw_mode();
            },
            Some(ref mut raw_terminal) => {
                raw_terminal.enable()?;
                self.raw_mode = true;
            },
        }

        return Ok(())
    }

    pub fn disable_raw_mode(&mut self) -> Result<()>
    {
        match self.raw_terminal
        {
            None => {
                self.raw_terminal = Some(RawTerminal::new());
                return self.disable_raw_mode();
            },
            Some(ref mut raw_terminal) => {
                raw_terminal.disable()?;
                self.raw_mode = false;
            },
        }

        return Ok(())
    }

    pub fn enable_alternate_screen(&mut self) -> Result<()>
    {
        match self.alternate_screen
        {
            None => {
                self.alternate_screen = Some(AlternateScreen::new());
                return self.enable_alternate_screen();
            },
            Some(ref mut alternate_screen) => {
                alternate_screen.to_alternate_screen(&mut self.active_screen)?;
                self.alternate_mode = true;
            },
        }

        return Ok(())
    }

    pub fn disable_alternate_screen(&mut self) -> Result<()>
    {
        match self.alternate_screen
        {
            None => {
                self.alternate_screen = Some(AlternateScreen::new());
                return self.disable_alternate_screen();
            },
            Some(ref mut alternate_screen) => {
                alternate_screen.to_main_screen(&mut self.active_screen)?;
                self.alternate_mode = false;
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

}

impl Drop for Terminal
{
    fn drop(&mut self) {
        if let Some(ref mut screen) = self.alternate_screen
        {
            screen.to_main_screen(&mut self.active_screen);
        }
        if let Some(ref mut raw_terminal) = self.raw_terminal
        {
            raw_terminal.disable();
        }
    }
}