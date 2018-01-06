pub enum ClearType {
    All,
    AfterCursor,
    BeforeCursor,
    CurrentLine,
    UntilNewLine,
}

pub trait ITerminal {
    fn clear(&self, clear_type: ClearType);
    fn terminal_size(&self) -> Option<(u16, u16)>;
    fn scroll_up(&self, count: i16);
    fn scroll_down(&self, count: u16);
}
