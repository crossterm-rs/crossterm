use std::fmt::Display;
use std::io::{self, Write};
use std::sync::Arc;

/// This type offers a easy way to use functionalities like `cursor, terminal, color, input, styling`.
///
/// To get a cursor instance to perform cursor related actions, you can do the following:
///
/// ```rust
/// let crossterm = Crossterm::new();
/// let cursor = crossterm.cursor();
/// ```
///
/// If you want to perform actions on the `AlternateScreen` make sure to pass a reference to the screen of the `AlternateScreen`.
/// If you don't do this you actions won't be performed on the alternate screen but on the main screen.
///
/// ```
/// let main_screen = Screen::default();
///
/// if let Ok(alternate_srceen) = main_screen.enable_alternate_modes(false)
/// {
///    let crossterm = Crossterm::new(&alternate_screen.screen);
///    let cursor = crossterm.cursor();
/// }
/// ```
///
/// # Remark
/// - depending on the feature flags you've enabled you are able to call methods of this type.
/// - checkout the crossterm book for more information about feature flags or alternate screen.
pub struct Crossterm;

impl<'crossterm> Crossterm {
    /// Create a new instance of `Crossterm`
    pub fn new() -> Crossterm {
        Crossterm
    }

    /// Get a `TerminalCursor` implementation whereon cursor related actions can be performed.
    ///
    /// ```rust
    /// let crossterm = Crossterm::new();
    /// let cursor = crossterm.cursor();
    /// ```
    #[cfg(feature = "cursor")]
    pub fn cursor(&self) -> crossterm_cursor::TerminalCursor {
        crossterm_cursor::TerminalCursor::new()
    }

    /// Get a `TerminalInput` implementation whereon terminal related actions can be performed.
    ///
    /// ```rust
    /// let crossterm = Crossterm::new();
    /// let input = crossterm.input();
    /// ```
    #[cfg(feature = "input")]
    pub fn input(&self) -> crossterm_input::TerminalInput {
        crossterm_input::TerminalInput::new()
    }

    /// Get a `Terminal` implementation whereon terminal related actions can be performed.
    ///
    /// ```rust
    /// let crossterm = Crossterm::new();
    /// let mut terminal = crossterm.terminal();
    /// ```
    #[cfg(feature = "terminal")]
    pub fn terminal(&self) -> crossterm_terminal::Terminal {
        crossterm_terminal::Terminal::new()
    }

    /// Get a `TerminalColor` implementation whereon color related actions can be performed.
    ///
    /// ```rust
    /// let crossterm = Crossterm::new();
    /// let mut terminal = crossterm.color();
    /// ```
    #[cfg(feature = "style")]
    pub fn color(&self) -> crossterm_style::TerminalColor {
        crossterm_style::TerminalColor::new()
    }

    /// This could be used to style any type implementing `Display` with colors and attributes.
    ///
    /// # Example
    /// ```rust
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
    ///
    /// # Remark
    /// `val`:  any type implementing Display e.g. string.
    #[cfg(feature = "style")]
    pub fn style<D>(&self, val: D) -> crossterm_style::StyledObject<D>
    where
        D: Display,
    {
        crossterm_style::ObjectStyle::new().apply_to(val)
    }

    /// This could be used to paint the styled object onto the given screen. You have to pass a reference to the screen whereon you want to perform the painting.
    ///
    /// ``` rust
    /// style("Some colored text")
    ///     .with(Color::Blue)
    ///     .on(Color::Black)
    ///     .paint(&screen);
    /// ```
    ///
    /// You should take note that `StyledObject` implements `Display`. You don't need to call paint unless you are on alternate screen.
    /// Checkout `StyledObject::into_displayable()` for more information about this.
    #[cfg(feature = "style")]
    #[cfg(feature = "screen")]
    pub fn paint<'a, D: Display + 'a>(
        &self,
        styled_object: crossterm_style::StyledObject<D>,
    ) -> super::crossterm_utils::Result<()> {
        let colored_terminal = super::TerminalColor::new();

        let mut reset = false;

        if let Some(bg) = styled_object.object_style.bg_color {
            colored_terminal.set_bg(bg)?;
            reset = true;
        }

        if let Some(fg) = styled_object.object_style.fg_color {
            colored_terminal.set_fg(fg)?;
            reset = true;
        }

        let mut stdout = io::stdout();

        for attr in styled_object.object_style.attrs.iter() {
            write_cout!(&format!(csi!("{}m"), *attr as i16));
            reset = true;
        }

        write!(stdout, "{}", styled_object.content)?;

        if reset {
            colored_terminal.reset()?;
        }

        Ok(())
    }
}