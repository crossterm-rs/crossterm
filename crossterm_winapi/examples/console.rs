extern crate crossterm_winapi;

use crossterm_winapi::ConsoleMode;

pub fn change_console_mode() {
    let console_mode = ConsoleMode::new().unwrap();

    // get the current console mode:
    let mode: u32 = console_mode.mode().unwrap();

    // set the console mode (not sure if this is an actual value xp)
    console_mode.set_mode(10);
}

fn main() {}
