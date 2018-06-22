extern crate crossterm;

use crossterm::Context;
use crossterm::screen::AlternateScreen;
use crossterm::cursor::cursor;
use crossterm::terminal::{self, ClearType};

use std::io::{Write, stdout};
use std::{time, thread};
use std::rc::Rc;
fn print_wait_screen(context: Rc<Context>)
{
    terminal::terminal(context.clone()).clear(ClearType::All);

    let mut cursor = cursor(context.clone());
    cursor.goto(0,0);

    {
        let mut screen_manager = context.screen_manager.lock().unwrap();
        {
            write!(screen_manager,
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
    let context = Context::new();

    // create scope. If this scope ends the screen will be switched back to mainscreen.
    // because `AlternateScreen` switches back to main screen when switching back.
    {
        // create new alternate screen instance and switch to the alternate screen.
        let mut screen = AlternateScreen::from(context.clone());

        // Print the wait screen.
        print_wait_screen(context.clone());
    }

    println!("Whe are back at the main screen");
}

pub fn switch_between_main_and_alternate_screen()
{
    let context = Context::new();
    let mut cursor = cursor(context.clone());

    {
        // create new alternate screen instance and switch to the alternate screen.
        let mut screen = AlternateScreen::from(context.clone());
        cursor.goto(0,0);
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