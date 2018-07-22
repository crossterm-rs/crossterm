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

use crossterm::Context;

// mod terminal;
// mod color;
// mod cursor;
// mod crossterm_type;
mod input;

use input::keyboard::{async_input, input as stdin};

use crossterm::raw::IntoRawMode;

use std::{thread, time};

fn main() {
    //    let context = Context::new();
    crossterm();
    {
        //        let screen = ::crossterm::screen::AlternateScreen::from(context.clone());
        //        screen.into_raw_mode(context.clone());

        //        async_input::async_reading_on_alternate_screen();
        //        async_input::test();
        //        stdin::t();
        //        stdin::read_line();
        //        stdin::read_char();
        //    stdin::read_char();
    }
}

use crossterm::raw::RawTerminal;
use crossterm::Crossterm;

pub fn crossterm() {
    let crossterm = Crossterm::new();
    let mut term = crossterm.terminal();
    let mut cursor = crossterm.cursor();
    let input = crossterm.input();

    // clear screen
    term.clear(ClearType::All);

    let mut raw_screen = RawTerminal::new(&crossterm.context());
    raw_screen.enable();

    let mut stdin = input.read_async().bytes();

    let mut buf = String::new();

    let mut counter: u16 = 1;

    loop {
        cursor.goto(0, counter);
        term.write("test data");

        let (term_width, term_height) = term.terminal_size();
        let (cursor_x, cursor_y) = cursor.pos();

        if cursor_y >= term_height {
            term.scroll_up(1);
        }

        cursor.goto(0, term_height);
        term.clear(ClearType::CurrentLine);
        term.write(format!("> {}", buf));

        while let Some(b) = stdin.next() {
            if let Ok(b) = b {
                if b == 3 {
                    term.exit();
                } else if b == 13 {
                    buf.clear();
                } else {
                    buf.push(b as char);
                }
            }
        }
        counter += 1;
        thread::sleep(time::Duration::from_millis(100));
    }
}

use crossterm::cursor::cursor::TerminalCursor;
use crossterm::terminal::terminal::Terminal;
use crossterm::terminal::ClearType;
use std::io::Read;

//pub fn swap_write(terminal: &mut Terminal, out: &mut RawTerminal, cursor: &mut TerminalCursor, msg: &str, input_buf: &String) {
//    let (term_width,term_height) = terminal.terminal_size();
//    let (x,y) = cursor.get_post();
//    cursor.goto(0,0);
//
//
//
//
//}
