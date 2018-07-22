extern crate crossterm;

use self::crossterm::input::input;
use self::crossterm::Context;

pub fn read_char() {
    let context = Context::new();
    let input = input(&context);

    match input.read_char() {
        Ok(c) => println!("character pressed: {}", c),
        Err(e) => println!("error: {}", e),
    }
}

pub fn read_line() {
    let context = Context::new();
    let input = input(&context);

    match input.read_line() {
        Ok(s) => println!("string typed: {}", s),
        Err(e) => println!("error: {}", e),
    }
}
