#![allow(dead_code)]

use std::io::Result;

use crossterm_winapi::ScreenBuffer;

fn print_screen_buffer_information() -> Result<()> {
    let screen_buffer = ScreenBuffer::current()?;

    // get console screen buffer information
    let csbi = screen_buffer.info()?;

    println!("cursor post: {:?}", csbi.cursor_pos());
    println!("attributes: {:?}", csbi.attributes());
    println!("terminal window dimentions {:?}", csbi.terminal_window());
    println!("terminal size {:?}", csbi.terminal_size());

    Ok(())
}

fn multiple_screen_buffers() -> Result<()> {
    // create new screen buffer
    let screen_buffer = ScreenBuffer::create();

    // which to this screen buffer
    screen_buffer.show()
}

fn main() -> Result<()> {
    print_screen_buffer_information()
}
