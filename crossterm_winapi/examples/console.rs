extern crate crossterm_winapi;

use crossterm_winapi::ConsoleMode;

fn change_console_mode() {
    let console_mode = ConsoleMode::new().unwrap();

    // get the current console mode:
    let _mode: u32 = console_mode.mode().unwrap();

    // set the console mode (not sure if this is an actual value xp)
    console_mode
        .set_mode(10)
        .expect("Unable to set console mode");
}

fn main() {}
