use Construct;
use kernel::windows_kernel::cursor;
use super::base_cursor::ITerminalCursor;

/// This struct will be used for cursor actions in windows terminals performed by winapi. 
pub struct WinApiCursor
{
    has_moved: bool,
}

impl Construct for WinApiCursor
{
    fn new() -> Box<WinApiCursor>
    {        
       Box::from(WinApiCursor {has_moved: false})
    }
}

impl ITerminalCursor for WinApiCursor
{
    fn goto(&self, x: i16 , y: i16)
    {     
        // cursor::set(x,y);         
    }   

    fn move_up(&self, count: u16)
    {
        let xpos = cursor::xpos();
        let ypos = cursor::ypos();
        
        cursor::set(xpos, ypos -1);
    }

    fn move_right(&self, count: u16)
    {
        let xpos = cursor::xpos();
        let ypos = cursor::ypos();

        cursor::set(xpos + 1, ypos);
    }

    fn move_down(&self, count: u16)
    {
        let xpos = cursor::xpos();
        let ypos = cursor::ypos();

        cursor::set(xpos, ypos + count);
    }

    fn move_left(&self, count: u16)
    {
        let xpos = cursor::xpos();
        let ypos = cursor::ypos();

        cursor::set(xpos -1, ypos);
    }
}
