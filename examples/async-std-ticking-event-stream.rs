use std::pin::Pin;
use std::thread;
use std::time::Duration;

use async_std::task;
use futures::{
    future::FutureExt,
    select,
    task::{Context, Poll},
    Stream, StreamExt,
};
use futures_timer::Delay;

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

async fn print_ticking_events() {
    let mut reader = EventReader;

    loop {
        let mut delay = Delay::new(Duration::from_millis(100)).fuse();
        let mut event = reader.next().fuse();

        select! {
            _ = delay => { println!("Tick tock\r"); },
            maybe_event = event => {
                if let Some(Ok(event)) = maybe_event {
                    println!("{:?}\r", event);

                    if event == Event::Key(KeyEvent::Char('c')) {
                        println!("Cursor position: {:?}\r", position());
                    }

                    if event == Event::Key(KeyEvent::Esc) {
                        break;
                    }
                }
            }
        };
    }
}

fn main() {
    let _r = RawScreen::into_raw_mode().unwrap();
    task::block_on(print_ticking_events());
}
