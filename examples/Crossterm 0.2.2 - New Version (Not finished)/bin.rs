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

mod terminal;
mod cursor;
mod color;
mod program_examples;

fn main() {

    use crossterm::Context;

    {
        let mut context = Context::new();

        terminal::alternate_screen::print_wait_screen_on_alternate_window(context.clone());

        println!("count: {}", std::rc::Rc::strong_count(&context));
    }
}