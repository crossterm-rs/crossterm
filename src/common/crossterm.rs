use super::commands::{IRawScreenCommand, IAlternateScreenCommand};

use super::screen::RawScreen;
use super::screen::AlternateScreen;

use super::super::manager;
use super::super::cursor;
use super::super::input;
use super::super::terminal;
use super::super::style;

use std::io::Result;

pub struct Crossterm {
    raw_mode: bool,
    alternate_mode: bool,
    pub active_screen: manager::ScreenManager,
    raw_terminal: Option<Box<IRawScreenCommand>>,
    alternate_screen: Option<Box<IAlternateScreenCommand>>
}

impl<'crossterm> Crossterm
{
    pub fn new() -> Crossterm
    {
        Crossterm
        {
            raw_mode: false,
            alternate_mode: false,
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
                    self.raw_terminal = Some(RawScreen::new());
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

    pub fn terminal(&self) -> terminal::Terminal {
        return terminal::Terminal::new(&self.active_screen)
    }

    pub fn color(&self) -> style::TerminalColor {
        return style::TerminalColor::new(&self.active_screen)
    }
}

impl Drop for Crossterm
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



/// Wraps an displayable object so it can be formatted with colors and attributes.
    ///
    /// Check `/examples/color` in the libary for more spesific examples.
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate crossterm;
    ///
    /// use self::crossterm::style::{paint,Color};
    ///
    /// fn main()
    /// {
    ///     // Create an styledobject object from the text 'Unstyled font'
    ///     // Currently it has the default foregroundcolor and backgroundcolor.
    ///     println!("{}",paint("Unstyled font"));
    ///
    ///     // Create an displayable object from the text 'Colored font',
    ///     // Paint this with the `Red` foreground color and `Blue` backgroundcolor.
    ///     // Print the result.
    ///     let styledobject = paint("Colored font").with(Color::Red).on(Color::Blue);
    ///     println!("{}", styledobject);
    ///
    ///     // Or all in one line
    ///     println!("{}", paint("Colored font").with(Color::Red).on(Color::Blue));
    /// }
    ///
    /// ```
//    pub fn paint<D>(&self, val: D) -> style::StyledObject<D>
//    where
//        D: fmt::Display,
//    {
//        style::ObjectStyle::new().apply_to(val, self.context.clone())
//    }