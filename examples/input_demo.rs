#![allow(dead_code)]

use crossterm::cursor::position;
use crossterm::{
    event::{poll, read, Event, KeyEvent},
    screen::RawScreen,
};
use futures::{Stream, StreamExt};
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

/// Sync main
fn main() {
    let _r = RawScreen::into_raw_mode().unwrap();
    read_sync(ReadMode::ReadWithoutTimeout)
}

/// Async main
//#[tokio::main]
//async fn main() {
//    let _r = RawScreen::into_raw_mode().unwrap();
//
//    let mut stream = EventStream;
//
//    while let Some(event) = stream.next().await {
//        if let Ok(Event::Key(KeyEvent::Esc)) = event {
//            break;
//        }
//
//        println!("event: {:?}", event);
//    }
//}

// Demonstrates different ways to use the event read api.
fn read_sync(read_mode: ReadMode) {
    loop {
        let result = match read_mode {
            ReadMode::ReadWithoutTimeout => read_without_timeout(),
            ReadMode::ReadWithTimeout => read_with_timeout(),
            ReadMode::ReadWithoutPoll => read_without_poll(),
        };

        match result {
            Ok(event) => {
                if handle_event(&event) {
                    break;
                }
            }
            Err(e) => println!("Error Occurred: {:?}", e),
        };
    }
}

fn read_without_timeout() -> crossterm::Result<Event> {
    loop {
        if poll(None)? {
            return read();
        } else {
            // never happens, will only happen on timeout.
        }
    }
}

fn read_with_timeout() -> crossterm::Result<Event> {
    loop {
        if poll(Some(Duration::from_millis(500)))? {
            return read();
        } else {
            println!("timeout after waiting 500ms\r");
        }
    }
}

fn read_without_poll() -> crossterm::Result<Event> {
    // will be a blocking read
    return read();
}

/// A few different examples to read events with the crossterm API.
enum ReadMode {
    // Reads indefinitely until an event arrives.
    ReadWithoutTimeout,
    /// Reads just for an certain duration.
    ReadWithTimeout,
    /// Reads without polling, this will be a blocking call.
    ReadWithoutPoll,
}

// Prints the key event, or cursor position when c is pressed.
// Returns true if the given key is 'Esc'.
fn handle_event(event: &Event) -> bool {
    match event {
        &Event::Key(KeyEvent::Char('c')) => {
            println!("{:?}\r", position());
        }
        _ => println!("{:?}\r", event),
    }

    *event == Event::Key(KeyEvent::Esc)
}

struct EventStream;

impl Stream for EventStream {
    type Item = crossterm::Result<Event>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        if poll(Some(Duration::from_millis(50))).unwrap() {
            Poll::Ready(Some(read()))
        } else {
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}
