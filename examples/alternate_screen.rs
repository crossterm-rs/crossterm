use std::{thread, time};

use crossterm::{style, AlternateScreen, ClearType, Color, Crossterm, Result};

fn print_wait_screen() -> Result<()> {
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
fn print_wait_screen_on_alternate_window() -> Result<()> {
    let _alt = AlternateScreen::to_alternate(false)?;
    print_wait_screen()
}

// cargo run --example alternate_screen
fn main() -> Result<()> {
    print_wait_screen_on_alternate_window()
}
