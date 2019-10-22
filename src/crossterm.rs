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
    pub fn cursor(&self) -> crate::cursor::TerminalCursor {
        crate::cursor::TerminalCursor::new()
    }

    /// Creates a new `TerminalInput`.
    #[cfg(feature = "input")]
    pub fn input(&self) -> crate::input::TerminalInput {
        crate::input::TerminalInput::new()
    }

    /// Creates a new `Terminal`.
    #[cfg(feature = "terminal")]
    pub fn terminal(&self) -> crate::terminal::Terminal {
        crate::terminal::Terminal::new()
    }

    /// Creates a new `TerminalColor`.
    #[cfg(feature = "style")]
    pub fn color(&self) -> crate::style::TerminalColor {
        crate::style::TerminalColor::new()
    }

    /// Creates a new `StyledObject`.
    #[cfg(feature = "style")]
    pub fn style<D>(&self, val: D) -> crate::style::StyledObject<D>
    where
        D: Display + Clone,
    {
        crate::style::ObjectStyle::new().apply_to(val)
    }
}
