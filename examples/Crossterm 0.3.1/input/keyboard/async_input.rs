extern crate crossterm;

use self::crossterm::input::input;
use self::crossterm::Context;
use self::crossterm::Crossterm;
use crossterm::terminal::ClearType;

use crossterm::raw::IntoRawMode;
use std::{thread, time};

use std::io::{stdout, Read, Write};
use std::time::Duration;

/// this will capture the input until the given key.
pub fn read_async_until() {
    let context = Context::new();
    let input = input(&context);

    let mut stdin = input.read_until_async(b'\r').bytes();

    for i in 0..100 {
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

        thread::sleep(time::Duration::from_millis(50));
    }
}

/// this will read pressed characters async until `x` is typed .
pub fn read_async() {
    let context = Context::new();
    let input = input(&context);

    let mut stdin = input.read_async().bytes();

    for i in 0..100 {
        let a = stdin.next();

        println!("pressed key: {:?}", a);

        if let Some(Ok(b'x')) = a {
            println!("The key: `x` was pressed and program is terminated.");
            break;
        }

        thread::sleep(time::Duration::from_millis(50));
    }
}

pub fn read_async_demo() {
    let crossterm = Crossterm::new();

    // init some modules we use for this demo
    let input = crossterm.input();
    let terminal = crossterm.terminal();
    let mut cursor = crossterm.cursor();

    // put stdout in raw mode so that characters wil not be outputted.
    let mut stdout = stdout().into_raw_mode(crossterm.context()).unwrap();

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
        terminal.write(format!("\r{:?}    <- Character pressed", pressed_key));


        // check if pressed key is enter (\r)
        if let Some(Ok(b'\r')) = pressed_key {
            break;
        }

        // wait 200 ms and reset cursor write
        thread::sleep(Duration::from_millis(200));
    }
}

pub fn async_reading_on_alternate_screen() {
    use crossterm::screen::AlternateScreen;

    let crossterm = Crossterm::new();

    // init some modules we use for this demo
    let input = crossterm.input();
    let terminal = crossterm.terminal();
    let mut cursor = crossterm.cursor();

    // switch to alternate screen
    let mut alternate_screen = AlternateScreen::from(crossterm.context());
    // put alternate screen in raw mode so that characters wil not be outputted.
    let mut raw_screen = alternate_screen.into_raw_mode(crossterm.context());

    // this will setup the async reading.
    let mut stdin = input.read_async().bytes();


    // loop until the enter key (\r) is pressed.
    loop {
        terminal.clear(ClearType::All);
        cursor.goto(1, 1);

        // get the next pressed key
        let pressed_key = stdin.next();

        terminal.write(format!("\r{:?}    <- Character pressed", pressed_key));

        // check if pressed key is enter (\r)
        if let Some(Ok(b'\r')) = pressed_key {
            break;
        }

        // wait 200 ms and reset cursor write
        thread::sleep(Duration::from_millis(200));
    }
}
