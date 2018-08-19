
//! This bin folder can be used to try the examples out located in the examples directory.
//!
//! All you need to do is:
//!
//! - Download the crossterm source code.
//! - Run program with: `cargo run --example examples`

extern crate crossterm;
#[macro_use]
extern crate lazy_static;

// modules that could be test
mod terminal;
mod color;
mod cursor;
mod some_types;
mod input;

use crossterm::{Screen, Crossterm};
use crossterm::terminal::{Terminal, ClearType};
use crossterm::cursor::TerminalCursor;

use std::{time, thread};
use std::sync::mpsc;
use std::sync::{Arc,Mutex};
use crossterm::cursor::cursor;
use std::io::Read;

fn main() {
    use crossterm::color;

    let input = CROSSTERM.input();
    let mut stdin = input.read_async().bytes();
    CROSSTERM.cursor().hide();

    let mut input_buf = Arc::new(Mutex::new(String::new()));

    loop
    {
        let a = stdin.next();

        swap_write("dddd", &input_buf.lock().unwrap());

        match a {
            Some(Ok(b'\r')) =>
            {
                input_buf.lock().unwrap().clear();

                // need to start receiving again because if pressed enter then async reading will stop
                stdin = input.read_async().bytes();
            }
            Some(Ok(val)) =>
            {
                input_buf.lock().unwrap().push(val as char);
            }
            _ => {}
        }

        thread::sleep(time::Duration::from_millis(100));
    }
}

pub fn swap_write(msg: &str, input_buf: &String) {
    let term =  CROSSTERM.terminal();
    let (_, term_height) = term.terminal_size();
    CROSSTERM.cursor().goto(0, term_height);
    term.clear(ClearType::CurrentLine);
    term.write(format!("{}\r\n", msg));
    term.write(format!(">{}", input_buf));
}

lazy_static! {
    static ref CROSSTERM: Crossterm = {
        let screen = Screen::new(true);
        Crossterm::new(&screen)
    };
}
