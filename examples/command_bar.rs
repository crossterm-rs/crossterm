use std::sync::{Arc, Mutex};
use std::{thread, time};

use crossterm::{
    cursor, input, terminal, ClearType, Crossterm, InputEvent, KeyEvent, RawScreen, Result,
    Terminal, TerminalCursor,
};

fn log(input_buf: Arc<Mutex<String>>) -> Result<Vec<thread::JoinHandle<()>>> {
    let mut threads = Vec::with_capacity(10);

    let (_, term_height) = terminal().size()?;

    for i in 0..1 {
        let input_buffer = input_buf.clone();

        let crossterm = Crossterm::new();

        let join = thread::spawn(move || {
            let cursor = crossterm.cursor();
            let terminal = crossterm.terminal();

            for j in 0..1000 {
                if let Err(_) = swap_write(
                    format!("Some output: {} from thread: {}", j, i).as_ref(),
                    &input_buffer.lock().unwrap(),
                    &terminal,
                    &cursor,
                    term_height,
                ) {
                    return;
                }
                thread::sleep(time::Duration::from_millis(100));
            }
        });

        threads.push(join);
    }

    Ok(threads)
}

fn swap_write(
    msg: &str,
    input_buf: &String,
    terminal: &Terminal,
    cursor: &TerminalCursor,
    term_height: u16,
) -> Result<()> {
    cursor.goto(0, term_height)?;
    terminal.clear(ClearType::CurrentLine)?;
    terminal.write(format!("{}\r\n", msg))?;
    terminal.write(format!("> {}", input_buf)).map(|_| ())
}

// cargo run --example command_bar
fn main() -> Result<()> {
    let _screen = RawScreen::into_raw_mode();
    cursor().hide()?;

    let input_buf = Arc::new(Mutex::new(String::new()));

    let threads = log(input_buf.clone())?;

    let mut count = 0;

    thread::spawn(move || {
        let input = input();
        let mut stdin = input.read_async();

        loop {
            match stdin.next() {
                Some(InputEvent::Keyboard(KeyEvent::Char('\n'))) => {
                    input_buf.lock().unwrap().clear();
                }
                Some(InputEvent::Keyboard(KeyEvent::Char(character))) => {
                    input_buf.lock().unwrap().push(character as char);
                }
                _ => {}
            }

            thread::sleep(time::Duration::from_millis(10));
            count += 1;
        }
    })
    .join()
    .expect("Couldn't join");

    for thread in threads {
        thread.join().expect("Couldn't join");
    }

    cursor().show()
}
