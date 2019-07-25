//! This is an ANSI specific implementation for cursor related action.
//! This module is used for windows 10 terminals and UNIX terminals by default.
//! Note that the cursor position is 0 based. This means that we start counting at 0 when setting the cursor position etc.

use super::ITerminalCursor;
use crate::sys::{get_cursor_position, show_cursor};
use std::io::Write;

use crossterm_utils::{write_cout, ErrorKind, Result};
use std::intrinsics::transmute;

#[inline]
pub fn get_goto_ansi(x: u16, y: u16) -> String {
    format!(csi!("{};{}H"), y + 1, x + 1)
}
#[inline]
pub fn get_move_up_ansi(count: u16) -> String {
    format!(csi!("{}A"), count)
}
#[inline]
pub fn get_move_right_ansi(count: u16) -> String {
    format!(csi!("{}C"), count)
}
#[inline]
pub fn get_move_down_ansi(count: u16) -> String {
    format!(csi!("{}B"), count)
}
#[inline]
pub fn get_move_left_ansi(count: u16) -> String {
    format!(csi!("{}D"), count)
}

pub static SAFE_POS_ANSI: &'static str = csi!("s");
pub static RESET_POS_ANSI: &'static str = csi!("u");
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

    fn pos(&self) -> (u16, u16) {
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
        write_cout!(SAFE_POS_ANSI)?;
        Ok(())
    }

    fn reset_position(&self) -> Result<()> {
        write_cout!(RESET_POS_ANSI)?;
        Ok(())
    }

    fn hide(&self) -> Result<()> {
        show_cursor(false);
        Ok(())
    }

    fn show(&self) -> Result<()> {
        show_cursor(true);
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
