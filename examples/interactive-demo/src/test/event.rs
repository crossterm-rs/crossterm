#![allow(clippy::cognitive_complexity)]

use crossterm::{
    cursor::position,
    event::{read, EnableMouseCapture, Event, KeyCode},
    execute, Result,
};
use std::io::Write;

fn test_event<W>(w: &mut W) -> Result<()>
where
    W: Write,
{
    execute!(w, EnableMouseCapture)?;

    loop {
        // Blocking read
        let event = read()?;

        println!("Event::{:?}\r", event);

        if event == Event::Key(KeyCode::Char('c').into()) {
            println!("Cursor position: {:?}\r", position());
        }

        if event == Event::Key(KeyCode::Char('q').into()) {
            break;
        }
    }

    Ok(())
}

pub fn run<W>(w: &mut W) -> Result<()>
where
    W: Write,
{
    run_tests!(w, test_event);
    Ok(())
}
