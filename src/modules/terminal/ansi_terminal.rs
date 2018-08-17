//! This is an `ANSI escape code` specific implementation for terminal related action.
//! This module is used for windows 10 terminals and unix terminals by default.

use super::super::cursor::cursor;
use super::*;

/// This struct is an ansi escape code implementation for terminal related actions.
pub struct AnsiTerminal;

impl AnsiTerminal {
    pub fn new() -> AnsiTerminal {
        AnsiTerminal {}
    }
}

impl ITerminal for AnsiTerminal {
    fn clear(&self, clear_type: ClearType, screen_manager: &Arc<Stdout>) {
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

    fn terminal_size(&self, screen_manager: &Arc<Stdout>) -> (u16, u16) {
        functions::get_terminal_size()
    }

    fn scroll_up(&self, count: i16, screen_manager: &Arc<Stdout>) {
        screen_manager.write_string(format!(csi!("{}S"), count));
    }

    fn scroll_down(&self, count: i16, screen_manager: &Arc<Stdout>) {
        screen_manager.write_string(format!(csi!("{}T"), count));
    }

    fn set_size(&self, width: i16, height: i16, screen_manager: &Arc<Stdout>) {
        screen_manager.write_string(format!(csi!("8;{};{}t"), width, height));
    }

    fn exit(&self, screen_manager: &Arc<Stdout>) {
        // drop the screen with the current stdout. This will make sure when in raw mode this will be disabled first.
        let mut screen = Screen::from(screen_manager.clone());
        drop(screen);
        functions::exit_terminal();
    }
}
