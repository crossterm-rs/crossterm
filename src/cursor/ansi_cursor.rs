//! This is an ANSI specific implementation for cursor related action.
//! This module is used for windows 10 terminals and unix terminals by default.

use std::sync::Mutex;
use { Context, ScreenManager, Terminal };
use shared::functions;
use super::ITerminalCursor;
use std::io::{ self, Write };


/// This struct is an ansi implementation for cursor related actions.
pub struct AnsiCursor
{ }

impl<'output> AnsiCursor {
    pub fn new() -> Box<AnsiCursor> {
        Box::from(AnsiCursor {})
    }
}

impl<'term> ITerminalCursor for AnsiCursor {

    fn goto(&self, x: u16, y: u16, terminal: &Terminal)
    {
        // ANSI codes are one-based. I want 0 based so we just need to increment and x,y.

        let mut screen = terminal.screen_manager.lock().unwrap();
        {
            screen.write_ansi(format!(csi!("{};{}H"), y + 1, x +1));
        }
    }

    fn pos(&self, terminal: &Terminal) -> (u16, u16) {
        functions::get_cursor_position(&terminal)
    }

    fn move_up(&self, count: u16, terminal: &Terminal) {
        let mut screen = terminal.screen_manager.lock().unwrap();
        {
            screen.write_ansi_str(csi!("{}A"));
        }
    }

    fn move_right(&self, count: u16, terminal: &Terminal) {
        let mut screen = terminal.screen_manager.lock().unwrap();
        {
            screen.write_ansi_str(csi!("{}C"));
        }
    }

    fn move_down(&self, count: u16, terminal: &Terminal) {
        let mut screen = terminal.screen_manager.lock().unwrap();
        {
            screen.write_ansi_str(csi!("{}B"));
        }
    }

    fn move_left(&self, count: u16, terminal: &Terminal) {
        let mut screen = terminal.screen_manager.lock().unwrap();
        {
            screen.write_ansi_str(csi!("{}D"));
        }
    }

    fn save_position(&mut self, terminal: &Terminal)
    {
        let mut screen = terminal.screen_manager.lock().unwrap();
        {
            screen.write_ansi_str(csi!("s"));
        }
    }

    fn reset_position(&self, terminal: &Terminal)
    {
        let mut screen = terminal.screen_manager.lock().unwrap();
        {
            screen.write_ansi_str(csi!("u"));
        }
    }
}
