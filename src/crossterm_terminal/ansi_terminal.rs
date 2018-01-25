use std::io;
use std::io::Write;

use Construct;
use super::base_terminal::{ClearType, ITerminal};
use kernel::linux_kernel::terminal::*;

/// This struct is an ansi implementation for terminal related actions.
pub struct UnixTerminal;

impl Construct for UnixTerminal {
    fn new() -> Box<UnixTerminal> {
        Box::from(UnixTerminal {})
    }
}

impl ITerminal for UnixTerminal {
    fn clear(&self, clear_type: ClearType) {
        let mut some_writer = io::stdout();
        
        match clear_type {
            ClearType::All => {                
                write!(&mut some_writer, csi!("2J"));
            },
            ClearType::FromCursorDown=> {
                write!(&mut some_writer, csi!("J"));
            },
            ClearType::FromCursorUp => {
                write!(&mut some_writer, csi!("1J"));
            },
            ClearType::CurrentLine => {
                write!(&mut some_writer, csi!("2K"));
            },
            ClearType::UntilNewLine => {
                write!(&mut some_writer, csi!("K"));
            },
        };
    }

    fn terminal_size(&self) -> Option<(u16, u16)> {
        terminal_size()
    }

    fn scroll_up(&self, count: i16) {
        let mut some_writer = io::stdout();
        write!(&mut some_writer, csi!("{}S"), count);
    }

    fn scroll_down(&self, count: i16) {
        let mut some_writer = io::stdout();
        write!(&mut some_writer, csi!("{}T"), count);
    }

    fn set_size(width: i16, height: i16) {
        let mut some_writer = io::stdout();
        write!(&mut some_writer, csi!("8;{};{}t"), width, height);
    }
}
