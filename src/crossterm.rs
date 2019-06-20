use std::fmt::Display;

/// This type offers a easy way to use functionalities like `cursor, terminal, color, input, styling`.
///
/// To get a cursor instance to perform cursor related actions, you can do the following:
///
/// ```rust
/// let crossterm = Crossterm::new();
/// let cursor = crossterm.cursor();
//  let color = crossterm.color();
//  let terminal = crossterm.terminal();
//  let terminal = crossterm.input();
//  let style = crossterm
//        .style(format!("{} {}", 0, "Black text on green background"))
//        .with(Color::Black)
//        .on(Color::Green);
/// ```
///
/// # Remark
/// - depending on the feature flags you've enabled you are able to call methods of this type.
/// - checkout the crossterm book for more information about feature flags or alternate screen.
pub struct Crossterm;

impl Crossterm {
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
    /// // print the styled text * times to the current screen.
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
}
