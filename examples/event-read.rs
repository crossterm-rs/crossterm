//! Demonstrates how to block read events.
//!
//! cargo run --example event-read

use std::io::{stdout, Write};

use crossterm::{
    cursor::position,
    event::{read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
    Result,
};

const HELP: &str = r#"Blocking read()
 - Keyboard, mouse and terminal resize events enabled
 - Hit "c" to print current cursor position
 - Use Esc to quit
"#;

fn print_events() -> Result<()> {
    loop {
        // Blocking read
        let event = read()?;

        println!("Event: {:?}\r", event);

        if event == Event::Key(KeyCode::Char('c').into()) {
            println!("Cursor position: {:?}\r", position());
        }

        if event == Event::Key(KeyCode::Esc.into()) {
            break;
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    println!("{}", HELP);

    enable_raw_mode()?;

    let mut stdout = stdout();
    execute!(stdout, EnableMouseCapture)?;

    if let Err(e) = print_events() {
        println!("Error: {:?}\r", e);
    }

    execute!(stdout, DisableMouseCapture)?;

    disable_raw_mode()
}
