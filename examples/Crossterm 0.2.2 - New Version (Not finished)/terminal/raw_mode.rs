extern crate crossterm;

use crossterm::cursor::cursor;
use crossterm::screen::AlternateScreen;
use crossterm::terminal::{self, ClearType};
use crossterm::Context;

use std::io::{stdout, Write};
use std::rc::Rc;
use std::{thread, time};

use crossterm::raw::IntoRawMode;

fn print_wait_screen(context: Rc<Context>) {
    terminal::terminal(context.clone()).clear(ClearType::All);

    let mut cursor = cursor(context.clone());
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

/// print wait screen on raw alternate screen | demonstration
pub fn print_wait_screen_on_alternate_window() {
    let context = Context::new();

    // create scope. If this scope ends the screen will be switched back to mainscreen.
    // because `AlternateScreen` switches back to main screen when going out of scope.
    {
        // create new alternate screen instance this call is also switching the screen to alternate screen.
        // then convert the output of the program to raw mode.
        // then print the wait screen on the alternate screen in raw mode.
        let mut screen = AlternateScreen::from(context.clone());
        let alternate_screen = screen.into_raw_mode(context.clone());

        // Print the wait screen.
        print_wait_screen(context.clone());

        screen.flush();
    }

    println!("Whe are back at the main screen");
}
