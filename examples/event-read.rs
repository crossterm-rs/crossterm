//
// cargo run --example event-read
//
use std::io::{stdout, Write};

use crossterm::{
    cursor::position,
    event::{read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    screen::RawScreen,
    Result,
};
use crossterm::event::MouseEvent;
use std::time::Instant;

const HELP: &str = r#"Blocking read()
 - Keyboard, mouse and terminal resize events enabled
 - Hit "c" to print current cursor position
 - Use Esc to quit
"#;

fn print_events() -> Result<()> {

    let mut counter = 0;
    let mut instant = Instant::now();


    loop {
        // Blocking read
        let event = read()?;

        println!("Event::{:?}, avg: {}\r", event, instant.elapsed().as_secs() / counter);

        if let Event::Mouse(m) = event {
            counter += 1;
        }

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

    let _r = RawScreen::into_raw_mode()?;

    let mut stdout = stdout();
    execute!(stdout, EnableMouseCapture)?;

    if let Err(e) = print_events() {
        println!("Error: {:?}\r", e);
    }

    execute!(stdout, DisableMouseCapture)?;
    Ok(())
}
