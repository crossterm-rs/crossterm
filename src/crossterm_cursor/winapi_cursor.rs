use Construct;
use kernel::windows_kernel::cursor;
use super::base_cursor::ITerminalCursor;

/// This struct is an windows implementation for cursor related actions.
pub struct WinApiCursor;

impl Construct for WinApiCursor {
    fn new() -> Box<WinApiCursor> {
        Box::from(WinApiCursor { })
    }
}

impl ITerminalCursor for WinApiCursor {
    fn goto(&self, x: u16, y: u16) {
        cursor::set(x as i16, y as i16);
    }

    fn pos(&self) -> (i16, i16) {
        cursor::pos()
    }

    fn move_up(&self, count: u16) {
        let (xpos,ypos) = cursor::pos();
        cursor::set(xpos, ypos - count as i16);
    }

    fn move_right(&self, count: u16) {
        let (xpos,ypos) = cursor::pos();

        cursor::set(xpos + count as i16, ypos);
    }

    fn move_down(&self, count: u16) {
        let (xpos,ypos) = cursor::pos();

        cursor::set(xpos, ypos + count as i16);
    }

    fn move_left(&self, count: u16) {
        let (xpos,ypos) = cursor::pos();

        cursor::set(xpos - count as i16, ypos);
    }


    fn safe_position(&mut self)
    {
        cursor::save_cursor_pos();
    }

    fn reset_position(&self)
    {
        cursor::reset_to_saved_position();
    }
}
