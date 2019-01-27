extern crate crossterm_winapi;

use crossterm_winapi::{Handle, ScreenBuffer};

fn main() {}

fn print_screen_buffer_information() {
    let screen_buffer = ScreenBuffer::current().unwrap();

    // get console screen buffer information
    let csbi = screen_buffer.info().unwrap();

    println!("cursor post: {:?}", csbi.cursor_pos());
    println!("attributes: {:?}", csbi.attributes());
    println!("terminal window dimentions {:?}", csbi.terminal_window());
    println!("terminal size {:?}", csbi.terminal_size());
}

fn multiple_screen_buffers() {
    // create new screen buffer
    let screen_buffer = ScreenBuffer::create();

    // which to this screen buffer
    screen_buffer.show();
}
