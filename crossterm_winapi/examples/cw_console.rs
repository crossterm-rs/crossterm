use std::io::Result;

use crossterm_winapi::ConsoleMode;

fn change_console_mode() -> Result<()> {
    let console_mode = ConsoleMode::new()?;

    // get the current console mode:
    let _mode: u32 = console_mode.mode()?;

    // set the console mode (not sure if this is an actual value xp)
    console_mode.set_mode(10)
}

fn main() -> Result<()> {
    change_console_mode()
}
