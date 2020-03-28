use std::io::stdin;
use crossterm::tty::IsTty;

pub fn main() {
    if stdin().is_tty() {
        println!("Is TTY");
    } else {
        println!("Is not TTY");
    }
}