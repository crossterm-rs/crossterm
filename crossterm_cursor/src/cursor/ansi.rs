//! This is an ANSI specific implementation for cursor related action.
//! This module is used for windows 10 terminals and UNIX terminals by default.
//! Note that the cursor position is 0 based. This means that we start counting at 0 when setting the cursor position etc.

use crossterm_utils::{csi, write_cout, Result};

use crate::sys::{get_cursor_position, show_cursor};

use super::Cursor;

pub(crate) fn get_goto_code(x: u16, y: u16) -> String {
    format!(csi!("{};{}H"), y + 1, x + 1)
}

pub(crate) fn get_move_up_code(count: u16) -> String {
    format!(csi!("{}A"), count)
}

pub(crate) fn get_move_right_code(count: u16) -> String {
    format!(csi!("{}C"), count)
}

pub(crate) fn get_move_down_code(count: u16) -> String {
    format!(csi!("{}B"), count)
}

pub(crate) fn get_move_left_code(count: u16) -> String {
    format!(csi!("{}D"), count)
}

pub(crate) static SAVE_POS_CODE: &'static str = csi!("s");
pub(crate) static RESTORE_POS_CODE: &'static str = csi!("u");
pub(crate) static HIDE_CODE: &'static str = csi!("?25l");
pub(crate) static SHOW_CODE: &'static str = csi!("?25h");
pub(crate) static BLINK_ON_CODE: &'static str = csi!("?12h");
pub(crate) static BLINK_OFF_CODE: &'static str = csi!("?12l");

/// This struct is an ANSI implementation for cursor related actions.
pub(crate) struct AnsiCursor;

impl AnsiCursor {
    pub(crate) fn new() -> AnsiCursor {
        AnsiCursor
    }
}

impl Cursor for AnsiCursor {
    fn goto(&self, x: u16, y: u16) -> Result<()> {
        write_cout!(get_goto_code(x, y))?;
        Ok(())
    }

    fn pos(&self) -> Result<(u16, u16)> {
        get_cursor_position()
    }

    fn move_up(&self, count: u16) -> Result<()> {
        write_cout!(get_move_up_code(count))?;
        Ok(())
    }

    fn move_right(&self, count: u16) -> Result<()> {
        write_cout!(get_move_right_code(count))?;
        Ok(())
    }

    fn move_down(&self, count: u16) -> Result<()> {
        write_cout!(get_move_down_code(count))?;
        Ok(())
    }

    fn move_left(&self, count: u16) -> Result<()> {
        write_cout!(get_move_left_code(count))?;
        Ok(())
    }

    fn save_position(&self) -> Result<()> {
        write_cout!(SAVE_POS_CODE)?;
        Ok(())
    }

    fn restore_position(&self) -> Result<()> {
        write_cout!(RESTORE_POS_CODE)?;
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
            write_cout!(BLINK_ON_CODE)?;
        } else {
            write_cout!(BLINK_OFF_CODE)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{AnsiCursor, Cursor};

    // TODO - Test is ingored, because it's stalled on Travis CI
    #[test]
    #[ignore]
    fn test_save_restore_position() {
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
    fn test_goto() {
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
