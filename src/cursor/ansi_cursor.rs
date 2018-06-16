//! This is an ANSI specific implementation for cursor related action.
//! This module is used for windows 10 terminals and unix terminals by default.
//! Note that the cursor position is 0 based. This means that we start counting at 0 when setting the cursor position ect.

use Context;
use shared::functions;
use super::ITerminalCursor;

/// This struct is an ansi implementation for cursor related actions.
pub struct AnsiCursor;

impl AnsiCursor {
    pub fn new() -> Box<AnsiCursor> {
        Box::from(AnsiCursor {})
    }
}

impl ITerminalCursor for AnsiCursor {

    fn goto(&self, x: u16, y: u16, context: &Context)
    {
        let mut screen = context.screen_manager.lock().unwrap();
        {
            screen.write_ansi(format!(csi!("{};{}H"), y + 1, x +1));
        }
    }

    fn pos(&self, context: &Context) -> (u16, u16) {
        functions::get_cursor_position(&context)
    }

    fn move_up(&self, count: u16, context: &Context) {
        let mut screen = context.screen_manager.lock().unwrap();
        {
            screen.write_ansi(format!(csi!("{}A"), count));
        }
    }

    fn move_right(&self, count: u16, context: &Context) {
        let mut screen = context.screen_manager.lock().unwrap();
        {
            screen.write_ansi(format!(csi!("{}C"), count));
        }
    }

    fn move_down(&self, count: u16, context: &Context) {
        let mut screen = context.screen_manager.lock().unwrap();
        {
            screen.write_ansi(format!(csi!("{}B"), count));
        }
    }

    fn move_left(&self, count: u16, context: &Context) {
        let mut screen = context.screen_manager.lock().unwrap();
        {
            screen.write_ansi(format!(csi!("{}D"), count));
        }
    }

    fn save_position(&mut self, context: &Context)
    {
        let mut screen = context.screen_manager.lock().unwrap();
        {
            screen.write_ansi_str(csi!("s"));
        }
    }

    fn reset_position(&self, context: &Context)
    {
        let mut screen = context.screen_manager.lock().unwrap();
        {
            screen.write_ansi_str(csi!("u"));
        }
    }
}
