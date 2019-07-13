//!
//! Examples of actions that could be performed with the cursor.
//!

extern crate crossterm_cursor;

use crossterm_cursor::cursor;

/// Set the cursor to position X: 10, Y: 5 in the terminal.
pub fn goto() {
    // Get the cursor
    let cursor = cursor();
    // Set the cursor to position X: 10, Y: 5 in the terminal
    cursor.goto(10, 5);
}

/// get the cursor position
pub fn pos() {
    // Get the cursor
    let cursor = cursor();
    // get the cursor position.
    let (x, y) = cursor.pos();

    println!("{} {}", x, y);
}

/// Move the cursor 3 up | demonstration.
pub fn move_up() {
    // Get the cursor
    let mut cursor = cursor();

    // Move the cursor to position 3 times to the up in the terminal
    cursor.move_up(10);
}

/// Move the cursor 3 to the right | demonstration.
pub fn move_right() {
    let mut cursor = cursor();
    // Move the cursor to position 3 times to the right in the terminal
    cursor.move_right(3);
}

/// Move the cursor 3 down | demonstration.
pub fn move_down() {
    let mut cursor = cursor();
    // Move the cursor to position 3 times to the down in the terminal
    cursor.move_down(3);
}

/// Save and reset cursor position | demonstration..
pub fn save_and_reset_position() {
    let cursor = cursor();

    // Goto X: 5 Y: 5
    cursor.goto(5, 5);
    // Safe cursor position: X: 5 Y: 5
    cursor.save_position();
    // Goto X: 5 Y: 20
    cursor.goto(5, 20);
    // Print at X: 5 Y: 20.
    println!("Yea!");
    // Reset back to X: 5 Y: 5.
    cursor.reset_position();
    // Print Back at X: 5 Y: 5.
    println!("Back");

    println!()
}

/// Hide cursor display | demonstration.
pub fn hide_cursor() {
    let cursor = cursor();
    cursor.hide();
}

/// Show cursor display | demonstration.
pub fn show_cursor() {
    let cursor = cursor();
    cursor.show();
}

/// Show cursor display, only works on certain terminals.| demonstration
pub fn blink_cursor() {
    let cursor = cursor();
    cursor.blink(false);
    cursor.blink(false);
}

use self::crossterm_cursor::{
    execute, schedule, BlinkOff, BlinkOn, Command, Down, Goto, Hide, Left, Output, ResetPos, Right,
    SavePos, Show, Up,
};
use std::io::{stdout, Write};
use std::thread;
use std::time::{Duration, Instant};

//use crossterm_cursor::{Result, ErrorKind};

fn goto_pos_speed_test() {
    let mut stdout = ::std::io::stdout();

    let instant1 = Instant::now();
    for i in 0..10 {
        for x in 0..200 {
            for y in 0..50 {
                schedule!(stdout, Goto(x, y), Hide, Output(y.to_string()));
            }
        }
    }
    execute!(Goto(0, 52));
    println!("Lazy flush: {:?}", instant1.elapsed());

    let instant2 = Instant::now();
    for i in 0..10 {
        for x in 0..200 {
            for y in 0..50 {
                execute!(stdout, Goto(x, y), Hide, Output(y.to_string()));
            }
        }
    }

    execute!(Goto(0, 53));
    println!("Hard flush: {:?}", instant2.elapsed());
}

fn main() {
    let mut stdout = ::std::io::stdout();

    execute!(Goto(5, 5));
    print!("1");
    execute!(Right(1));
    print!("2");
    execute!(Down(2));
    print!("3");
    execute!(Left(2));
    print!("4");
    execute!(Up(2));
    print!("5");
}
