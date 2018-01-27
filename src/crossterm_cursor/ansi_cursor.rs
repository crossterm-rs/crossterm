use std::io;
use std::io::Write;

use Construct;
use super::base_cursor::ITerminalCursor;

/// This struct is an ansi implementation for cursor related actions.
pub struct AnsiCursor;

impl Construct for AnsiCursor {
    fn new() -> Box<AnsiCursor> {
        Box::from(AnsiCursor {})
    }
}

impl ITerminalCursor for AnsiCursor {
    fn goto(&self, x: u16, y: u16) {
        let mut some_writer = io::stdout();
        // ANSI codes are one-based. I want 0 based so we just need to increment and x,y.
        write!(&mut some_writer, csi!("{};{}H"), y + 1, x +1);
    }

    fn pos(&self) -> (i16, i16) {
        (0, 0)
    }

    fn move_up(&self, count: u16) {

        let mut some_writer = io::stdout();
        write!(&mut some_writer, csi!("{}A"), count);
    }

    fn move_right(&self, count: u16) {
        let mut some_writer = io::stdout();
        write!(&mut some_writer, csi!("{}C"), count);
    }

    fn move_down(&self, count: u16) {
        let mut some_writer = io::stdout();
        write!(&mut some_writer, csi!("{}B"), count); 
    }

    fn move_left(&self, count: u16) {
        let mut some_writer = io::stdout();
        write!(&mut some_writer, csi!("{}D"), count);
    }

    fn safe_position(&mut self)
    {
        let mut some_writer = io::stdout();
        write!(&mut some_writer, csi!("s"));
    }

    fn reset_position(&self)
    {
        let mut some_writer = io::stdout();
        write!(&mut some_writer, csi!("u"));
    }
}
