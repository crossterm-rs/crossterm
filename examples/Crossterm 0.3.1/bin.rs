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


use crossterm::cursor::cursor::TerminalCursor;
use crossterm::terminal::terminal::Terminal;
use crossterm::terminal::ClearType;
use std::io::Read;

pub fn crossterm() {
    let crossterm = Crossterm::new();
    let mut term = crossterm.terminal();
    let mut cursor = crossterm.cursor();
    let input = crossterm.input();

    // clear screen
    term.clear(ClearType::All);

    let mut raw_screen = RawTerminal::new(&crossterm.context());
    raw_screen.enable();

    let mut stdin = input.read_until_async().bytes();

    let mut buf = String::new();

    let (term_x, term_y) = term.terminal_size();
    let mut command_bar_y = term_y;
    let (curs_x, curs_y) = cursor.pos();

    let mut counter: u16 = 0 + curs_y;
    loop {
        cursor.goto(0, counter);
        let (curs_x, curs_y) = cursor.pos();
        term.write(format!("cursor pos {} term pos: {} command pos: {}", curs_y, term_y, command_bar_y));
        cursor.goto(0, counter + 1);

        if (curs_y >= term_y - 1 )
        {
            cursor.goto(0, counter + 1);
            term.clear(ClearType::CurrentLine);
            cursor.goto(0, counter + 2);
            term.write(format!("> {}", buf));
            term.scroll_up(1);
        }

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