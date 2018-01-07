use Construct;
use super::base_terminal::{ClearType, ITerminal};

#[cfg(windows)]
use kernel::windows_kernel::terminal;
/// This struct will be used for ansi terminals and unix systems.
pub struct WinApiTerminal;

impl Construct for WinApiTerminal {
    fn new() -> Box<WinApiTerminal> {
        Box::from(WinApiTerminal {})
    }
}

impl ITerminal for WinApiTerminal {
    fn clear(&self, clear_type: ClearType) {
        match clear_type
        {
            ClearType::All => terminal::clear_entire_screen(),
            ClearType::AfterCursor => terminal::clear_after_cursor(),            
            ClearType::BeforeCursor => terminal::clear_before_cursor(),
            ClearType::CurrentLine => terminal::clear_current_line(),
            ClearType::UntilNewLine => terminal::clear_until_line(),
        };
    }

    fn terminal_size(&self) -> Option<(u16, u16)> {
        terminal::terminal_size()
    }

    fn scroll_up(&self, count: i16) {
        // yet to be inplemented
    }

    fn scroll_down(&self, count: u16) {
        terminal::scroll_down(count as i16);
    }
}
