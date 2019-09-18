#![allow(dead_code)]

use std::{thread, time::Duration};

use crossterm::{input, InputEvent, KeyEvent, MouseButton, MouseEvent, RawScreen, Result};

fn process_input_event(key_event: InputEvent) -> bool {
    match key_event {
        InputEvent::Keyboard(k) => {
            match k {
                KeyEvent::Char(c) => match c {
                    'q' => {
                        println!("The 'q' key is hit and the program is not listening to input anymore.\n\n");
                        return true;
                    }
                    _ => {
                        println!("{}", format!("'{}' pressed\n\n", c));
                    }
                },
                KeyEvent::Alt(c) => {
                    println!("{}", format!("ALT +'{}' pressed\n\n", c));
                }
                KeyEvent::Ctrl(c) => {
                    println!("{}", format!("CTRL +'{}' Pressed\n\n", c));
                }
                KeyEvent::Esc => {
                    println!("{}", format!("ESC pressed\n\n"));
                }
                KeyEvent::F(number) => {
                    println!("{}", format!("F{} key pressed\n\n", number));
                }
                KeyEvent::PageUp => {
                    println!("{}", format!("Page Up\n\n"));
                }
                KeyEvent::PageDown => {
                    println!("{}", format!("Page Down\n\n"));
                }
                KeyEvent::Delete => {
                    println!("{}", format!("Delete\n\n"));
                }
                _ => {
                    println!("{}", format!("OTHER: {:?}\n\n", k));
                    ()
                }
            }
        }
        InputEvent::Mouse(m) => match m {
            MouseEvent::Press(b, x, y) => match b {
                MouseButton::Left => {
                    println!("{}", format!("left mouse press @ {}, {}\n\n", x, y));
                }
                MouseButton::Right => {
                    println!("{}", format!("right mouse press @ {}, {}\n\n", x, y));
                }
                MouseButton::Middle => {
                    println!("{}", format!("mid mouse press @ {}, {}\n\n", x, y));
                }
                MouseButton::WheelUp => println!("{}", format!("wheel up @ {}, {}\n\n", x, y)),
                MouseButton::WheelDown => {
                    println!("{}", format!("wheel down @ {}, {}\n\n", x, y));
                }
            },
            MouseEvent::Release(x, y) => {
                println!("{}", format!("mouse released @ {}, {}\n\n", x, y));
            }
            MouseEvent::Hold(x, y) => {
                println!("{}", format!("dragging @ {}, {}\n\n", x, y));
            }
            _ => {
                println!("{}", "Unknown mouse event");
            }
        },
        _ => println!("Unknown!"),
    }

    false
}

fn read_asynchronously() -> Result<()> {
    // make sure to enable raw mode, this will make sure key events won't be handled by the terminal it's self and allows crossterm to read the input and pass it back to you.
    let _raw = RawScreen::into_raw_mode()?;

    let input = input();

    // enable mouse events to be captured.
    input.enable_mouse_mode()?;

    let mut stdin = input.read_async();

    loop {
        if let Some(key_event) = stdin.next() {
            if process_input_event(key_event) {
                break;
            }
        }
        thread::sleep(Duration::from_millis(50));
    }

    // disable mouse events to be captured.
    input.disable_mouse_mode()
} // <=== raw modes will be disabled here

fn read_synchronously() -> Result<()> {
    // make sure to enable raw mode, this will make sure key events won't be handled by the terminal it's self and allows crossterm to read the input and pass it back to you.
    let _raw = RawScreen::into_raw_mode()?;

    let input = input();

    // enable mouse events to be captured.
    input.enable_mouse_mode()?;

    let mut sync_stdin = input.read_sync();

    loop {
        let event = sync_stdin.next();

        if let Some(key_event) = event {
            if process_input_event(key_event) {
                break;
            }
        }
    }

    // disable mouse events to be captured.
    input.disable_mouse_mode()
} // <=== raw modes will be disabled here

// cargo run --example key_events
fn main() -> Result<()> {
    read_synchronously()
    // read_asynchronously()
}
