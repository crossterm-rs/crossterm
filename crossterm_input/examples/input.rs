extern crate crossterm_input;

use self::crossterm_input::{input, KeyEvent, Screen, TerminalInput};

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
    let terminal_input = TerminalInput::new();
    terminal_input.wait_until(KeyEvent::OnKeyPress(b'x'));
}

fn main() {}
