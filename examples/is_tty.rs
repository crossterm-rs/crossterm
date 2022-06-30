use crossterm::{tty::IsTty, terminal::{size, SetSize}, execute};
use std::io::{stdin, stdout};

pub fn main() {
    println!("{:?}", size().unwrap());
    execute!(stdout(), SetSize(10, 10)).unwrap();
    println!("{:?}", size().unwrap());

    if stdin().is_tty() {
        println!("Is TTY");
    } else {
        println!("Is not TTY");
    }
}
