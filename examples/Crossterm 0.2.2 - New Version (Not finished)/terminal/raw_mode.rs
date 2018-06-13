extern crate crossterm;

use crossterm::Terminal;
use crossterm::screen::AlternateScreen;
use crossterm::cursor::cursor;
use crossterm::terminal::{self, ClearType};

use std::io::{Write, stdout};
use std::{time, thread};

use crossterm::raw::IntoRawMode;

// raw screen is not working correctly currently
fn print_wait_screen(terminal: &Terminal)
{
    terminal::terminal(&terminal).clear(ClearType::All);

    let mut cursor = cursor(&terminal);
    cursor.goto(0,0);

    {
        let mut screen_manager = terminal.screen_manager.lock().unwrap();
        {
            write!(screen_manager.stdout(),
                   "Welcome to the wait screen.\n\
                Please wait a few seconds until we arrive back at the main screen.\n\
                Progress: "
            );
        }
    }

    // print some progress example.
    for i in 1..5
        {
            // print the current counter at the line of `Seconds to Go: {counter}`
            cursor.goto(10,2).print(format!("{} of the 5 items processed", i));

            // 1 second delay
            thread::sleep(time::Duration::from_secs(1));
        }
}

pub fn print_wait_screen_on_alternate_window()
{
    let terminal = Terminal::new();

    // create scope. If this scope ends the screen will be switched back to mainscreen.
    // because `AlternateScreen` switches back to main screen when switching back.
    {
        let a = stdout().into_raw_mode(&terminal);

        thread::sleep(time::Duration::from_secs(3));


//        // Print the wait screen.
//        print_wait_screen(&terminal);
    }

    println!("Whe are back at the main screen");
}