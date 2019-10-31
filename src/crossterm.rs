/// A crossterm functionality wrapper.
pub struct Crossterm;

impl Crossterm {
    /// Creates a new `Crossterm`.
    pub fn new() -> Crossterm {
        Crossterm
    }

    /// Creates a new `TerminalInput`.
    #[cfg(feature = "input")]
    pub fn input(&self) -> crate::input::TerminalInput {
        crate::input::TerminalInput::new()
    }
}
