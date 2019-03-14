extern crate crossterm_input;
extern crate crossterm_screen;
extern crate crossterm_utils;

use crossterm_input::{parse_event, InputEvent, KeyEvent, MouseButton, MouseEvent, TerminalInput};
use crossterm_screen::Screen;
use crossterm_utils::write;

use std::io::Read;
use std::io::Write;
use std::{thread, time};

pub fn read_async() {
    let mut screen = Screen::new(true);

    let input = TerminalInput::from_output(&screen.stdout);
    input.enable_mouse_mode().unwrap();

    let stdin = input.read_async();
    let mut stdin = stdin.start_receiving();

    for i in 0..100 {
        if let Some(key_event) = stdin.next() {
            match key_event {
                InputEvent::Keyboard(k) => match k {
                    KeyEvent::Char(c) => match c {
                        '\n' => {
                            screen.stdout.write_str("The enter key is hit and the program is not listening to input anymore.\n\n").unwrap();
//                            break;
                        }
                        'q' => {
                            screen.stdout.write_str("The 'q' key is hit and the program is not listening to input anymore.\n\n").unwrap();
                            break;
                        }
                        _ => {
                            screen
                                .stdout
                                .write_string(format!("'{}' pressed\n\n", c));
                        }
                    },
                    KeyEvent::Alt(c) => {
                        screen
                            .stdout
                            .write_string(format!("alt+'{}' pressed\n\n", c));
                    }
                    KeyEvent::Ctrl(c) => {
                        screen
                            .stdout
                            .write_string(format!("ctrl+'{}' pressed\n\n", c));
                    }
                    KeyEvent::Esc => {
                        screen
                            .stdout
                            .write_string(format!("esc pressed\n\n"));
                    }
                    _ => (),
                },
                InputEvent::Mouse(m) => match m {
                    MouseEvent::Press(b, x, y) => match b {
                        MouseButton::Left => {
                            screen
                                .stdout
                                .write_string(format!("left mouse press @ {}, {}\n\n", x, y));
                        }
                        MouseButton::Right => {
                            screen
                                .stdout
                                .write_string(format!("right mouse press @ {}, {}\n\n", x, y));
                        }
                        MouseButton::Middle => {
                            screen
                                .stdout
                                .write_string(format!("mid mouse press @ {}, {}\n\n", x, y));
                        }
                        MouseButton::WheelUp => {
                            screen
                                .stdout
                                .write_string(format!("wheel up @ {}, {}\n\n", x, y));
                        }
                        MouseButton::WheelDown => {
                            screen
                                .stdout
                                .write_string(format!("wheel down @ {}, {}\n\n", x, y));
                        }
                    },
                    MouseEvent::Release(x, y) => {
                        screen
                            .stdout
                            .write_string(format!("mouse released @ {}, {}\n\n", x, y));
                    }
                    MouseEvent::Hold(x, y) => {
                        screen
                            .stdout
                            .write_string(format!("dragging @ {}, {}\n\n", x, y));
                    }
                },
                _ => println!("Unknown!"),
            }
        } else {
            println!("None Key");
        }
    }

    input.disable_mouse_mode().unwrap();
}

fn main() {
    // un-comment below and run with
    // `cargo run --example async_input`:

    read_async();
}
