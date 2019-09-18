use std::io::{stdout, Write};
use std::{thread, time};

use crossterm::{style, AlternateScreen, ClearType, Color, Crossterm, Result};

fn print_wait_screen() -> Result<()> {
    let crossterm = Crossterm::new();
    let terminal = crossterm.terminal();
    let cursor = crossterm.cursor();

    terminal.clear(ClearType::All)?;

    cursor.hide()?;
    cursor.goto(0, 0)?;
    println!("Welcome to the wait screen.");
    cursor.goto(0, 1)?;
    println!("Please wait a few seconds until we arrive back at the main screen.");
    cursor.goto(0, 2)?;
    println!("Progress:");
    cursor.goto(0, 3)?;

    // print some progress example.
    for i in 1..5 {
        // print the current counter at the line of `Seconds to Go: {counter}`
        cursor.goto(10, 2)?;
        print!(
            "{}",
            style(format!("{} of the 5 items processed", i))
                .with(Color::Red)
                .on(Color::Blue)
        );

        stdout().flush()?;

        // 1 second delay
        thread::sleep(time::Duration::from_secs(1));
    }

    Ok(())
}

fn print_wait_screen_on_alternate_window() -> Result<()> {
    // by passing in 'true' the alternate screen will be in raw modes.
    let _alt = AlternateScreen::to_alternate(true)?;
    print_wait_screen()
}

// cargo run --example raw_mode
fn main() -> Result<()> {
    print_wait_screen_on_alternate_window()
}
