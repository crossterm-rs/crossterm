//! This is an `ANSI escape code` specific implementation for terminal related action.
//! This module is used for windows 10 terminals and unix terminals by default.

use crossterm_cursor::TerminalCursor;
use crossterm_utils::{csi, write_cout, Result};

use crate::{sys::get_terminal_size, ClearType};

use super::ITerminal;

pub static CLEAR_ALL: &'static str = csi!("2J");
pub static CLEAR_FROM_CURSOR_DOWN: &'static str = csi!("J");
pub static CLEAR_FROM_CURSOR_UP: &'static str = csi!("1J");
pub static CLEAR_FROM_CURRENT_LINE: &'static str = csi!("2K");
pub static CLEAR_UNTIL_NEW_LINE: &'static str = csi!("K");

#[inline]
pub fn get_scroll_up_ansi(count: i16) -> String {
    format!(csi!("{}S"), count)
}

#[inline]
pub fn get_scroll_down_ansi(count: i16) -> String {
    format!(csi!("{}T"), count)
}

#[inline]
pub fn get_set_size_ansi(width: i16, height: i16) -> String {
    format!(csi!("8;{};{}t"), height, width)
}

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
                write_cout!(CLEAR_ALL)?;
                TerminalCursor::new().goto(0, 0)?;
            }
            ClearType::FromCursorDown => {
                write_cout!(CLEAR_FROM_CURSOR_DOWN)?;
            }
            ClearType::FromCursorUp => {
                write_cout!(CLEAR_FROM_CURSOR_UP)?;
            }
            ClearType::CurrentLine => {
                write_cout!(CLEAR_FROM_CURRENT_LINE)?;
            }
            ClearType::UntilNewLine => {
                write_cout!(CLEAR_UNTIL_NEW_LINE)?;
            }
        };
        Ok(())
    }

    fn terminal_size(&self) -> (u16, u16) {
        get_terminal_size()
    }

    fn scroll_up(&self, count: i16) -> Result<()> {
        write_cout!(get_scroll_up_ansi(count))?;
        Ok(())
    }

    fn scroll_down(&self, count: i16) -> Result<()> {
        write_cout!(get_scroll_down_ansi(count))?;
        Ok(())
    }

    fn set_size(&self, width: i16, height: i16) -> Result<()> {
        write_cout!(get_set_size_ansi(width, height))?;
        Ok(())
    }
}
