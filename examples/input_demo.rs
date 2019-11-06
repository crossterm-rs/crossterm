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
                if should_stop(&event) {
                    break;
                }
                println!("{:?}", event);
            }
            Err(_) => {
                // read() error
            }
        }
    }
}

fn sync_read2() {
    loop {
        match poll(None) {
            Ok(_) => {
                match read() {
                    Ok(event) => {
                        if should_stop(&event) {
                            break;
                        }
                        println!("{:?}", event);
                    }
                    Err(_) => {
                        // read() error
                    }
                }
            }
            Err(_) => {
                // poll() error
            }
        }
    }
}

fn sync_read3() {
    loop {
        match poll(None).and_then(|_| read()) {
            Ok(event) => {
                if should_stop(&event) {
                    break;
                }
                println!("{:?}", event);
            }
            Err(e) => println!("{:?}", e),
        }
    }
}

fn read_async() {
    loop {
        match poll(Some(Duration::from_millis(100))) {
            Ok(true) => {
                // Event available - read() wont block
                match read() {
                    Ok(event) => {
                        if should_stop(&event) {
                            break;
                        }
                        println!("{:?}", event);
                    }
                    Err(_) => {
                        // read() error
                    }
                }
            }
            Ok(false) => {
                // Event not available, but 100ms timeout expired
                // 10fps if there's no event available
            }
            Err(_) => {
                // poll() error
            }
        }
    }
}

fn should_stop(event: &Event) -> bool {
    *event == Event::Keyboard(KeyEvent::Esc)
}
