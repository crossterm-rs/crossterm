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
    fn clear(&self, clear_type: ClearType, stdout: &Option<&Arc<TerminalOutput>>) {
        match clear_type {
            ClearType::All => {
                functions::write_str(&stdout,csi!("2J"));
                TerminalCursor::new().goto(0,0);
            }
            ClearType::FromCursorDown => {
                functions::write_str(&stdout,csi!("J"));
            }
            ClearType::FromCursorUp => {
                functions::write_str(&stdout,csi!("1J"));
            }
            ClearType::CurrentLine => {
                functions::write_str(&stdout,csi!("2K"));
            }
            ClearType::UntilNewLine => {
                functions::write_str(&stdout,csi!("K"));
            }
        };
    }

    fn terminal_size(&self, _stdout: &Option<&Arc<TerminalOutput>>) -> (u16, u16) {
        functions::get_terminal_size()
    }

    fn scroll_up(&self, count: i16, stdout: &Option<&Arc<TerminalOutput>>) {
        functions::write(&stdout,format!(csi!("{}S"), count));
    }

    fn scroll_down(&self, count: i16, stdout: &Option<&Arc<TerminalOutput>>) {
        functions::write(&stdout,format!(csi!("{}T"), count));
    }

    fn set_size(&self, width: i16, height: i16, stdout: &Option<&Arc<TerminalOutput>>) {
        functions::write(&stdout,format!(csi!("8;{};{}t"), height, width));
    }

    fn exit(&self,stdout: &Option<&Arc<TerminalOutput>>) {
        if let Some(output) = stdout {
            // drop the screen with the current stdout. This will make sure when in raw mode this will be disabled first.
            let screen = Screen::from(output.to_owned().clone());
            drop(screen);
            functions::exit_terminal();
        }
    }
}
