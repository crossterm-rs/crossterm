use Construct;
use super::base_cursor::ITerminalCursor;

/// This struct will be used for ansi terminals and unix systems.
pub struct AnsiCursor;

impl Construct for AnsiCursor {
    fn new() -> Box<AnsiCursor> {
        Box::from(AnsiCursor {})
    }
}

impl ITerminalCursor for AnsiCursor {
    fn goto(&self, x: u16, y: u16) {
        format!(csi!("{};{}H"), x, y);
    }

    fn pos(&self) -> (u16, u16) {
        (0, 0)
    }

    fn move_up(&self, count: u16) {
        format!(csi!("{}A"), count);
    }

    fn move_right(&self, count: u16) {
        format!(csi!("{}C"), count);
    }

    fn move_down(&self, count: u16) {
        format!(csi!("{}B"), count);
    }

    fn move_left(&self, count: u16) {
        format!(csi!("{}D"), count);
    }
}
