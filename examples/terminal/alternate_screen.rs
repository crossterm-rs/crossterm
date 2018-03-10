extern crate crossterm;

use crossterm::terminal::screen::{AlternateScreen, ToAlternateScreen, ToMainScreen};
use crossterm::cursor::cursor;
use crossterm::terminal::{self, ClearType};

use std::io::{Write, stdout};
use std::{time, thread};

fn print_wait_screen(screen: &mut Write)
{
    terminal::terminal().clear(ClearType::All);
    write!(screen,
    "Welcome to the wait screen.\n\
    Please wait a few seconds until we arrive back at the main screen.\n\
    Seconds to Go: "
    );

    let mut counter = 5;
    // get cursor instance
    let mut cursor = cursor();

    // loop until the counter hits 0
    loop
    {
        // 1 second delay
        thread::sleep(time::Duration::from_secs(1));
        // decrement counter
        counter -= 1;

        // print the current counter at the line of `Seconds to Go: {counter}`
        cursor.goto(15,2).print(counter);

        if counter <= 0
        {
            break;
        }
    }
}

pub fn with_alternate_screen_instance()
{
    // create scope. If this scope ends the screen will be switched back to mainscreen.
    // becouse `AlternateScreen` switches back to main screen when switching back.
    {
        // create new alternate screen instance and switch to the alternate screen.
        let mut screen = AlternateScreen::from(stdout());

        // Print the wait screen.
        print_wait_screen(&mut screen);
    }

    println!("Whe are back at the main screen");
}

pub fn manually_switch_to_alternate_screen()
{
    // You can switch to alternate screen manually but if you forget to switch back your terminal may cause some undefined behavior.

    let mut screen = stdout();

    // switch to alternate screeen
    write!(screen, "{}", ToAlternateScreen);
    // load wait screen
    print_wait_screen(&mut screen);
    // switch back
    write!(screen,"{}", ToMainScreen);
    println!("Whe are back at the main screen");

}