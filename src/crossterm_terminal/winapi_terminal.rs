use Construct;
use super::base_terminal::{ClearType, ITerminal};

use kernel::windows_kernel::terminal;

/// This struct is an windows implementation for terminal related actions.
pub struct WinApiTerminal;

impl Construct for WinApiTerminal {
    fn new() -> Box<WinApiTerminal> {
        Box::from(WinApiTerminal {})
    }
}

impl ITerminal for WinApiTerminal {
    fn clear(&self, clear_type: ClearType) {
        println! ("Windows!!!");
        match clear_type
        {
            ClearType::All => terminal::clear_entire_screen(),
            ClearType::FromCursorDown => terminal::clear_after_cursor(),            
            ClearType::FromCursorUp => terminal::clear_before_cursor(),
            ClearType::CurrentLine => terminal::clear_current_line(),
            ClearType::UntilNewLine => terminal::clear_until_line(),
        };
    }

    fn terminal_size(&self) -> (u16, u16) {
        terminal::terminal_size()
    }

    fn scroll_up(&self, count: i16) {
        // yet to be inplemented
    }

    fn scroll_down(&self, count: i16) {
        terminal::scroll_down(count as i16);
    }

    fn set_size(&self, width: i16, height: i16) { terminal::resize_terminal(width,height); }
}
