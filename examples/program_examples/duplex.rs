//! This bin folder can be used to try the examples out located in the examples directory.
//!
//! All you need to do is:
//!
//! - Download the crossterm source code.
//! - Add this in the Cargo.toml file:
//!   ``` [[bin]]
//!        name = "example_bin"
//!        path = "./examples/bin.rs"
//!   ```
//!   
//! - Run program with: `cargo run`
extern crate crossterm;

use crossterm::style::Color;
use crossterm::Crossterm;
use crossterm::terminal::ClearType;
use std::thread::sleep;
use std::sync::{Arc,Mutex};
use std::io::Read;
use std::time::Duration;
// mod terminal;
// mod color;
// mod cursor;
// mod crossterm_type;
// mod input;

//use input::keyboard::{async_input, input as stdin};

use std::thread;


            }fn main() {
            let mut terminal = Arc::new(Mutex::new(Crossterm::new()));
            let input = terminal.lock().unwrap().input().read_async();
            terminal.lock().unwrap().enable_raw_mode();
            let mut input_buf = Arc::new(Mutex::new(String::new()));
            let mut key_buf = [0 as u8; 32];

            thread::spawn(move || {
                loop {
                    swap_write(&mut terminal.lock().unwrap(), "random program output",&input_buf.lock().unwrap());
                    sleep(Duration::from_millis(100));
                }
            });

            loop {
                let mut term = terminal.lock().unwrap();
                let (term_width, term_height) = term.terminal().terminal_size();
                if let Ok(count) = input.read(&mut key_buf) {
                    for idx in 0..count {
                        let b = key_buf.get(idx).unwrap();
                        if *b == 3 {
                            std::process::exit(0); // Ctrl+C = exit immediate
                        } else if *b == 13 {
                            // The return key was pressed.
                            let mut input_buf_tmp = &mut input_buf.lock().unwrap();
                            input_buf.lock().unwrap().clear();
                            swap_write(&mut term, "", &input_buf_tmp);
                        } else {
                            let mut input_buf_tmp = &mut input_buf.lock().unwrap();
                            input_buf_tmp.push(*b as char);
                            swap_write(&mut term, "", &input_buf_tmp);
                        }
                    }
    }
}

pub fn swap_write(terminal: &mut Crossterm, msg: &str, input_buf: &String) {
    let (term_width, term_height) = terminal.terminal().terminal_size();
    terminal.cursor().goto(0, term_height);
    terminal.terminal().clear(ClearType::CurrentLine);
    terminal
        .terminal()
        .write(format!("{}\n\r>{}", msg, input_buf));
}
