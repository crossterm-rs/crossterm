extern crate crossterm;

use crossterm::{style, AlternateScreen, ClearType, Color, Crossterm};
use std::{thread, time, io};

fn print_wait_screen() -> io::Result<()> {
    let crossterm = Crossterm::new();
    let terminal = crossterm.terminal();
    let cursor = crossterm.cursor();

    terminal.clear(ClearType::All)?;
    cursor.goto(0, 0)?;
    cursor.hide()?;
    terminal.write(
        "Welcome to the wait screen.\n\
         Please wait a few seconds until we arrive back at the main screen.\n\
         Progress: ",
    )?;
    // print some progress example.
    for i in 1..5 {
        // print the current counter at the line of `Seconds to Go: {counter}`
        cursor.goto(10, 2)?;
        println!(
            "{}",
            style(format!("{} of the 5 items processed", i))
                .with(Color::Red)
                .on(Color::Blue)
        );

        // 1 second delay
        thread::sleep(time::Duration::from_secs(1));
    }

    Ok(())
}

/// print wait screen on alternate screen, then switch back.
pub fn print_wait_screen_on_alternate_window() -> io::Result<()> {
    if let Ok(_alternate) = AlternateScreen::to_alternate(false) {
        print_wait_screen()?;
    }

    Ok(())
}

fn main() {
    print_wait_screen_on_alternate_window().unwrap();
}
