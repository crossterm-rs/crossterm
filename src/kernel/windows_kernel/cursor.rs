extern crate winapi;
extern crate kernel32;

use self::winapi::{COORD};
use super::{handle,kernel};

/// These are the movement directions of an cursor 
#[derive(Debug)]
pub enum CursorDirection
{
    Top,
    Right,
    Down,
    Left,
}

/// Set the cursor position to an coordinate (x,y).
pub fn set(x: u16, y:u16)
{
    set_cursor_pos(x as i16,y as i16);
}

/// Get the current cursor x position.
pub fn xpos() -> u16
{
    if let Some(csbi) = kernel::get_console_screen_buffer_info() 
    {        
        csbi.dwCursorPosition.Y as u16
    }else{
        println!("xpos verkeerd");
        0 
    }
}

/// Get the current cursor y position.
pub fn ypos() -> u16
{
    if let Some(csbi) = kernel::get_console_screen_buffer_info() 
    {        
        csbi.dwCursorPosition.Y as u16
    }else{
        println!("ypos verkeerd");
        0 
    }
}

pub fn move_down(count: u16)
{
    if let Some(buffer) = kernel::get_console_screen_buffer_info() 
    {        
        unsafe
        {
             let handle = kernel32::GetStdHandle(winapi::STD_OUTPUT_HANDLE);
             kernel32::SetConsoleCursorPosition(handle, COORD {
                X: buffer.dwCursorPosition.X,
                Y: buffer.dwCursorPosition.Y + count as i16,
            });
        }
    }  
}


// pub fn move_direction(count: i16, cursor_direction: CursorDirection)
// {
//      
        
//       println!("{}, {}, {:?}",x,y, cursor_direction);

//         match cursor_direction 
//         {
//             CursorDirection::Top => set_cursor_pos(x,y - count) ,
//             CursorDirection::Right => set_cursor_pos(x + count, y) ,
//             CursorDirection::Down => set_cursor_pos(x, y + count),
//             CursorDirection::Left => set_cursor_pos(x - count,y),
//         };
//     }else{
//         println!("{}", "Not found");
//     }
      
// }

/// Set the cursor position to an coordinate (x,y).
fn set_cursor_pos(x: i16, y: i16)
{
    if let Some(handle) = handle::get_output_handle()
    {
        unsafe 
        {
            let position = COORD{X: x, Y:y};
            kernel32::SetConsoleCursorPosition(handle, position);        
        }
    }
}