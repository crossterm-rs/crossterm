extern crate crossterm;

use crossterm::style::Color;
use crossterm::cursor::cursor;
use crossterm::screen::AlternateScreen;
use crossterm::terminal::{self, ClearType};
use crossterm::Context;

use std::io::{stdout, Write};
use std::rc::Rc;
use std::{thread, time};

fn print_wait_screen(context: Rc<Context>) {
    let mut terminal = terminal::terminal(context.clone());
    terminal.clear(ClearType::All);

    let mut cursor = cursor(context.clone());
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
        cursor
            .goto(10, 2)
            .print(terminal.paint(format!("{} of the 5 items processed", i)).with(Color::Red).on(Color::Blue));

        // 1 second delay
        thread::sleep(time::Duration::from_secs(1));
    }

    stdout().flush();
}

/// print wait screen on alternate screen, then swich back.
pub fn print_wait_screen_on_alternate_window(context: Rc<Context>) {
    // create scope. If this scope ends the screen will be switched back to mainscreen.
    // because `AlternateScreen` switches back to main screen when switching back.
    {
        // create new alternate screen instance and switch to the alternate screen.
        let mut screen = AlternateScreen::from(context.clone());

        write!(screen,  "test");
        println!();
        // Print the wait screen.
        print_wait_screen(context.clone());
    }
}

/// some stress test switch from and to alternate screen.
pub fn switch_between_main_and_alternate_screen() {
    let context = Context::new();
    let mut cursor = cursor(context.clone());

    {
        // create new alternate screen instance and switch to the alternate screen.
        let mut screen = AlternateScreen::from(context.clone());
        cursor.goto(0, 0);
        write!(screen, "we are at the alternate screen!");
        screen.flush();
        thread::sleep(time::Duration::from_secs(3));

        screen.to_main();
        write!(screen, "we are at the main screen!");
        screen.flush();
        thread::sleep(time::Duration::from_secs(3));

        screen.to_alternate();
        write!(screen, "we are at the alternate screen!");
        screen.flush();
        thread::sleep(time::Duration::from_secs(3));
    }

    println!("Whe are back at the main screen");
}
