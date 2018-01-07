use Construct;
use super::base_cursor::ITerminalCursor;

/// Struct that will be instantiated when something went wrong or when an platform does not suport
/// the current concrete cursor inplementations.
pub struct NoCursor;

impl Construct for NoCursor {
    fn new() -> Box<NoCursor> {
        Box::new(NoCursor {})
    }
}

impl ITerminalCursor for NoCursor {
    fn goto(&self, x: u16, y: u16) {}

    fn pos(&self) -> (i16, i16) {
        (0, 0)
    }

    fn move_up(&self, count: u16) {}

    fn move_right(&self, count: u16) {}

    fn move_down(&self, count: u16) {}

    fn move_left(&self, count: u16) {}
}
