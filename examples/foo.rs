use crossterm::{
    cursor::{Hide, Show},
    execute,
    screen::AlternateScreen,
};
use std::{
    io::{stdout, Write},
    thread,
    time::Duration,
};

fn main() {
    let mut stdout = stdout();

    println!("Hide");
    execute!(stdout, Hide);

    thread::sleep(Duration::from_millis(2000));

    println!("Show");
    execute!(stdout, Show);

    thread::sleep(Duration::from_millis(2000));

    println!("Hide");
    execute!(stdout, Hide);

    thread::sleep(Duration::from_millis(2000));

    println!("Show");
    execute!(stdout, Show);

    thread::sleep(Duration::from_millis(2000));

    println!("Hide");
    execute!(stdout, Hide);

    thread::sleep(Duration::from_millis(2000));

    println!("Show");
    execute!(stdout, Show);
}
