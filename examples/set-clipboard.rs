//! Demonstrates how to set the clipboard.
//!
//! cargo run --example set-clipboard

use std::io::stdout;

use crossterm::{
    event::{read, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, ClipboardKind, SetClipboard},
    Result,
};

const HELP: &str = r#"Set clipboard
 - The clipboard is set to typed characters
 - Use Esc to quit
"#;

fn set_clipboard() -> Result<()> {
    loop {
        // Blocking read
        let event = read()?;

        println!("Event: {:?}\r", event);
        if let Event::Key(KeyEvent {
            code: KeyCode::Char(typed),
            ..
        }) = event
        {
            execute!(
                stdout(),
                SetClipboard::new(&typed.to_string(), ClipboardKind::Clipboard)
            )?;
            println!("Set clipboard to {}\r", typed);
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

    if let Err(e) = set_clipboard() {
        println!("Error: {:?}\r", e);
    }

    disable_raw_mode()
}
