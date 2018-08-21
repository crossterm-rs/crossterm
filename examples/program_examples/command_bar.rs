extern crate crossterm;

use crossterm::{Screen, Crossterm};
use crossterm::terminal::{terminal,Terminal, ClearType};
use crossterm::cursor::TerminalCursor;

use std::sync::{Arc,Mutex};
use std::io::Read;
use std::{thread,time};

fn main() {
    use crossterm::color;

    let screen = Screen::new(true);
    let crossterm = Crossterm::new(&screen);

    let input = crossterm.input();
    let terminal = crossterm.terminal();
    let cursor = crossterm.cursor();

    let mut stdin = input.read_async().bytes();
    cursor.hide();

    let mut input_buf = Arc::new(Mutex::new(String::new()));

    let mut count = 0;

    let threads = log(input_buf.clone());

    loop
    {
        let a = stdin.next();

        match a {
            Some(Ok(13)) =>
            {
                input_buf.lock().unwrap().clear();

                // need to start receiving again because if pressed enter then async reading will stop
                stdin = input.read_async().bytes();
            }
            Some(Ok(val)) =>
            {
                input_buf.lock().unwrap().push(val as char);
            }
            _ => {}
        }

        thread::sleep(time::Duration::from_millis(100));
        count += 1;
    }

    for thread in threads
    {
        thread.join();
    }

    cursor.show();
}

fn log(input_buf: Arc<Mutex<String>>) -> Vec<thread::JoinHandle<()>>
{
    let mut threads = Vec::with_capacity(10);
    let (_, term_height) = terminal(&Screen::default()).terminal_size();

    for i in 0..10
    {
        let input_buffer = input_buf.clone();

        let join = thread::spawn( move || {

            let crossterm = Crossterm::new(&Screen::new(true));
            let cursor = crossterm.cursor();
            let terminal = crossterm.terminal();

            for j in 0..1000
            {
                swap_write(format!("Some output: {} from thread: {}", j, i).as_ref(), &input_buffer.lock().unwrap(), &terminal, &cursor, term_height);
                thread::sleep(time::Duration::from_millis(300));
            }
        });

        threads.push(join);
    }

    return threads;
}

pub fn swap_write(msg: &str, input_buf: &String, terminal: &Terminal, cursor: &TerminalCursor, term_height: u16) {
    cursor.goto(0, term_height);
    terminal.clear(ClearType::CurrentLine);
    terminal.write(format!("{}\r\n", msg));
    terminal.write(format!(">{}", input_buf));
}