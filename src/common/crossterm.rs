use {Screen, TerminalOutput};

use super::super::cursor;
use super::super::input;
use super::super::style;
use super::super::terminal;

use std::fmt::Display;
use std::sync::Arc;

/// This type could be used to access the `cursor, terminal, color, input, styling` module more easily.
/// You need to pass a reference to the screen whereon you want to perform the actions to the `Crossterm` type.
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
    stdout: Option<Arc<TerminalOutput>>,
}

impl<'crossterm> Crossterm {
    /// Create a new instance of `Crossterm`
    pub fn new() -> Crossterm {
        Crossterm { stdout: None }
    }

    /// Create a new instance of `Crossterm`
    pub fn from_screen(screen: &Screen) -> Crossterm {
        Crossterm {
            stdout: Some(screen.stdout.clone()),
        }
    }

    /// Get a `TerminalCursor` implementation whereon cursor related actions can be performed.
    ///
    /// ```rust
    /// extern crate crossterm;
    /// use crossterm::Crossterm;
    ///
    /// let crossterm = Crossterm::new();
    /// let cursor = crossterm.cursor();
    /// ```
    pub fn cursor(&self) -> cursor::TerminalCursor {
        match &self.stdout {
            None => cursor::TerminalCursor::new(),
            Some(stdout) => cursor::TerminalCursor::from_output(&stdout),
        }
    }

    /// Get a `TerminalInput` implementation whereon terminal related actions can be performed.
    ///
    /// ```rust
    /// extern crate crossterm;
    /// use crossterm::Crossterm;
    ///
    /// let crossterm = Crossterm::new();
    /// let input = crossterm.input();
    /// ```
    pub fn input(&self) -> input::TerminalInput {
        match &self.stdout {
            None => input::TerminalInput::new(),
            Some(stdout) => input::TerminalInput::from_output(&stdout),
        }
    }

    /// Get a `Terminal` implementation whereon terminal related actions can be performed.
    ///
    /// ```rust
    /// extern crate crossterm;
    /// use crossterm::Crossterm;
    ///
    /// let crossterm = Crossterm::new();
    /// let mut terminal = crossterm.terminal();
    /// ```
    pub fn terminal(&self) -> terminal::Terminal {
        match &self.stdout {
            None => terminal::Terminal::new(),
            Some(stdout) => terminal::Terminal::from_output(&stdout),
        }
    }

    /// Get a `TerminalColor` implementation whereon color related actions can be performed.
    ///
    /// ```rust
    /// extern crate crossterm;
    /// use crossterm::Crossterm;
    ///
    /// let crossterm = Crossterm::new();
    /// let mut terminal = crossterm.color();
    /// ```
    pub fn color(&self) -> style::TerminalColor {
        match &self.stdout {
            None => style::TerminalColor::new(),
            Some(stdout) => style::TerminalColor::from_output(&stdout),
        }
    }

    /// This could be used to style a `Displayable` type with colors and attributes.
    ///
    /// ```rust
    /// extern crate crossterm;
    /// use crossterm::Crossterm;
    ///
    /// let crossterm = Crossterm::new();
    ///
    /// // get an styled object which could be painted to the terminal.
    /// let styled_object = crossterm.style("Some Blue colored text on black background")
    ///     .with(Color::Blue)
    ///     .on(Color::Black);
    ///
    /// // print the styled font * times to the current screen.
    /// for i in 1..10
    /// {
    ///     println!("{}", styled_object);
    /// }
    /// ```
    pub fn style<D>(&self, val: D) -> style::StyledObject<D>
    where
        D: Display,
    {
        style::ObjectStyle::new().apply_to(val)
    }
}

impl From<Arc<TerminalOutput>> for Crossterm {
    fn from(stdout: Arc<TerminalOutput>) -> Self {
        Crossterm {
            stdout: Some(stdout),
        }
    }
}

impl From<Screen> for Crossterm {
    fn from(screen: Screen) -> Self {
        Crossterm {
            stdout: Some(screen.stdout.clone()),
        }
    }
}
