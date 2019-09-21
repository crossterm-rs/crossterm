//! This is an ANSI specific implementation for cursor related action.
//! This module is used for windows 10 terminals and UNIX terminals by default.
//! Note that the cursor position is 0 based. This means that we start counting at 0 when setting the cursor position etc.

use crossterm_utils::{csi, write_cout, Result};

use crate::sys::{get_cursor_position, show_cursor};

use super::ITerminalCursor;

pub fn get_goto_ansi(x: u16, y: u16) -> String {
    format!(csi!("{};{}H"), y + 1, x + 1)
}

pub fn get_move_up_ansi(count: u16) -> String {
    format!(csi!("{}A"), count)
}

pub fn get_move_right_ansi(count: u16) -> String {
    format!(csi!("{}C"), count)
}

pub fn get_move_down_ansi(count: u16) -> String {
    format!(csi!("{}B"), count)
}

pub fn get_move_left_ansi(count: u16) -> String {
    format!(csi!("{}D"), count)
}

pub static SAVE_POS_ANSI: &'static str = csi!("s");
pub static RESTORE_POS_ANSI: &'static str = csi!("u");
pub static HIDE_ANSI: &'static str = csi!("?25l");
pub static SHOW_ANSI: &'static str = csi!("?25h");
pub static BLINK_ON_ANSI: &'static str = csi!("?12h");
pub static BLINK_OFF_ANSI: &'static str = csi!("?12l");

/// This struct is an ANSI implementation for cursor related actions.
pub struct AnsiCursor;

impl AnsiCursor {
    pub fn new() -> AnsiCursor {
        AnsiCursor
    }
}

impl ITerminalCursor for AnsiCursor {
    fn goto(&self, x: u16, y: u16) -> Result<()> {
        write_cout!(get_goto_ansi(x, y))?;
        Ok(())
    }

    fn pos(&self) -> Result<(u16, u16)> {
        get_cursor_position()
    }

    fn move_up(&self, count: u16) -> Result<()> {
        write_cout!(get_move_up_ansi(count))?;
        Ok(())
    }

    fn move_right(&self, count: u16) -> Result<()> {
        write_cout!(get_move_right_ansi(count))?;
        Ok(())
    }

    fn move_down(&self, count: u16) -> Result<()> {
        write_cout!(get_move_down_ansi(count))?;
        Ok(())
    }

    fn move_left(&self, count: u16) -> Result<()> {
        write_cout!(get_move_left_ansi(count))?;
        Ok(())
    }

    fn save_position(&self) -> Result<()> {
        write_cout!(SAVE_POS_ANSI)?;
        Ok(())
    }

    fn restore_position(&self) -> Result<()> {
        write_cout!(RESTORE_POS_ANSI)?;
        Ok(())
    }

    fn hide(&self) -> Result<()> {
        show_cursor(false)?;
        Ok(())
    }

    fn show(&self) -> Result<()> {
        show_cursor(true)?;
        Ok(())
    }

    fn blink(&self, blink: bool) -> Result<()> {
        if blink {
            write_cout!(BLINK_ON_ANSI)?;
        } else {
            write_cout!(BLINK_OFF_ANSI)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{AnsiCursor, ITerminalCursor};

    // TODO - Test is ingored, because it's stalled on Travis CI
    #[test]
    #[ignore]
    fn test_ansi_save_restore_position() {
        if try_enable_ansi() {
            let cursor = AnsiCursor::new();

            let (saved_x, saved_y) = cursor.pos().unwrap();

            cursor.save_position().unwrap();
            cursor.goto(saved_x + 1, saved_y + 1).unwrap();
            cursor.restore_position().unwrap();

            let (x, y) = cursor.pos().unwrap();

            assert_eq!(x, saved_x);
            assert_eq!(y, saved_y);
        }
    }

    // TODO - Test is ingored, because it's stalled on Travis CI
    #[test]
    #[ignore]
    fn test_ansi_goto() {
        if try_enable_ansi() {
            let cursor = AnsiCursor::new();

            let (saved_x, saved_y) = cursor.pos().unwrap();

            cursor.goto(saved_x + 1, saved_y + 1).unwrap();
            assert_eq!(cursor.pos().unwrap(), (saved_x + 1, saved_y + 1));

            cursor.goto(saved_x, saved_y).unwrap();
            assert_eq!(cursor.pos().unwrap(), (saved_x, saved_y));
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
