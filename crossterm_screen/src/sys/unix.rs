use crossterm_utils::Result;

/// This command is used for enabling and disabling raw mode for the terminal.
pub struct RawModeCommand;

impl RawModeCommand {
    pub fn new() -> Self {
        RawModeCommand
    }

    /// Enables raw mode.
    pub fn enable(&mut self) -> Result<()> {
        crossterm_utils::sys::unix::enable_raw_mode()?;
        Ok(())
    }

    /// Disables raw mode.
    pub fn disable(&mut self) -> Result<()> {
        crossterm_utils::sys::unix::disable_raw_mode()?;
        Ok(())
    }
}
