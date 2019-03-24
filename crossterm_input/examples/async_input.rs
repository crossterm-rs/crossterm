extern crate crossterm_input;
extern crate crossterm_screen;
extern crate crossterm_utils;

use crossterm_input::{
    input, parse_event, InputEvent, KeyEvent, MouseButton, MouseEvent, TerminalInput,
};
use crossterm_screen::Screen;
use crossterm_utils::write;

use std::fmt::Write;
use std::io::Read;
use std::{thread, time};

pub fn read_async() {
    let screen = Screen::new(true);
    let input = TerminalInput::from_output(&screen.stdout);

    input.enable_mouse_mode().unwrap();

    let mut stdin = input.read_async().bytes();

    for _i in 0..100 {
        let a = stdin.next();
        if a.is_none() {
            thread::sleep(time::Duration::from_millis(100));
            continue;
        } else {
            let event = parse_event(a.unwrap().unwrap(), &mut stdin);
            match event.unwrap() {
                InputEvent::Keyboard(k) => match k {
                    KeyEvent::Char(c) => match c {
                        '\n' => {
                            let mut msg = String::new();
                            write!(msg, "{}", "The enter key is hit and the program is not listening to input anymore.\n\n").unwrap();
                            write(&Some(&screen.stdout), msg).unwrap();

//                            break;
                        }
                        'q' => {
                            let mut msg = String::new();
                            write!(msg, "{}", "The 'q' key is hit and the program is not listening to input anymore.\n\n").unwrap();
                            write(&Some(&screen.stdout), msg).unwrap();

                            break;
                        }
                        _ => {
                            let mut msg = String::new();
                            write!(msg, "{}", format!("'{}' pressed\n\n", c)).unwrap();
                            write(&Some(&screen.stdout), msg).unwrap();
                        }
                    },
                    KeyEvent::Alt(c) => {
                        let mut msg = String::new();
                        write!(msg, "{}", format!("alt+'{}' pressed\n\n", c)).unwrap();
                        write(&Some(&screen.stdout), msg).unwrap();
                    }
                    KeyEvent::Ctrl(c) => {
                        let mut msg = String::new();
                        write!(msg, "{}", format!("ctrl+'{}' pressed\n\n", c)).unwrap();
                        write(&Some(&screen.stdout), msg).unwrap();
                    }
                    KeyEvent::Esc => {
                        let mut msg = String::new();
                        write!(msg, "{}", format!("esc pressed\n\n")).unwrap();
                        write(&Some(&screen.stdout), msg).unwrap();
                    }
                    _ => (),
                },
                InputEvent::Mouse(m) => match m {
                    MouseEvent::Press(b, x, y) => match b {
                        MouseButton::Left => {
                            let mut msg = String::new();
                            write!(msg, "{}", format!("left mouse press @ {}, {}\n\n", x, y))
                                .unwrap();
                            write(&Some(&screen.stdout), msg).unwrap();
                        }
                        MouseButton::Right => {
                            let mut msg = String::new();
                            write!(msg, "{}", format!("right mouse press @ {}, {}\n\n", x, y))
                                .unwrap();
                            write(&Some(&screen.stdout), msg).unwrap();
                        }
                        MouseButton::Middle => {
                            let mut msg = String::new();
                            write!(msg, "{}", format!("mid mouse press @ {}, {}\n\n", x, y))
                                .unwrap();
                            write(&Some(&screen.stdout), msg).unwrap();
                        }
                        MouseButton::WheelUp => {
                            let mut msg = String::new();
                            write!(msg, "{}", format!("wheel up @ {}, {}\n\n", x, y)).unwrap();
                            write(&Some(&screen.stdout), msg).unwrap();
                        }
                        MouseButton::WheelDown => {
                            let mut msg = String::new();
                            write!(msg, "{}", format!("wheel down @ {}, {}\n\n", x, y)).unwrap();
                            write(&Some(&screen.stdout), msg).unwrap();
                        }
                    },
                    MouseEvent::Release(x, y) => {
                        let mut msg = String::new();
                        write!(msg, "{}", format!("mouse released @ {}, {}\n\n", x, y)).unwrap();
                        write(&Some(&screen.stdout), msg).unwrap();
                    }
                    MouseEvent::Hold(x, y) => {
                        let mut msg = String::new();
                        write!(msg, "{}", format!("dragging @ {}, {}\n\n", x, y)).unwrap();
                        write(&Some(&screen.stdout), msg).unwrap();
                    }
                },
                _ => (),
            };
        };
        thread::sleep(time::Duration::from_millis(100));
    }

    input.disable_mouse_mode().unwrap();
}

fn main() {
    // un-comment below and run with
    // `cargo run --example async_input`:

    read_async();
}
