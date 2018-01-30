//! This trait defines the actions that can be preformed with the termial cursor.
//! This trait can be inplemented so that an concrete inplementation of the ITerminalCursor can forfill
//! the wishes to work on an specific platform.
//!
//! ## For example:
//!
//! This trait is inplemented for winapi (Windows specific) and ansi (Unix specific),
//! so that the cursor related actions can be preformed on both unix and windows systems.

pub trait ITerminalCursor {
    /// Goto some location (x,y) in the terminal.
    fn goto(&self, x: u16, y: u16);
    /// Get the location (x,y) of the current curor in the terminal
    fn pos(&self) -> (i16, i16);
    /// Move cursor n times up
    fn move_up(&self, count: u16);
    /// Move the cursor `n` times to the right.
    fn move_right(&self, count: u16);
    /// Move the cursor `n` times down.
    fn move_down(&self, count: u16);
    /// Move the cursor `n` times left.
    fn move_left(&self, count: u16);
    /// Save cursor position for recall later. Note that this position is stored program based not per instance of the cursor struct.
    fn save_position(&mut self);
    /// Return to saved cursor position
    fn reset_position(&self);
}
