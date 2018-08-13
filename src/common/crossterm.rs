use super::commands::{IAlternateScreenCommand};

use super::screen::{AlternateScreen, Screen};

use super::super::cursor;
use super::super::input;
use super::super::write;
use super::super::style;
 use super::super::terminal;

use std::fmt::Display;
use std::io::Write;
use std::sync::RwLock;
use std::io::Result;
use std::sync::Arc;

#[cfg(not(windows))]
use common::commands::unix_command;

#[cfg(windows)]
use common::commands::win_commands;

use write::Stdout;

/// This type could be used to access the `cursor, terminal, color, input, styling` module more easily.
/// You need to pass a reference to the screen where on you want to perform the actions.
///
///
/// #Example
/// If you want to use the default screen you could do it like this:
///
/// ```rust
///
/// extern crate crossterm;
/// use crossterm::{Crossterm, Screen};
///
/// let crossterm = Crossterm::new();
/// let cursor = crossterm.cursor(&Screen::default());
///
/// ```
///
/// If you want to perform actions on the `AlternateScreen` make sure to pass a refrence to the screen of the `AlternateScreen`.
///
/// ```
/// /// extern crate crossterm;
/// use crossterm::{Crossterm, Screen};
///
/// let main_screen = Screen::default();
///
/// if let Ok(alternate_srceen) = main_screen.enable_alternate_modes(false)
/// {
///    let crossterm = Crossterm::new();
///    let cursor = crossterm.cursor(&alternate_screen.screen);
/// }
///
/// ```
pub struct Crossterm { }

impl<'crossterm> Crossterm {

    /// Create a new instance of `Crossterm`
    pub fn new() -> Crossterm {
        Crossterm {}
    }

    /// Get an TerminalCursor implementation whereon cursor related actions can be performed.
    ///
    /// #Example
    ///
    /// ```rust
    ///
    /// extern crate crossterm;
    /// use crossterm::{Crossterm, Screen};
    ///
    /// let crossterm = Crossterm::new();
    /// let cursor = crossterm.cursor(&Screen::default());
    ///
    /// ```
    pub fn cursor(&self, screen: &'crossterm Screen) -> cursor::TerminalCursor {
        cursor::TerminalCursor::new(&screen.stdout.clone())
    }

    /// Get an TerminalInput implementation whereon terminal related actions can be performed.
    ///
    /// #Example
    ///
    /// ```rust
    ///
    /// extern crate crossterm;
    /// use crossterm::{Crossterm, Screen};
    /// use crossterm::terminal;
    ///
    /// let crossterm = Crossterm::new();
    /// let input = crossterm.input(&Screen::default());
    ///
    /// ```
    pub fn input(&self, screen: &'crossterm Screen) -> input::TerminalInput {
        return input::TerminalInput::new(&screen.stdout);
    }

    /// Get an Terminal implementation whereon terminal related actions can be performed.
    ///
    /// #Example
    ///
    /// ```rust
    ///
    /// extern crate crossterm;
    /// use crossterm::{Crossterm, Screen};
    ///
    /// let crossterm = Crossterm::new();
    /// let mut terminal = crossterm.terminal(&Screen::default());
    ///
    /// ```
     pub fn terminal(&self, screen: &'crossterm Screen) -> terminal::Terminal {
         return terminal::Terminal::new(&screen.stdout);
     }

    /// Get an TerminalColor implementation whereon color related actions can be performed.
    ///
    /// #Example
    ///
    /// ```rust
    ///
    /// extern crate crossterm;
    /// use crossterm::{Crossterm, Screen};
    ///
    /// let crossterm = Crossterm::new();
    /// let mut terminal = crossterm.terminal(&Screen::default());
    ///
    /// ```
    pub fn color(&self, screen: &'crossterm Screen) -> style::TerminalColor {
        return style::TerminalColor::new(&screen.stdout);
    }

    /// This could be used to style an Displayable with colors and attributes.
    ///
    /// #Example
    ///
    /// ```rust
    ///
    /// use crossterm::{ Screen };
    ///
    /// // get an styled object which could be painted to the terminal.
    /// let styled_object = style("Some Blue colored text on black background").with(Color::Blue).on(Color::Black);
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