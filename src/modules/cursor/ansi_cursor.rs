//! This is an ANSI specific implementation for cursor related action.
//! This module is used for windows 10 terminals and unix terminals by default.
//! Note that the cursor position is 0 based. This means that we start counting at 0 when setting the cursor position ect.

use super::*;

/// This struct is an ansi implementation for cursor related actions.
pub struct AnsiCursor { }

impl AnsiCursor
{
    pub fn new() -> Box<AnsiCursor> {
        Box::from(AnsiCursor { })
    }
}

impl ITerminalCursor for AnsiCursor {
    fn goto(&self, x: u16, y: u16, stdout: &Option<&Arc<TerminalOutput>>) {
        functions::write(stdout, format!(csi!("{};{}H"), y + 1, x + 1));
    }

    fn pos(&self) -> (u16, u16) {
        functions::get_cursor_position()
    }

    fn move_up(&self, count: u16, stdout: &Option<&Arc<TerminalOutput>>) {
        functions::write(stdout, format!(csi!("{}A"), count));
    }

    fn move_right(&self, count: u16, stdout: &Option<&Arc<TerminalOutput>>) {
        functions::write(stdout, format!(csi!("{}C"), count));
    }

    fn move_down(&self, count: u16, stdout: &Option<&Arc<TerminalOutput>>) {
        functions::write(stdout, format!(csi!("{}B"), count));
    }

    fn move_left(&self, count: u16, stdout: &Option<&Arc<TerminalOutput>>) {
        functions::write(stdout, format!(csi!("{}D"), count));
    }

    fn save_position(&self, stdout: &Option<&Arc<TerminalOutput>>) {
        functions::write_str(stdout, csi!("s"));
    }

    fn reset_position(&self, stdout: &Option<&Arc<TerminalOutput>>) {
        functions::write_str(stdout, csi!("u"));
    }

    fn hide(&self, stdout: &Option<&Arc<TerminalOutput>>) {
        functions::write_str(stdout, csi!("?25l"));
    }

    fn show(&self, stdout: &Option<&Arc<TerminalOutput>>) {
        functions::write_str(stdout, csi!("?25h"));
    }

    fn blink(&self, blink: bool, stdout: &Option<&Arc<TerminalOutput>>) {
        if blink {
            functions::write_str(stdout, csi!("?12h"));
        } else {
            functions::write_str(stdout, csi!("?12l"));
        }
    }
}
