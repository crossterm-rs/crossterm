extern crate crossterm;

use self::crossterm::terminal_style::{paint,Color};
use self::crossterm::terminal_cursor::cursor;
use std::io::Write;


fn main()
{      
    cursor::get().move_down(1);
    print!("2");
    std::io::stdout().flush().expect("asdf"); 

    cursor::get().move_down(1);
    print!("3");    
    std::io::stdout().flush().expect("asdf"); 

    cursor::get().move_down(1);
    print!("4");    
    std::io::stdout().flush().expect("asdf"); 

    cursor::get().move_down(1);
    print!("5");    
    std::io::stdout().flush().expect("asdf"); 

}