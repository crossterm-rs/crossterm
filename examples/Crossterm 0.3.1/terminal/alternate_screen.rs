extern crate crossterm;

use crossterm::style::Color;
use crossterm::terminal::{self, ClearType};
use crossterm::Crossterm;

use std::io::{stdout, Write};
use std::{thread, time};

fn print_wait_screen(crossterm: &mut Crossterm) {
    let mut terminal = crossterm.terminal();
    let mut cursor = crossterm.cursor();

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
        cursor
            .goto(10, 2)
            .print(crossterm.paint(format!("{} of the 5 items processed", i)).with(Color::Red).on(Color::Blue));

        // 1 second delay
        thread::sleep(time::Duration::from_secs(1));
    }

    stdout().flush();
}

/// print wait screen on alternate screen, then swich back.
pub fn print_wait_screen_on_alternate_window() {

    let mut term = Crossterm::new();
    term.to_alternate_screen();

    term.write(b"test");
    print_wait_screen(&mut term);
}

/// some stress test switch from and to alternate screen.
pub fn switch_between_main_and_alternate_screen() {

    {
        let mut term = Crossterm::new();
        let cursor = term.cursor();

        // create new alternate screen instance and switch to the alternate screen.
        term.to_alternate_screen();
        cursor.goto(0, 0);
        write!(term, "we are at the alternate screen!");
        thread::sleep(time::Duration::from_secs(3));

        term.to_main_screen();
        write!(term, "we are at the alternate screen!");
        thread::sleep(time::Duration::from_secs(3));

        term.to_alternate_screen();
        write!(term, "we are at the alternate screen!");
        thread::sleep(time::Duration::from_secs(3));
    } // <- Crossterm goes out of scope.

    println!("Whe are back at the main screen");
}
