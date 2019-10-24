//! This is an `ANSI escape code` specific implementation for terminal related action.
//! This module is used for windows 10 terminals and unix terminals by default.

use crate::utils::Result;
use crate::{csi, cursor::MoveTo, write_cout};

use super::{super::sys::get_terminal_size, ClearType, Terminal};

pub(crate) static CLEAR_ALL_CSI_SEQUENCE: &'static str = csi!("2J");
pub(crate) static CLEAR_FROM_CURSOR_DOWN_CSI_SEQUENCE: &'static str = csi!("J");
pub(crate) static CLEAR_FROM_CURSOR_UP_CSI_SEQUENCE: &'static str = csi!("1J");
pub(crate) static CLEAR_FROM_CURRENT_LINE_CSI_SEQUENCE: &'static str = csi!("2K");
pub(crate) static CLEAR_UNTIL_NEW_LINE_CSI_SEQUENCE: &'static str = csi!("K");

pub(crate) fn scroll_up_csi_sequence(count: u16) -> String {
    format!(csi!("{}S"), count)
}

pub(crate) fn scroll_down_csi_sequence(count: u16) -> String {
    format!(csi!("{}T"), count)
}

pub(crate) fn set_size_csi_sequence(width: u16, height: u16) -> String {
    format!(csi!("8;{};{}t"), height, width)
}

/// This struct is an ansi escape code implementation for terminal related actions.
pub(crate) struct AnsiTerminal;

impl AnsiTerminal {
    pub(crate) fn new() -> AnsiTerminal {
        AnsiTerminal
    }
}

impl Terminal for AnsiTerminal {
    fn clear(&self, clear_type: ClearType) -> Result<()> {
        match clear_type {
            ClearType::All => write_cout!(CLEAR_ALL_CSI_SEQUENCE)?,
            ClearType::FromCursorDown => write_cout!(CLEAR_FROM_CURSOR_DOWN_CSI_SEQUENCE)?,
            ClearType::FromCursorUp => write_cout!(CLEAR_FROM_CURSOR_UP_CSI_SEQUENCE)?,
            ClearType::CurrentLine => write_cout!(CLEAR_FROM_CURRENT_LINE_CSI_SEQUENCE)?,
            ClearType::UntilNewLine => write_cout!(CLEAR_UNTIL_NEW_LINE_CSI_SEQUENCE)?,
        };

        if clear_type == ClearType::All {
            write_cout!(MoveTo(0, 0))?;
        }

        Ok(())
    }

    fn size(&self) -> Result<(u16, u16)> {
        get_terminal_size()
    }

    fn scroll_up(&self, count: u16) -> Result<()> {
        write_cout!(scroll_up_csi_sequence(count))?;
        Ok(())
    }

    fn scroll_down(&self, count: u16) -> Result<()> {
        write_cout!(scroll_down_csi_sequence(count))?;
        Ok(())
    }

    fn set_size(&self, width: u16, height: u16) -> Result<()> {
        write_cout!(set_size_csi_sequence(width, height))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::{thread, time};

    use super::{AnsiTerminal, Terminal};

    // TODO - Test is disabled, because it's failing on Travis CI
    #[test]
    #[ignore]
    fn test_resize_ansi() {
        if try_enable_ansi() {
            let terminal = AnsiTerminal::new();

            let (width, height) = terminal.size().unwrap();

            terminal.set_size(35, 35).unwrap();
            // see issue: https://github.com/eminence/terminal-size/issues/11
            thread::sleep(time::Duration::from_millis(30));
            assert_eq!((35, 35), terminal.size().unwrap());

            // reset to previous size
            terminal.set_size(width, height).unwrap();
            // see issue: https://github.com/eminence/terminal-size/issues/11
            thread::sleep(time::Duration::from_millis(30));
            assert_eq!((width, height), terminal.size().unwrap());
        }
    }

    fn try_enable_ansi() -> bool {
        #[cfg(windows)]
        {
            if cfg!(target_os = "windows") {
                use crate::utils::sys::winapi::ansi::set_virtual_terminal_processing;

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
