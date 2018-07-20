extern crate crossterm;

use self::crossterm::input::input;
use self::crossterm::Context;

use std::{thread, time};
use std::io::Read;

// this will capture the input until the given key was pressed.
pub fn capture_input_until_a_certain_char_async()
{
    let context = Context::new();
    let input = input(&context);

    let mut stdin = input.read_until_async(b'\r').bytes();

    for i in 0..100
    {
        let a = stdin.next();

        if let Some(Ok(b'x')) = a {
            println!("The key: x was pressed.");
            break;
        }

        thread::sleep(time::Duration::from_millis(50));
    }
}

// this will capture an character input until the given key was pressed.
pub fn read_async()
{
    let context = Context::new();
    let input = input(&context);

    let mut stdin = input.read_async().bytes();

    for i in 0..100
    {
        let a = stdin.next();

        println!("pressed: {:?}", a);

        if let Some(Ok(b'x')) = a {
            println!("The key: x was pressed.");
            break;
        }

        thread::sleep(time::Duration::from_millis(50));
    }
}