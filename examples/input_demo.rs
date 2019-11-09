#![allow(dead_code)]

use crossterm::{
    event::{poll, read, EnableMouseCapture, Event, KeyEvent},
    execute,
    screen::RawScreen,
};
use std::{io::Write, time::Duration};

fn main() {
    let _r = RawScreen::into_raw_mode().unwrap();
    read_sync(ReadMode::ReadWithTimeout);
}

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

fn handle_event(event: &Event) -> bool {
    println!("{:?}\r", event);
    *event == Event::Key(KeyEvent::Esc)
}
