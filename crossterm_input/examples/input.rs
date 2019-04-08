extern crate crossterm_input;

use self::crossterm_input::input;

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

fn main() {
    // un-comment below and run with
    // `cargo run --example input`:
    read_char();
    read_line();
}
