//!
//! Examples of actions that could be performed with the cursor.
//!

#![allow(dead_code)]

use crossterm::{cursor, Result};

/// Set the cursor to position X: 10, Y: 5 in the terminal.
fn goto() -> Result<()> {
    // Get the cursor
    let cursor = cursor();
    // Set the cursor to position X: 10, Y: 5 in the terminal
    cursor.goto(10, 5)?;

    Ok(())
}

/// get the cursor position
fn pos() -> Result<()> {
    // Get the cursor
    let cursor = cursor();
    // get the cursor position.
    let (x, y) = cursor.pos()?;

    println!("{} {}", x, y);
    Ok(())
}

/// Move the cursor 3 up | demonstration.
fn move_up() -> Result<()> {
    // Get the cursor
    let mut cursor = cursor();

    // Move the cursor to position 3 times to the up in the terminal
    cursor.move_up(3)?;
    Ok(())
}

/// Move the cursor 3 down | demonstration.
fn move_down() -> Result<()> {
    let mut cursor = cursor();
    // Move the cursor to position 3 times to the down in the terminal
    cursor.move_down(3)?;
    Ok(())
}

/// Move the cursor 3 to the right | demonstration.
fn move_right() -> Result<()> {
    let mut cursor = cursor();
    // Move the cursor to position 3 times to the right in the terminal
    cursor.move_right(3)?;
    Ok(())
}

/// Move the cursor 3 left | demonstration.
fn move_left() -> Result<()> {
    let mut cursor = cursor();
    // Move the cursor to position 3 times to the left in the terminal
    cursor.move_left(3)?;
    Ok(())
}

/// Save and reset cursor position | demonstration..
fn save_and_restore_position() -> Result<()> {
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
    cursor.restore_position()?;
    // Print Back at X: 5 Y: 5.
    println!("Back");

    println!();

    Ok(())
}

/// Hide cursor display | demonstration.
fn hide_cursor() -> Result<()> {
    let cursor = cursor();
    cursor.hide()
}

/// Show cursor display | demonstration.
fn show_cursor() -> Result<()> {
    let cursor = cursor();
    cursor.show()
}

/// Show cursor display, only works on certain terminals.| demonstration
fn blink_cursor(enable: bool) -> Result<()> {
    let cursor = cursor();
    cursor.blink(enable)
}

// cargo run --example cursor
fn main() -> Result<()> {
    println!("Going to show cursor...");
    show_cursor()?;
    println!("Going to enable blinking cursor & sleep for 5s...");
    blink_cursor(true)?;
    std::thread::sleep(std::time::Duration::from_secs(5));
    println!("Going to disable blinking cursor...");
    blink_cursor(false)
}
