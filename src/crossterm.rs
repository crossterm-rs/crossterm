use std::fmt::Display;

// TODO Should be removed? This adds just another way to achieve the same thing.
/// A crossterm functionality wrapper.
pub struct Crossterm;

impl Crossterm {
    /// Creates a new `Crossterm`.
    pub fn new() -> Crossterm {
        Crossterm
    }

    /// Crates a new `TerminalCursor`.
    #[cfg(feature = "cursor")]
    pub fn cursor(&self) -> crossterm_cursor::TerminalCursor {
        crossterm_cursor::TerminalCursor::new()
    }

    /// Creates a new `TerminalInput`.
    #[cfg(feature = "input")]
    pub fn input(&self) -> crossterm_input::TerminalInput {
        crossterm_input::TerminalInput::new()
    }

    /// Creates a new `Terminal`.
    #[cfg(feature = "terminal")]
    pub fn terminal(&self) -> crossterm_terminal::Terminal {
        crossterm_terminal::Terminal::new()
    }

    /// Creates a new `TerminalColor`.
    #[cfg(feature = "style")]
    pub fn color(&self) -> crossterm_style::TerminalColor {
        crossterm_style::TerminalColor::new()
    }

    /// Creates a new `StyledObject`.
    #[cfg(feature = "style")]
    pub fn style<D>(&self, val: D) -> crossterm_style::StyledObject<D>
    where
        D: Display + Clone,
    {
        crossterm_style::ObjectStyle::new().apply_to(val)
    }
}
