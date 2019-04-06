extern crate crossterm;

use crossterm::{InputEvent, KeyEvent, MouseButton, MouseEvent, Screen, TerminalInput};

use std::{thread, time::Duration};

fn process_input_event(key_event: InputEvent, screen: &Screen) -> bool {
    match key_event {
        InputEvent::Keyboard(k) => match k {
            KeyEvent::Char(c) => match c {
                'q' => {
                    screen.stdout.write_str("The 'q' key is hit and the program is not listening to input anymore.\n\n").unwrap();
                    return true;
                }
                _ => {
                    screen
                        .stdout
                        .write_string(format!("'{}' pressed\n\n", c))
                        .unwrap();
                }
            },
            KeyEvent::Alt(c) => {
                screen
                    .stdout
                    .write_string(format!("ALT +'{}' pressed\n\n", c))
                    .unwrap();
            }
            KeyEvent::Ctrl(c) => {
                screen
                    .stdout
                    .write_string(format!("CTRL +'{}' Pressed\n\n", c))
                    .unwrap();
            }
            KeyEvent::Esc => {
                screen
                    .stdout
                    .write_string(format!("ESC pressed\n\n"))
                    .unwrap();
            }
            KeyEvent::F(number) => {
                screen
                    .stdout
                    .write_string(format!("F{} key pressed\n\n", number))
                    .unwrap();
            }
            KeyEvent::PageUp => {
                screen.stdout.write_string(format!("Page Up\n\n")).unwrap();
            }
            KeyEvent::PageDown => {
                screen
                    .stdout
                    .write_string(format!("Page Down\n\n"))
                    .unwrap();
            }
            KeyEvent::Delete => {
                screen.stdout.write_string(format!("Delete\n\n")).unwrap();
            }
            _ => {
                screen
                    .stdout
                    .write_string(format!("OTHER: {:?}\n\n", k))
                    .unwrap();
                ()
            }
        },
        InputEvent::Mouse(m) => match m {
            MouseEvent::Press(b, x, y) => match b {
                MouseButton::Left => {
                    screen
                        .stdout
                        .write_string(format!("left mouse press @ {}, {}\n\n", x, y))
                        .unwrap();
                }
                MouseButton::Right => {
                    screen
                        .stdout
                        .write_string(format!("right mouse press @ {}, {}\n\n", x, y))
                        .unwrap();
                }
                MouseButton::Middle => {
                    screen
                        .stdout
                        .write_string(format!("mid mouse press @ {}, {}\n\n", x, y))
                        .unwrap();
                }
                MouseButton::WheelUp => {
                    screen
                        .stdout
                        .write_string(format!("wheel up @ {}, {}\n\n", x, y))
                        .unwrap();
                }
                MouseButton::WheelDown => {
                    screen
                        .stdout
                        .write_string(format!("wheel down @ {}, {}\n\n", x, y))
                        .unwrap();
                }
            },
            MouseEvent::Release(x, y) => {
                screen
                    .stdout
                    .write_string(format!("mouse released @ {}, {}\n\n", x, y))
                    .unwrap();
            }
            MouseEvent::Hold(x, y) => {
                screen
                    .stdout
                    .write_string(format!("dragging @ {}, {}\n\n", x, y))
                    .unwrap();
            }
            _ => {
                screen.stdout.write_str("Unknown mouse event").unwrap();
            }
        },
        _ => println!("Unknown!"),
    }

    return false;
}

pub fn read_asynchronously() {
    // make sure to enable raw mode, this will make sure key events won't be handled by the terminal it's self and allows crossterm to read the input and pass it back to you.
    let screen = Screen::new(true);

    let input = TerminalInput::from_output(&screen.stdout);

    // enable mouse events to be captured.
    input.enable_mouse_mode().unwrap();

    let mut stdin = input.read_async();

    loop {
        if let Some(key_event) = stdin.next() {
            if process_input_event(key_event, &screen) {
                break;
            }
        }
        thread::sleep(Duration::from_millis(50));
    }

    // disable mouse events to be captured.
    input.disable_mouse_mode().unwrap();
} // <=== background reader will be disposed when dropped.

pub fn read_synchronously() {
    // make sure to enable raw mode, this will make sure key events won't be handled by the terminal it's self and allows crossterm to read the input and pass it back to you.
    let screen = Screen::new(true);

    let mut input = TerminalInput::from_output(&screen.stdout);

    // enable mouse events to be captured.
    input.enable_mouse_mode().unwrap();

    let mut sync_stdin = input.read_sync();

    loop {
        let event = sync_stdin.next();

        if let Some(key_event) = event {
            if process_input_event(key_event, &screen) {
                break;
            }
        }
    }

    // disable mouse events to be captured.
    input.disable_mouse_mode().unwrap();
}

fn test() {
    use crossterm::{InputEvent, KeyEvent, Screen};

        let screen = Screen::new(true);

        let alternate =  screen.enable_alternate_modes(true).unwrap();

        let crossterm = crossterm::Crossterm::from_screen(&alternate.screen);
        let mut input = crossterm.input();
        let mut crosstem_events = input.read_sync();
        loop {
            println!("wait for event");
            let event = crosstem_events.next();
            println!(" => crossterm event={:?}", event);
            if let Some(InputEvent::Keyboard(KeyEvent::Ctrl('q'))) = event {
                println!("ctrl-Q");
                break;
            }
        }
}

fn main() {
    // un-comment below and run with
    // `cargo run --example key_events`:
    // read_synchronously();
//    read_asynchronously();
    test();
}
