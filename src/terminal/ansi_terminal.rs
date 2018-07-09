//! This is an `ANSI escape code` specific implementation for terminal related action.
//! This module is used for windows 10 terminals and unix terminals by default.

use super::{ClearType, ITerminal, Rc};
use super::super::cursor::cursor;
use shared::functions;
use Context;

/// This struct is an ansi implementation for terminal related actions.
pub struct AnsiTerminal {
    context: Rc<Context>,
}

impl AnsiTerminal {
    pub fn new(context: Rc<Context>) -> Box<AnsiTerminal> {
        Box::from(AnsiTerminal { context: context })
    }
}

impl ITerminal for AnsiTerminal {
    fn clear(&self, clear_type: ClearType) {
        let mut screen_manager = self.context.screen_manager.lock().unwrap();
        {
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
    }

    fn terminal_size(&self) -> (u16, u16) {
        functions::get_terminal_size(&self.context.screen_manager)
    }

    fn scroll_up(&self, count: i16) {
        let mut screen = self.context.screen_manager.lock().unwrap();
        {
            screen.write_string(format!(csi!("{}S"), count));
        }
    }

    fn scroll_down(&self, count: i16) {
        let mut screen = self.context.screen_manager.lock().unwrap();
        {
            screen.write_string(format!(csi!("{}T"), count));
        }
    }

    fn set_size(&self, width: i16, height: i16) {
        let mut screen = self.context.screen_manager.lock().unwrap();
        {
            screen.write_string(format!(csi!("8;{};{}t"), width, height));
        }
    }

    fn exit(&self) {
        functions::exit_terminal();
    }
}
