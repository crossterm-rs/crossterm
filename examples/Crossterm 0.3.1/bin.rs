
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

fn main()
{
    let context = Context::new();

    {
//        let screen = ::crossterm::screen::AlternateScreen::from(context.clone());
//        screen.into_raw_mode(context.clone());


        async_input::async_reading_on_alternate_screen();
//        async_input::test();
//        stdin::t();
//        stdin::read_line();
//        stdin::read_char();
//    stdin::read_char();
    }
}


