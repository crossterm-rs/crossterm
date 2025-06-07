//! Demonstrates how to read events asynchronously with tokio.
//!
//! cargo run --features="event-stream" --example event-stream-tokio-pixels

use std::{io::stdout, time::Duration};

use futures::{future::FutureExt, select, StreamExt};
use futures_timer::Delay;

use crossterm::{
    cursor::position,
    event::{DisableMousePixelCapture, EnableMousePixelCapture, Event, EventStream, KeyCode},
    execute,
    terminal::{self, cell_size, disable_raw_mode, enable_raw_mode},
};

const HELP: &str = r#"EventStream based on futures_util::Stream with tokio
 - Keyboard, mouse and terminal resize events enabled
 - Prints "." every second if there's no event
 - Hit "c" to print current cursor position
 - Hit "s" to print current the cell size in pixels 
 - Use Esc to quit
"#;

async fn print_events() {
    let mut reader = EventStream::new();

    loop {
        let mut delay = Delay::new(Duration::from_millis(1_000)).fuse();
        let mut event = reader.next().fuse();

        select! {
            _ = delay => { println!(".\r"); },
            maybe_event = event => {
                match maybe_event {
                    Some(Ok(event)) => {
                        println!("Event::{:?}\r", event);

                        if event == Event::Key(KeyCode::Char('c').into()) {
                            println!("Cursor position: {:?}\r", position());
                        }

                        if event == Event::Key(KeyCode::Char('s').into()) {
                            println!("CSI Cell size (pixels): {:?}\r", cell_size());

                            let s = terminal::window_size().unwrap();
                            let width = s.width/(s.columns);
                            let height = s.height/(s.rows);
                            println!("Window Calculated Cell size (pixels): {}, {}\r", height, width);
                        }

                        if event == Event::Key(KeyCode::Esc.into()) {
                            break;
                        }
                    }
                    Some(Err(e)) => println!("Error: {:?}\r", e),
                    None => break,
                }
            }
        };
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("{}", HELP);

    enable_raw_mode()?;

    let mut stdout = stdout();
    execute!(stdout, EnableMousePixelCapture)?;

    print_events().await;

    execute!(stdout, DisableMousePixelCapture)?;

    disable_raw_mode()
}
