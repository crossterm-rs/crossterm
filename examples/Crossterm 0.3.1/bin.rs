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

use crossterm::Terminal;

// mod terminal;
// mod color;
// mod cursor;
// mod crossterm_type;
// mod input;

//use input::keyboard::{async_input, input as stdin};

use std::{thread, time};

fn main() {
<<<<<<< HEAD
    let term = Terminal::new();
    let mut cursor = term.cursor();
    cursor.goto(10,10);
    cursor.print("test");

    let stdin = term.input();
    let line = stdin.read_line();
    println!("{:?}", line)
}
=======
    {
        let mut terminal = Terminal::new();
        terminal.enable_alternate_screen();
        thread::sleep(time::Duration::from_millis(5000));
    }
}
>>>>>>> 403d0668a72e9ca04a05bbe137a30d6a2d9ba90c
