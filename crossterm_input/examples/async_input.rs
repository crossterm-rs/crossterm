extern crate crossterm_input;
// this is included to enable raw mode
extern crate crossterm_screen;
use crossterm_screen::Screen;
// this is included to enable writing in raw mode
extern crate crossterm_utils;
use crossterm_utils::write;
use std::fmt::Write;


use crossterm_input::{input, TerminalInput, parse_event, InputEvent, KeyEvent, MouseEvent, MouseButton};

use std::io::{Read};
use std::{thread, time};

/// this will capture the input until the given key.
/// TODO: make sure terminal is in raw mode before this function is called.
/// for more information checkout `crossterm_screen` or  use crossterm with the `screen` feature flag.
pub fn read_async_until() {
    // TODO: make sure terminal is in raw mode.
    // for more information checkout `crossterm_screen` or  use crossterm with the `screen` feature flag.

    // init some modules we use for this demo
    let screen = Screen::new(true);
    let input = TerminalInput::from_output(&screen.stdout);

    input.enable_mouse_mode().unwrap();

    let mut stdin = input.read_async().bytes();
    for _i in 0..100 {
        let mut buf = Vec::new();
        let mut switch = true;
        while switch {
            let a = stdin.next();
            if a.is_none() {
                thread::sleep(time::Duration::from_millis(100));
                switch = false;
            } else {
                buf.push(a.unwrap().unwrap() as char);
            }
        }
        println!("{:?}", buf);
    }
    //     let a = stdin.next();
    //     if a.is_none() {
    //         thread::sleep(time::Duration::from_millis(100));
    //         continue
    //     } else {
    //         let event = parse_event(a.unwrap().unwrap(), &mut stdin);
    //         match event.unwrap() {
    //             InputEvent::Keyboard(k) => {
    //                 match k {
    //                     KeyEvent::Char(c) => {
    //                         match c {
    //                             '\n' => {
    //                                 let mut msg = String::new();
    //                                 write!(msg, "{}", "The enter key is hit and the program is not listening to input anymore.\n\n").unwrap();
    //                                 write(&Some(&screen.stdout), msg).unwrap();

    //                                 break;
    //                             },
    //                             'q' => {
    //                                 let mut msg = String::new();
    //                                 write!(msg, "{}", "The 'q' key is hit and the program is not listening to input anymore.\n\n").unwrap();
    //                                 write(&Some(&screen.stdout), msg).unwrap();

    //                                 break;
    //                             },
    //                             _ => {
    //                                 let mut msg = String::new();
    //                                 write!(msg, "{}", format!("'{}' pressed\n\n", c)).unwrap();
    //                                 write(&Some(&screen.stdout), msg).unwrap();
    //                             }
    //                         }
    //                     },
    //                     KeyEvent::Alt(c) => {
    //                         let mut msg = String::new();
    //                         write!(msg, "{}", format!("alt+'{}' pressed\n\n", c)).unwrap();
    //                         write(&Some(&screen.stdout), msg).unwrap();
    //                     },
    //                     KeyEvent::Ctrl(c) => {
    //                         let mut msg = String::new();
    //                         write!(msg, "{}", format!("ctrl+'{}' pressed\n\n", c)).unwrap();
    //                         write(&Some(&screen.stdout), msg).unwrap();
    //                     },
    //                     KeyEvent::Esc => {
    //                         let mut msg = String::new();
    //                         write!(msg, "{}", format!("esc pressed\n\n")).unwrap();
    //                         write(&Some(&screen.stdout), msg).unwrap();
    //                     },
    //                     _ => { () }
    //                 }
    //             },
    //             InputEvent::Mouse(m) => {
    //                 match m {
    //                     MouseEvent::Press(b, x, y) => {
    //                         match b {
    //                             MouseButton::Left => {
    //                                 let mut msg = String::new();
    //                                 write!(msg, "{}", format!("left mouse press @ {}, {}\n\n", x, y)).unwrap();
    //                                 write(&Some(&screen.stdout), msg).unwrap();
    //                             },
    //                             MouseButton::Right => {
    //                                 let mut msg = String::new();
    //                                 write!(msg, "{}", format!("right mouse press @ {}, {}\n\n", x, y)).unwrap();
    //                                 write(&Some(&screen.stdout), msg).unwrap();
    //                             },
    //                             MouseButton::Middle => {
    //                                 let mut msg = String::new();
    //                                 write!(msg, "{}", format!("mid mouse press @ {}, {}\n\n", x, y)).unwrap();
    //                                 write(&Some(&screen.stdout), msg).unwrap();
    //                             },
    //                             MouseButton::WheelUp => {
    //                                 let mut msg = String::new();
    //                                 write!(msg, "{}", format!("wheel up @ {}, {}\n\n", x, y)).unwrap();
    //                                 write(&Some(&screen.stdout), msg).unwrap();
    //                             },
    //                             MouseButton::WheelDown => {
    //                                 let mut msg = String::new();
    //                                 write!(msg, "{}", format!("wheel down @ {}, {}\n\n", x, y)).unwrap();
    //                                 write(&Some(&screen.stdout), msg).unwrap();
    //                             },
    //                         }
    //                     },
    //                     MouseEvent::Release(x, y) => {
    //                         let mut msg = String::new();
    //                         write!(msg, "{}", format!("mouse released @ {}, {}\n\n", x, y)).unwrap();
    //                         write(&Some(&screen.stdout), msg).unwrap();
    //                     },
    //                     MouseEvent::Hold(x, y) => {
    //                         let mut msg = String::new();
    //                         write!(msg, "{}", format!("dragging @ {}, {}\n\n", x, y)).unwrap();
    //                         write(&Some(&screen.stdout), msg).unwrap();
    //                     }
    //                 }
    //             },
    //             _ => { () }
    //         };
    //     };
    //     thread::sleep(time::Duration::from_millis(100));
    // }

    input.disable_mouse_mode().unwrap();
}

/// this will read pressed characters async until `x` is typed.
/// TODO: make sure terminal is in raw mode before this function is called.
/// for more information checkout `crossterm_screen` or  use crossterm with the `screen` feature flag.
pub fn read_async() {
    let input = input();

    let mut stdin = input.read_async().bytes();

    for _i in 0..100 {
        let a = stdin.next();

        println!("pressed key: {:?}", a);

        if let Some(Ok(b'x')) = a {
            println!("The key: `x` was pressed and program is terminated.");
            break;
        }

        thread::sleep(time::Duration::from_millis(50));
    }
}

/// TODO: make sure terminal is in raw mode before this function is called.
/// for more information checkout `crossterm_screen` or  use crossterm with the `screen` feature flag.
// NOTE (@imdaveho): below is for reference; it is not standalone
// as it requires the crossterm_terminal module
// pub fn read_async_demo() {
//     // init some modules we use for this demo
//     let input = input();

//     // this will setup the async reading.
//     let mut stdin = input.read_async().bytes();

//     // clear terminal and reset the cursor.
//     terminal.clear(ClearType::All);
//     cursor.goto(1, 1);

//     // loop until the enter key (\r) is pressed.
//     loop {
//         terminal.clear(ClearType::All);
//         cursor.goto(1, 1);

//         // get the next pressed key
//         let pressed_key = stdin.next();
//         terminal.write(format!("{:?}    <- Character pressed", pressed_key));

//         // check if pressed key is enter (\r)
//         if let Some(Ok(b'\r')) = pressed_key {
//             break;
//         }

//         // wait 200 ms and reset cursor write
//         thread::sleep(Duration::from_millis(200));
//     }
// }

/// TODO: make sure terminal is in raw mode before this function is called.
/// for more information checkout `crossterm_screen` or  use crossterm with the `screen` feature flag.
// NOTE (@imdaveho): below is for reference; it is not standalone
// as it requires the crossterm_screen module
// pub fn async_reading_on_alternate_screen() {
//     let screen = Screen::new(false);

//     // switch to alternate screen
//     if let Ok(alternate) = screen.enable_alternate_modes(true) {
//         let crossterm = Crossterm::from_screen(&alternate.screen);
//         // init some modules we use for this demo
//         let input = crossterm.input();
//         let terminal = crossterm.terminal();
//         let mut cursor = crossterm.cursor();

//         // this will setup the async reading.
//         let mut stdin = input.read_async().bytes();

//         // loop until the enter key (\r) is pressed.
//         loop {
//             terminal.clear(ClearType::All);
//             cursor.goto(1, 1);

//             // get the next pressed key
//             let pressed_key = stdin.next();

//             terminal.write(format!("{:?}    <- Character pressed", pressed_key));

//             // check if pressed key is enter (\r)
//             if let Some(Ok(b'\r')) = pressed_key {
//                 break;
//             }

//             // wait 200 ms and reset cursor write
//             thread::sleep(Duration::from_millis(200));
//         }
//     }
// }

fn main() {
    // un-comment below and run with
    // `cargo run --example async_input`:

    // read_async();
    read_async_until();
}
