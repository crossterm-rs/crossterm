use {Screen, TerminalOutput};

use super::super::cursor;
use super::super::input;
use super::super::style;
use super::super::terminal;

use std::fmt::Display;
use std::sync::Arc;

/// This type could be used to access the `cursor, terminal, color, input, styling` module more easily.
/// You need to pass a reference to the screen where on you want to perform the actions to the `Crossterm` type.
///
/// If you want to use the default screen you could do it like this:
///
/// ```rust
/// extern crate crossterm;
/// use crossterm::{Crossterm, Screen};
///
/// let crossterm = Crossterm::new(&Screen::default());
/// let cursor = crossterm.cursor();
/// ```
///
/// If you want to perform actions on the `AlternateScreen` make sure to pass a reference to the screen of the `AlternateScreen`.
///
/// ```
/// extern crate crossterm;
/// use crossterm::{Crossterm, Screen};
///
/// let main_screen = Screen::default();
///
/// if let Ok(alternate_srceen) = main_screen.enable_alternate_modes(false)
/// {
///    let crossterm = Crossterm::new(&alternate_screen.screen);
///    let cursor = crossterm.cursor();
/// }
/// ```
pub struct Crossterm {
    stdout: Arc<TerminalOutput>
}

impl<'crossterm> Crossterm {
    /// Create a new instance of `Crossterm`
    pub fn new(screen: &Screen) -> Crossterm {
        Crossterm { stdout: screen.stdout.clone() }
    }

    /// Get an `TerminalCursor` implementation whereon cursor related actions can be performed.
    ///
    /// ```rust
    /// extern crate crossterm;
    /// use crossterm::{Crossterm, Screen};
    ///
    /// let crossterm = Crossterm::new(&Screen::default());
    /// let cursor = crossterm.cursor();
    /// ```
    pub fn cursor(&self) -> cursor::TerminalCursor {
        cursor::TerminalCursor::new(&self.stdout)
    }

    /// Get an `TerminalInput` implementation whereon terminal related actions can be performed.
    ///
    /// ```rust
    /// extern crate crossterm;
    /// use crossterm::{Crossterm, Screen};
    /// use crossterm::terminal;
    ///
    /// let crossterm = Crossterm::new(&Screen::default());
    /// let input = crossterm.input();
    /// ```
    pub fn input(&self) -> input::TerminalInput {
        return input::TerminalInput::new(&self.stdout);
    }

    /// Get an `Terminal` implementation whereon terminal related actions can be performed.
    ///
    /// ```rust
    /// extern crate crossterm;
    /// use crossterm::{Crossterm, Screen};
    ///
    /// let crossterm = Crossterm::new(&Screen::default());
    /// let mut terminal = crossterm.terminal();
    /// ```
     pub fn terminal(&self) -> terminal::Terminal {
         return terminal::Terminal::new(&self.stdout);
     }

    /// Get an `TerminalColor` implementation whereon color related actions can be performed.
    ///
    /// ```rust
    /// extern crate crossterm;
    /// use crossterm::{Crossterm, Screen};
    ///
    /// let crossterm = Crossterm::new(&Screen::default());
    /// let mut terminal = crossterm.terminal();
    /// ```
    pub fn color(&self) -> style::TerminalColor {
        return style::TerminalColor::new(&self.stdout);
    }

    /// This could be used to style an `Displayable` type with colors and attributes.
    ///
    /// ```rust
     /// extern crate crossterm;
    /// use crossterm::{Crossterm, Screen};
    ///
    /// let crossterm = Crossterm::new(&Screen::default());
    ///
    /// // get an styled object which could be painted to the terminal.
    /// let styled_object = crossterm.style("Some Blue colored text on black background")
    ///     .with(Color::Blue)
    ///     .on(Color::Black);
    ///
    /// // create an default screen.
    /// let screen = Screen::default();
    ///
    /// // print the styled font * times to the current screen.
    /// for i in 1..10
    /// {
    ///     styled_object.paint(&screen);
    /// }
    /// ```
    pub fn style<D>(&self, val: D) -> style::StyledObject<D>
    where
        D: Display,    {
        style::ObjectStyle::new().apply_to(val)
    }
}

impl From<Arc<TerminalOutput>> for Crossterm
{
    fn from(stdout: Arc<TerminalOutput>) -> Self {
        Crossterm { stdout: stdout }
    }
}