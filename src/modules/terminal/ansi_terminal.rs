//! This is an `ANSI escape code` specific implementation for terminal related action.
//! This module is used for windows 10 terminals and unix terminals by default.

use super::super::cursor::cursor;
use super::{functions, ClearType, ITerminal, ScreenManager};

/// This struct is an ansi escape code implementation for terminal related actions.
pub struct AnsiTerminal;

impl AnsiTerminal {
    pub fn new() -> AnsiTerminal {
        AnsiTerminal {}
    }
}

impl ITerminal for AnsiTerminal {
    fn clear(&self, clear_type: ClearType, screen_manager: &ScreenManager) {
        match clear_type {
            ClearType::All => {
                screen_manager.write_str(csi!("2J"));
            }
            ClearType::FromCursorDown => {
                screen_manager.write_str(csi!("J"));
            }
            ClearType::FromCursorUp => {
                screen_manager.write_str(csi!("1J"));
            }
            ClearType::CurrentLine => {
                screen_manager.write_str(csi!("2K"));
            }
            ClearType::UntilNewLine => {
                screen_manager.write_str(csi!("K"));
            }
        };
    }

    fn terminal_size(&self, screen_manager: &ScreenManager) -> (u16, u16) {
        functions::get_terminal_size()
    }

    fn scroll_up(&self, count: i16, screen_manager: &ScreenManager) {
        screen_manager.write_string(format!(csi!("{}S"), count));
    }

    fn scroll_down(&self, count: i16, screen_manager: &ScreenManager) {
        screen_manager.write_string(format!(csi!("{}T"), count));
    }

    fn set_size(&self, width: i16, height: i16, screen_manager: &ScreenManager) {
        screen_manager.write_string(format!(csi!("8;{};{}t"), width, height));
    }

    fn exit(&self) {
        functions::exit_terminal();
    }
}
