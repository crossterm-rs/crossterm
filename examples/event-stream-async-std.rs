//
// cargo run --features async-event --example event-stream-async-std
//
use std::io::{stdout, Write};
use std::time::Duration;

use futures::{future::FutureExt, select, StreamExt};
use futures_timer::Delay;

use crossterm::{
    cursor::position,
    event::{DisableMouseCapture, EnableMouseCapture, Event, EventStream, KeyEvent},
    execute,
    screen::RawScreen,
    Result,
};

const HELP: &str = r#"EventStream based on futures::Stream with async-std
 - Keyboard, mouse and terminal resize events enabled
 - Prints "." every second if there's no event
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

                        if event == Event::Key(KeyEvent::Char('c')) {
                            println!("Cursor position: {:?}\r", position());
                        }

                        if event == Event::Key(KeyEvent::Esc) {
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

fn main() -> Result<()> {
    println!("{}", HELP);

    let _r = RawScreen::into_raw_mode()?;

    let mut stdout = stdout();
    execute!(stdout, EnableMouseCapture)?;

    async_std::task::block_on(print_events());

    execute!(stdout, DisableMouseCapture)?;
    Ok(())
}
