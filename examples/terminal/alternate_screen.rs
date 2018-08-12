extern crate crossterm;

use crossterm::style::{Color, style};
use crossterm::terminal::{self, ClearType};
use crossterm::{Crossterm, Screen};

use std::io::{stdout, Write};
use std::{thread, time};

fn print_wait_screen(screen: &Screen) {
    let crossterm = Crossterm::new();
    let terminal = crossterm.terminal(&screen);
    let cursor = crossterm.cursor(&screen);

    terminal.clear(ClearType::All);
    cursor.goto(0, 0);
    cursor.hide();

    terminal.write(
        "Welcome to the wait screen.\n\
         Please wait a few seconds until we arrive back at the main screen.\n\
         Progress: ",
    );

    // print some progress example.
    for i in 1..5 {
        // print the current counter at the line of `Seconds to Go: {counter}`
        cursor.goto(10, 2);
        style(format!("{} of the 5 items processed", i)).with(Color::Red).on(Color::Blue).paint(&screen);

        // 1 second delay
        thread::sleep(time::Duration::from_secs(1));
    }
}

/// print wait screen on alternate screen, then swich back.
pub fn print_wait_screen_on_alternate_window() {

    let screen = Screen::default();

    if let Ok(alternate) = screen.enable_alternate_modes(false)
    {
        print_wait_screen(&alternate.screen);
    }
}
