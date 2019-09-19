pub mod ansi {
    use winapi::um::wincon::ENABLE_VIRTUAL_TERMINAL_PROCESSING;

    use crate::Result;
    use crossterm_winapi::ConsoleMode;

    /// Toggle virtual terminal processing.
    ///
    /// This method attempts to toggle virtual terminal processing for this
    /// console. If there was a problem toggling it, then an error returned.
    /// On success, the caller may assume that toggling it was successful.
    ///
    /// When virtual terminal processing is enabled, characters emitted to the
    /// console are parsed for VT100 and similar control character sequences
    /// that control color and other similar operations.
    pub fn set_virtual_terminal_processing(yes: bool) -> Result<()> {
        let mask = ENABLE_VIRTUAL_TERMINAL_PROCESSING;

        let console_mode = ConsoleMode::new()?;
        let old_mode = console_mode.mode()?;

        let new_mode = if yes {
            old_mode | mask
        } else {
            old_mode & !mask
        };

        if old_mode != new_mode {
            console_mode.set_mode(new_mode)?;
        }

        Ok(())
    }
}
