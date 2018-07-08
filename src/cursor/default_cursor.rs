////! This is an WINAPI specific implementation for cursor related action.
////! This module is used for windows terminals that do not support ANSI escape codes.
////! Note that the cursor position is 0 based. This means that we start counting at 0 when setting the cursor position ect.
//
///// This struct is an windows implementation for cursor related actions.
//pub struct DefaultCursor;
//
//impl ITerminalCursor for DefaultCursor {
//    fn goto(&self, x: u16, y: u16) { }
//
//    fn pos(&self) -> (u16, u16) { return (0,0) }
//
//    fn move_up(&self, count: u16) { }
//
//    fn move_right(&self, count: u16) { }
//
//    fn move_down(&self, count: u16) { }
//
//    fn move_left(&self, count: u16) { }
//
//    fn save_position(&self) { }
//
//    fn reset_position(&self) { }
//
//    fn hide(&self) { }
//
//    fn show(&self) { }
//
//    fn blink(&self, blink: bool) {}
//}
