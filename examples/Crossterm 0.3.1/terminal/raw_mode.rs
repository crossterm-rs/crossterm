extern crate crossterm;

use crossterm::Crossterm;

use crossterm::terminal::{self, ClearType};

use std::io::{stdout, Write};
use std::{thread, time};

// raw screen is not working correctly currently
fn print_wait_screen(crossterm: &mut Crossterm) {
    let terminal = crossterm.terminal();
    let mut cursor = crossterm.cursor();

    terminal.clear(ClearType::All);

    cursor.goto(0, 0).print("Welcome to the wait screen.");
    cursor
        .goto(0, 1)
        .print("Please wait a few seconds until we arrive back at the main screen.");
    cursor.goto(0, 2).print("Progress: ");

    // print some progress example.
    for i in 1..5 {
        // print the current counter at the line of `Seconds to Go: {counter}`
        cursor
            .goto(10, 2)
            .print(format!("{} of the 5 items processed", i));

        // 1 second delay
        thread::sleep(time::Duration::from_secs(1));
    }
}

pub fn print_wait_screen_on_alternate_window() {
    let mut term = Crossterm::new();

    // create scope. If this scope ends the screen will be switched back to mainscreen.
    // because `AlternateScreen` switches back to main screen when going out of scope.
    {
        // create new alternate screen instance this call is also switching the screen to alternate screen.
        // then convert the output of the program to raw mode.
        // then print the wait screen on the alternate screen in raw mode.
        term.to_alternate_screen();
        term.enable_raw_mode();

        // Print the wait screen.
        print_wait_screen(&mut term);

        term.flush();
    }

    println!("Whe are back at the main screen");
}
