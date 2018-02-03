/// Enum that can be used for the kind of clearing that can be done in the terminal.
pub enum ClearType {
    All,
    FromCursorDown,
    FromCursorUp,
    CurrentLine,
    UntilNewLine,
}

pub trait ITerminal {
    /// Clear the current cursor by specifying the clear type
    fn clear(&self, clear_type: ClearType);
    /// Get the terminal size (x,y)
    fn terminal_size(&self) -> (u16, u16);
    /// Scroll `n` lines up in the current terminal.
    fn scroll_up(&self, count: i16);
    /// Scroll `n` lines down in the current terminal.
    fn scroll_down(&self, count: i16);
    /// Resize terminal to the given width and height.
    fn set_size(&self,width: i16, height: i16);
}
