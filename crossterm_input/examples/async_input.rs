extern crate crossterm_input;

use self::crossterm_input::input;

use std::io::{stdout, Read, Write};
use std::time::Duration;
use std::{thread, time};

/// this will capture the input until the given key.
/// TODO: make sure terminal is in raw mode before this function is called.
/// for more information checkout `crossterm_screen` or  use crossterm with the `screen` feature flag.
pub fn read_async_until() {
    // TODO: make sure terminal is in raw mode.
    // for more information checkout `crossterm_screen` or  use crossterm with the `screen` feature flag.

    // init some modules we use for this demo
    let input = input();

    let mut stdin = input.read_until_async(b'\r').bytes();

    for _i in 0..100 {
        let a = stdin.next();

        println!("pressed key: {:?}", a);

        if let Some(Ok(b'\r')) = a {
            println!("The enter key is hit and program is not listening to input anymore.");
            break;
        }

        if let Some(Ok(b'x')) = a {
            println!("The key: x was pressed and program is terminated.");
            break;
        }

        thread::sleep(time::Duration::from_millis(100));
    }
}

/// this will read pressed characters async until `x` is typed.
/// TODO: make sure terminal is in raw mode before this function is called.
/// for more information checkout `crossterm_screen` or  use crossterm with the `screen` feature flag.
pub fn read_async() {
    let input = input();

    let mut stdin = input.read_async().bytes();

    for _i in 0..100 {
        let a = stdin.next();

        println!("pressed key: {:?}", a);

        if let Some(Ok(b'x')) = a {
            println!("The key: `x` was pressed and program is terminated.");
            break;
        }

        thread::sleep(time::Duration::from_millis(50));
    }
}

/// TODO: make sure terminal is in raw mode before this function is called.
/// for more information checkout `crossterm_screen` or  use crossterm with the `screen` feature flag.
pub fn read_async_demo() {
    // init some modules we use for this demo
    let input = input();

    // this will setup the async reading.
    let mut stdin = input.read_async().bytes();

    // clear terminal and reset the cursor.
    terminal.clear(ClearType::All);
    cursor.goto(1, 1);

    // loop until the enter key (\r) is pressed.
    loop {
        terminal.clear(ClearType::All);
        cursor.goto(1, 1);

        // get the next pressed key
        let pressed_key = stdin.next();
        terminal.write(format!("{:?}    <- Character pressed", pressed_key));

        // check if pressed key is enter (\r)
        if let Some(Ok(b'\r')) = pressed_key {
            break;
        }

        // wait 200 ms and reset cursor write
        thread::sleep(Duration::from_millis(200));
    }
}

/// TODO: make sure terminal is in raw mode before this function is called.
/// for more information checkout `crossterm_screen` or  use crossterm with the `screen` feature flag.
pub fn async_reading_on_alternate_screen() {
    let screen = Screen::new(false);

    // switch to alternate screen
    if let Ok(alternate) = screen.enable_alternate_modes(true) {
        let crossterm = Crossterm::from_screen(&alternate.screen);
        // init some modules we use for this demo
        let input = crossterm.input();
        let terminal = crossterm.terminal();
        let mut cursor = crossterm.cursor();

        // this will setup the async reading.
        let mut stdin = input.read_async().bytes();

        // loop until the enter key (\r) is pressed.
        loop {
            terminal.clear(ClearType::All);
            cursor.goto(1, 1);

            // get the next pressed key
            let pressed_key = stdin.next();

            terminal.write(format!("{:?}    <- Character pressed", pressed_key));

            // check if pressed key is enter (\r)
            if let Some(Ok(b'\r')) = pressed_key {
                break;
            }

            // wait 200 ms and reset cursor write
            thread::sleep(Duration::from_millis(200));
        }
    }
}

fn main() {}
