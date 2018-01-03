/// This trait defines the actions that can be preformed on the termial cursor.
/// This trait can be inplemented so that an concrete inplementation of the ITerminalCursor can forfill
/// the wishes to work on an specific platform. 
/// 
/// ## For example:
/// 
/// This trait is inplemented for winapi (Windows specific) and ansi (Unix specific),
/// so that the cursor related actions can be preformed on both unix and windows systems.   
pub trait ITerminalCursor
{    
    /// Goto some location (x,y) in the terminal.
    fn goto(&self, x: i16, y: i16); 
    fn move_up(&self, count: u16);
    fn move_right(&self, count: u16);
    fn move_down(&self, count: u16);
    fn move_left(&self, count: u16);
}


