use crossterm::{
    execute,
    terminal::{size, SetSize},
};
use std::io::{stdin, stdout, IsTerminal};

pub fn main() {
    println!("size: {:?}", size().unwrap());
    execute!(stdout(), SetSize(10, 10)).unwrap();
    println!("resized: {:?}", size().unwrap());

    if stdin().is_terminal() {
        println!("Is TTY");
    } else {
        println!("Is not TTY");
    }
}
