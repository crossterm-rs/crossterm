//! This is an ANSI specific implementation for cursor related action.
//! This module is used for windows 10 terminals and UNIX terminals by default.
//! Note that the cursor position is 0 based. This means that we start counting at 0 when setting the cursor position etc.

use super::ITerminalCursor;
use crate::sys::get_cursor_position;

use crossterm_utils::Result;
use std::sync::Arc;

/// This struct is an ANSI implementation for cursor related actions.
pub struct AnsiCursor;

impl AnsiCursor {
    pub fn new() -> AnsiCursor {
        AnsiCursor
    }
}

use std::io::Write;

impl ITerminalCursor for AnsiCursor {
    fn goto(&self, x: u16, y: u16) -> Result<()> {
        write_cout!(&format!(csi!("{};{}H"), y + 1, x + 1));
        Ok(())
    }

    fn pos(&self) -> (u16, u16) {
        get_cursor_position()
    }

    fn move_up(&self, count: u16) -> Result<()> {
        write_cout!( &format!(csi!("{}A"), count));
        Ok(())
    }

    fn move_right(&self, count: u16) -> Result<()> {
        write_cout!( &format!(csi!("{}C"), count));
        Ok(())
    }

    fn move_down(&self, count: u16) -> Result<()> {
        write_cout!( &format!(csi!("{}B"), count));
        Ok(())
    }

    fn move_left(&self, count: u16) -> Result<()> {
        write_cout!( &format!(csi!("{}D"), count));
        Ok(())
    }

    fn save_position(&self) -> Result<()> {
        write_cout!( csi!("s"));
        Ok(())
    }

    fn reset_position(&self) -> Result<()> {
        write_cout!( csi!("u"));
        Ok(())
    }

    fn hide(&self) -> Result<()> {
        write_cout!( csi!("?25l"));
        Ok(())
    }

    fn show(&self) -> Result<()> {
        write_cout!( csi!("?25h"));
        Ok(())
    }

    fn blink(&self, blink: bool) -> Result<()> {
        if blink {
            write_cout!( csi!("?12h"));
        } else {
            write_cout!( csi!("?12l"));
        }
        Ok(())
    }
}
