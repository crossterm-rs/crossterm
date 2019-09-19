//! This is an `ANSI escape code` specific implementation for terminal related action.
//! This module is used for windows 10 terminals and unix terminals by default.

use crossterm_cursor::TerminalCursor;
use crossterm_utils::{csi, write_cout, Result};

use crate::sys::get_terminal_size;

use super::{ClearType, ITerminal};

pub static CLEAR_ALL: &'static str = csi!("2J");
pub static CLEAR_FROM_CURSOR_DOWN: &'static str = csi!("J");
pub static CLEAR_FROM_CURSOR_UP: &'static str = csi!("1J");
pub static CLEAR_FROM_CURRENT_LINE: &'static str = csi!("2K");
pub static CLEAR_UNTIL_NEW_LINE: &'static str = csi!("K");

pub fn get_scroll_up_ansi(count: u16) -> String {
    format!(csi!("{}S"), count)
}

pub fn get_scroll_down_ansi(count: u16) -> String {
    format!(csi!("{}T"), count)
}

pub fn get_set_size_ansi(width: u16, height: u16) -> String {
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

    fn size(&self) -> Result<(u16, u16)> {
        get_terminal_size()
    }

    fn scroll_up(&self, count: u16) -> Result<()> {
        write_cout!(get_scroll_up_ansi(count))?;
        Ok(())
    }

    fn scroll_down(&self, count: u16) -> Result<()> {
        write_cout!(get_scroll_down_ansi(count))?;
        Ok(())
    }

    fn set_size(&self, width: u16, height: u16) -> Result<()> {
        write_cout!(get_set_size_ansi(width, height))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::{thread, time};

    use super::{AnsiTerminal, ITerminal};

    /* ======================== ANSI =========================== */
    // TODO - Test is disabled, because it's failing on Travis CI
    #[test]
    #[ignore]
    fn resize_ansi() {
        if try_enable_ansi() {
            let terminal = AnsiTerminal::new();

            assert!(terminal.set_size(50, 50).is_ok());

            // see issue: https://github.com/eminence/terminal-size/issues/11
            thread::sleep(time::Duration::from_millis(30));

            let size = terminal.size();
            assert!(size.is_ok());
            let (x, y) = size.unwrap();

            assert_eq!(x, 50);
            assert_eq!(y, 50);
        }
    }

    fn try_enable_ansi() -> bool {
        #[cfg(windows)]
        {
            if cfg!(target_os = "windows") {
                use crossterm_utils::sys::winapi::ansi::set_virtual_terminal_processing;

                // if it is not listed we should try with WinApi to check if we do support ANSI-codes.
                match set_virtual_terminal_processing(true) {
                    Ok(_) => return true,
                    Err(_) => return false,
                }
            }
        }

        true
    }

}
