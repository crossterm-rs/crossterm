use std::time::Duration;

use crossterm::{
    input::{poll, read, Event, KeyEvent},
    screen::RawScreen,
};

fn main() {
    let r = RawScreen::into_raw_mode().unwrap();
    read_async();
}

fn sync_read1() {
    loop {
        match read() {
            Ok(event) => {
                if handle_event(&event) {
                    break;
                }
            }
            Err(_) => {
                // `read()` error
            }
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
        match poll(None).and_then(|succeed| read()) {
            Ok(event) => {
                if handle_event(&event) {
                    break;
                }
            }
            Err(_) => { /* Error when reading */ }
        }
    }
}

fn read_async() {
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
