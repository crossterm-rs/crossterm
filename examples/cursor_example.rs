extern crate crossterm;

use self::crossterm::cursor;

pub fn goto(x: i16, y: i16)
{
    cursor::get().goto(x,y);
}