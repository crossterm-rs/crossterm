#![allow(dead_code)]

use std::time::Duration;
use std::io::Write;
use crossterm::{
    event::{poll, read, Event, KeyEvent, EnableMouseCapture},
    screen::RawScreen,
    execute,
};

fn main() {
    execute!(std::io::stdout(), EnableMouseCapture);
    let _r = RawScreen::into_raw_mode().unwrap();
    sync_read2();\


}

fn sync_read1() {
    loop {
        match read() {
            Ok(event) => {
                if handle_event(&event) {
                    break;
                }
            }
            Err(_) => { /* `read()` error */ }
        }
    }
}

fn sync_read2() {
    loop {
        match poll(None) {
            Ok(true) => {
                match read() {
                    Ok(event) => {
                        if handle_event(&event) {
                            break;
                        }
                    }
                    Err(_) => { /* Error when reading */ }
                }
            }
            Ok(false) => { /* not possible, only possible on timeout */ }
            Err(_) => { /* poll() error */ }
        }
    }
}

fn sync_read3() {
    loop {
        match poll(None).and_then(|_| read()) {
            Ok(event) => {
                if handle_event(&event) {
                    break;
                }
            }
            Err(_) => { /* Error when reading */ }
        }
    }
}

fn read_async_1() {
    loop {
        match poll(Some(Duration::from_millis(200))) {
            Ok(true) => {
                // Event available - read() wont block
                match read() {
                    Ok(Event::Key(KeyEvent::Char('c'))) => {
                        let cursor = crossterm::cursor::position();
                        println!("Cursor position: {:?}\r", cursor);
                    }
                    Ok(event) => {
                        if handle_event(&event) {
                            break;
                        }
                    }
                    Err(_) => { /* Error when reading */ }
                }
            }
            Ok(false) => {
                /* Event not available, but 100ms timeout expired  */
                println!(".\r");
            }
            Err(_) => { /* poll() error */ }
        }
    }
}

fn handle_event(event: &Event) -> bool {
    println!("{:?}\r", event);

    *event == Event::Key(KeyEvent::Esc)
}
