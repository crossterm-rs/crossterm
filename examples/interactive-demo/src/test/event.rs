#![allow(clippy::cognitive_complexity)]

use crossterm::{
    cursor::position,
    event::{read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
};
use std::io::{self, Write};

fn test_event<W>(w: &mut W) -> io::Result<()>
where
    W: io::Write,
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

    execute!(w, DisableMouseCapture)?;

    Ok(())
}

pub fn run<W>(w: &mut W) -> std::io::Result<()>
where
    W: Write,
{
    run_tests!(w, test_event);
    Ok(())
}
