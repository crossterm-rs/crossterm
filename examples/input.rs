extern crate crossterm;

use self::crossterm::{KeyEvent, TerminalInput, input, Screen};

pub fn read_char() {
    let input = input();

    match input.read_char() {
        Ok(s) => println!("char typed: {}", s),
        Err(e) => println!("char error : {}", e),
    }
}

pub fn read_line() {
    let input = input();

    match input.read_line() {
        Ok(s) => println!("string typed: {}", s),
        Err(e) => println!("error: {}", e),
    }
}

pub fn pause_terminal() {
    println!("Press 'x' to quit...");
    let screen = Screen::new(true);
    let terminal_input = TerminalInput::from_output(&screen.stdout);
    terminal_input.wait_until(KeyEvent::OnKeyPress(b'x'));
}

fn main(){
    pause_terminal();
}
