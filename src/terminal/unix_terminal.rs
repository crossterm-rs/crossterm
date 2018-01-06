use Construct;
use super::base_terminal::{ClearType, ITerminal};

#[cfg(unix)]
use kernel::linux_kernel::terminal::*;
#[cfg(windows)]
use kernel::windows_kernel::terminal::*;

/// This struct will be used for ansi terminals and unix systems.
pub struct UnixTerminal;

impl Construct for UnixTerminal {
    fn new() -> Box<UnixTerminal> {
        Box::from(UnixTerminal {})
    }
}

impl ITerminal for UnixTerminal {
    fn clear(&self, clear_type: ClearType) {
        match clear_type {
            ClearType::All => format!(csi!("2J")),
            ClearType::AfterCursor => format!(csi!("J")),
            ClearType::BeforeCursor => format!(csi!("1J")),
            ClearType::CurrentLine => format!(csi!("2K")),
            ClearType::UntilNewLine => format!(csi!("K")),
        };
    }

    fn terminal_size(&self) -> Option<(u16, u16)> {
        terminal_size()
    }

    fn scroll_up(&self, count: i16) {
        format!(csi!("{}S"), count);
    }

    fn scroll_down(&self, count: u16) {
        format!(csi!("{}T"), count);
    }
}
