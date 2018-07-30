extern crate crossterm;

use self::crossterm::Crossterm;

pub fn read_char() {
    let context = Context::new();
    let input = input(&context);

    match input.read_char() {
        Ok(c) => println!("character pressed: {}", c),
        Err(e) => println!("error: {}", e),
    }
}

pub fn read_line() {
    let crossterm = Crossterm::new();
    let input = crossterm.input();

    match input.read_line() {
        Ok(s) => println!("string typed: {}", s),
        Err(e) => println!("error: {}", e),
    }
}
