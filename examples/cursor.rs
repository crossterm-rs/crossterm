//!
//! Examples of actions that could be performed with te cursor.
//!

extern crate crossterm_cursor;

use crossterm_cursor::cursor;
use std::io;

/// Set the cursor to position X: 10, Y: 5 in the terminal.
pub fn goto() -> io::Result<()> {
    // Get the cursor
    let cursor = cursor();
    // Set the cursor to position X: 10, Y: 5 in the terminal
    cursor.goto(10, 5)?;

    Ok(())
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
pub fn save_and_reset_position() -> io::Result<()> {
    let cursor = cursor();

    // Goto X: 5 Y: 5
    cursor.goto(5, 5)?;
    // Safe cursor position: X: 5 Y: 5
    cursor.save_position()?;
    // Goto X: 5 Y: 20
    cursor.goto(5, 20)?;
    // Print at X: 5 Y: 20.
    println!("Yea!");
    // Reset back to X: 5 Y: 5.
    cursor.reset_position()?;
    // Print Back at X: 5 Y: 5.
    println!("Back");

    println!();

    Ok(())
}

/// Hide cursor display | demonstration.
pub fn hide_cursor() {
    let cursor = cursor();
    cursor.hide().unwrap();
}

/// Show cursor display | demonstration.
pub fn show_cursor() {
    let cursor = cursor();
    cursor.show().unwrap();
}

/// Show cursor display, only works on certain terminals.| demonstration
pub fn blink_cursor() {
    let cursor = cursor();
    cursor.blink(false).unwrap();
    cursor.blink(false).unwrap();
}

fn main() {
    pos()
}
