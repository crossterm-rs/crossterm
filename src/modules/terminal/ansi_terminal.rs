//! This is an `ANSI escape code` specific implementation for terminal related action.
//! This module is used for windows 10 terminals and unix terminals by default.

use super::*;

/// This struct is an ansi escape code implementation for terminal related actions.
pub struct AnsiTerminal;
use cursor::TerminalCursor;

impl AnsiTerminal {
    pub fn new() -> AnsiTerminal {
        AnsiTerminal {}
    }
}

impl ITerminal for AnsiTerminal {
    fn clear(&self, clear_type: ClearType, stdout: &Arc<TerminalOutput>) {
        match clear_type {
            ClearType::All => {
                stdout.write_str(csi!("2J"));
                TerminalCursor::new(stdout).goto(0,0);
            }
            ClearType::FromCursorDown => {
                stdout.write_str(csi!("J"));
            }
            ClearType::FromCursorUp => {
                stdout.write_str(csi!("1J"));
            }
            ClearType::CurrentLine => {
                stdout.write_str(csi!("2K"));
            }
            ClearType::UntilNewLine => {
                stdout.write_str(csi!("K"));
            }
        };
    }

    fn terminal_size(&self, stdout: &Arc<TerminalOutput>) -> (u16, u16) {
        functions::get_terminal_size()
    }

    fn scroll_up(&self, count: i16, stdout: &Arc<TerminalOutput>) {
        stdout.write_string(format!(csi!("{}S"), count));
    }

    fn scroll_down(&self, count: i16, stdout: &Arc<TerminalOutput>) {
        stdout.write_string(format!(csi!("{}T"), count));
    }

    fn set_size(&self, width: i16, height: i16, stdout: &Arc<TerminalOutput>) {
        stdout.write_string(format!(csi!("8;{};{}t"), width, height));
    }

    fn exit(&self,stdout: &Arc<TerminalOutput>) {
        // drop the screen with the current stdout. This will make sure when in raw mode this will be disabled first.
        let screen = Screen::from(stdout.clone());
        drop(screen);
        functions::exit_terminal();
    }
}
