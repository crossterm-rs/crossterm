extern crate crossterm;

use crossterm::cursor::{cursor, TerminalCursor};
use crossterm::input::input;
use crossterm::terminal::{from_screen, ClearType, Terminal};
use crossterm::{Crossterm, Screen};
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::{thread, time};

fn main() {
    use crossterm::color;

    let screen = Screen::new(true);
    let crossterm = Crossterm::from_screen(&screen);
    let cursor = crossterm.cursor();
    cursor.hide();

    let input_buf = Arc::new(Mutex::new(String::new()));

    let threads = log(input_buf.clone(), &screen);

    let mut count = 0;

    thread::spawn(move || {
        let input = input();
        let mut stdin = input.read_async().bytes();

        loop {
            let a = stdin.next();

            match a {
                Some(Ok(13)) => {
                    input_buf.lock().unwrap().clear();
                }
                Some(Ok(val)) => {
                    input_buf.lock().unwrap().push(val as char);
                }
                _ => {}
            }

            thread::sleep(time::Duration::from_millis(100));
            count += 1;
        }
    })
    .join();

    for thread in threads {
        thread.join();
    }

    cursor.show();
}

fn log(input_buf: Arc<Mutex<String>>, screen: &Screen) -> Vec<thread::JoinHandle<()>> {
    let mut threads = Vec::with_capacity(10);

    let (_, term_height) = from_screen(screen).terminal_size();

    for i in 0..1 {
        let input_buffer = input_buf.clone();
        let _clone_stdout = screen.stdout.clone();

        let crossterm = Crossterm::from(screen.stdout.clone());

        let join = thread::spawn(move || {
            let cursor = crossterm.cursor();
            let terminal = crossterm.terminal();

            for j in 0..1000 {
                swap_write(
                    format!("Some output: {} from thread: {}", j, i).as_ref(),
                    &input_buffer.lock().unwrap(),
                    &terminal,
                    &cursor,
                    term_height,
                );
                thread::sleep(time::Duration::from_millis(300));
            }
        });

        threads.push(join);
    }

    return threads;
}

pub fn swap_write(
    msg: &str,
    input_buf: &String,
    terminal: &Terminal,
    cursor: &TerminalCursor,
    term_height: u16,
) {
    cursor.goto(0, term_height);
    terminal.clear(ClearType::CurrentLine);
    terminal.write(format!("{}\r\n", msg));
    terminal.write(format!(">{}", input_buf));
}
