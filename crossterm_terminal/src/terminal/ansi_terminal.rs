//! This is an `ANSI escape code` specific implementation for terminal related action.
//! This module is used for windows 10 terminals and unix terminals by default.

use super::ITerminal;
use crate::{sys::get_terminal_size, ClearType};
use crossterm_cursor::TerminalCursor;
use crossterm_utils::Result;
use std::io::Write;
use std::sync::Arc;

/// This struct is an ansi escape code implementation for terminal related actions.
pub struct AnsiTerminal;

impl AnsiTerminal {
    pub fn new() -> AnsiTerminal {
        AnsiTerminal
    }
}

impl ITerminal for AnsiTerminal {
    fn clear(&self, clear_type: ClearType) -> Result<()> {
        match clear_type {
            ClearType::All => {
                write_cout!( csi!("2J"));
                TerminalCursor::new().goto(0, 0);
            }
            ClearType::FromCursorDown => {
                write_cout!( csi!("J"));
            }
            ClearType::FromCursorUp => {
                write_cout!( csi!("1J"));
            }
            ClearType::CurrentLine => {
                write_cout!( csi!("2K"));
            }
            ClearType::UntilNewLine => {
                write_cout!( csi!("K"));
            }
        };
        Ok(())
    }

    fn terminal_size(&self) -> (u16, u16) {
        get_terminal_size()
    }

    fn scroll_up(&self, count: i16) -> Result<()> {
        write_cout!( &format!(csi!("{}S"), count));
        Ok(())
    }

    fn scroll_down(&self, count: i16) -> Result<()> {
        write_cout!( &format!(csi!("{}T"), count));
        Ok(())
    }

    fn set_size(
        &self,
        width: i16,
        height: i16,
    ) -> Result<()> {
        write_cout!( &format!(csi!("8;{};{}t"), height, width));
        Ok(())
    }
}
