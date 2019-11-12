use std::pin::Pin;
use std::thread;
use std::time::Duration;

use futures::{
    task::{Context, Poll},
    Stream, StreamExt,
};

use crossterm::{
    cursor::position,
    event::{poll, read, Event, KeyEvent},
    screen::RawScreen,
    Result,
};

struct EventReader;

impl Stream for EventReader {
    type Item = Result<Event>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        eprintln!("EventReader::poll_next\r");
        let result = match poll(Some(Duration::from_secs(0))) {
            Ok(true) => {
                eprintln!(" - poll -> Ok(true)\r");
                Poll::Ready(Some(read()))
            }
            Ok(false) => {
                eprintln!(" - poll -> Ok(false)\r");
                let waker = cx.waker().clone();
                thread::spawn(move || {
                    loop {
                        if let Ok(true) = poll(None) {
                            break;
                        }
                    }
                    eprintln!(" - wake\r");
                    waker.wake();
                });
                Poll::Pending
            }
            Err(e) => {
                eprintln!(" - poll -> Err({:?})\r", e);
                Poll::Ready(Some(Err(e)))
            }
        };
        eprintln!("EventReader::poll_next -> {:?}\r", result);
        result
    }
}

#[tokio::main]
async fn main() {
    let _r = RawScreen::into_raw_mode().unwrap();

    let mut reader = EventReader;

    while let Some(maybe_event) = reader.next().await {
        if let Ok(event) = maybe_event {
            println!("{:?}\r", event);

            if event == Event::Key(KeyEvent::Char('c')) {
                println!("Cursor position: {:?}\r", position());
            }

            if event == Event::Key(KeyEvent::Esc) {
                break;
            }
        }
    }
}
