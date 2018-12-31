//! This bin folder can be used to try the examples out located in the examples directory.
//!
//! All you need to do is:
//!
//! - Download the crossterm source code.
//! - Run program with: `cargo run --example examples`

extern crate crossterm;

// modules that could be test
mod color;
mod cursor;
mod input;
//mod some_types;
mod terminal;

fn main() {
    use input::keyboard::input;

    //    color::print_all_foreground_colors();
    //    color::print_all_background_colors();

    use terminal::alternate_screen;
    //    color::print_all_background_colors();
    //    color::print_all_foreground_colors();

    alternate_screen::print_wait_screen_on_alternate_window();
}
