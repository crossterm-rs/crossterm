//! This is an ANSI specific implementation for cursor related action.
//! This module is used for windows 10 terminals and unix terminals by default.
//! Note that the cursor position is 0 based. This means that we start counting at 0 when setting the cursor position ect.

use super::*;
use shared::functions;
use Context;

/// This struct is an ansi implementation for cursor related actions.
pub struct AnsiCursor {
    context: Rc<Context>,
}

impl AnsiCursor {
    pub fn new(context: Rc<Context>) -> Box<AnsiCursor> {
        Box::from(AnsiCursor { context })
    }
}

impl ITerminalCursor for AnsiCursor {
    fn goto(&self, x: u16, y: u16) {
        let mut screen = self.context.screen_manager.lock().unwrap();
        {
            screen.write_string(format!(csi!("{};{}H"), y + 1, x + 1));
        }
    }

    fn pos(&self) -> (u16, u16) {
        functions::get_cursor_position(self.context.clone())
    }

    fn absolute_pos(&self) -> (u16, u16)
    {
        functions::get_absolute_cursor_pos(&self.context)
    }

    fn move_up(&self, count: u16) {
        let mut screen = self.context.screen_manager.lock().unwrap();
        {
            screen.write_string(format!(csi!("{}A"), count));
        }
    }

    fn move_right(&self, count: u16) {
        let mut screen = self.context.screen_manager.lock().unwrap();
        {
            screen.write_string(format!(csi!("{}C"), count));
        }
    }

    fn move_down(&self, count: u16) {
        let mut screen = self.context.screen_manager.lock().unwrap();
        {
            screen.write_string(format!(csi!("{}B"), count));
        }
    }

    fn move_left(&self, count: u16) {
        let mut screen = self.context.screen_manager.lock().unwrap();
        {
            screen.write_string(format!(csi!("{}D"), count));
        }
    }

    fn save_position(&self) {
        let mut screen = self.context.screen_manager.lock().unwrap();
        {
            screen.write_str(csi!("s"));
        }
    }

    fn reset_position(&self) {
        let mut screen = self.context.screen_manager.lock().unwrap();
        {
            screen.write_str(csi!("u"));
        }
    }

    fn hide(&self) {
        let mut screen = self.context.screen_manager.lock().unwrap();
        {
            screen.write_str(csi!("?25l"));
        }
    }

    fn show(&self) {
        let mut screen = self.context.screen_manager.lock().unwrap();
        {
            screen.write_str(csi!("?25h"));
        }
    }

    fn blink(&self, blink: bool) {
        let mut screen = self.context.screen_manager.lock().unwrap();
        {
            if blink {
                screen.write_str(csi!("?12h"));
            } else {
                screen.write_str(csi!("?12l"));
            }
        }
    }
}
