extern crate crossterm;

use crossterm::{Crossterm, Screen, terminal, ClearType, Color, style};

use std::io::{stdout, Write};
use std::{thread, time};

fn print_wait_screen(screen: &mut Screen) {
    let crossterm = Crossterm::from_screen(screen);
    let terminal = crossterm.terminal();
    let cursor = crossterm.cursor();

    terminal.clear(ClearType::All);

    cursor.hide();
    cursor.goto(0, 0);
    screen.write(b"Welcome to the wait screen.");
    cursor.goto(0, 1);
    screen.write(b"Please wait a few seconds until we arrive back at the main screen.");
    cursor.goto(0, 2);
    screen.write(b"Progress:");
    cursor.goto(0, 3);

    // print some progress example.
    for i in 1..5 {
        // print the current counter at the line of `Seconds to Go: {counter}`
        cursor.goto(10, 2);
        style(format!("{} of the 5 items processed", i))
            .with(Color::Red)
            .on(Color::Blue)
            .paint(&screen.stdout);
        screen.stdout.flush();

        // 1 second delay
        thread::sleep(time::Duration::from_secs(1));
    }
}

pub fn print_wait_screen_on_alternate_window() {
    let screen = Screen::default();

    // by passing in 'true' the alternate screen will be in raw modes.
    if let Ok(ref mut alternate) = screen.enable_alternate_modes(true) {
        print_wait_screen(&mut alternate.screen);
    } // <- drop alternate screen; this will cause the alternate screen to drop.

    drop(screen); // <- drop screen; this will cause raw mode to be turned off.

    println!("Whe are back at the main screen");
}

fn main() {
    print_wait_screen_on_alternate_window();
}