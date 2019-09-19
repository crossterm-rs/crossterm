//!
//! Terminal Examples
//!

#![allow(dead_code)]

use crossterm::{cursor, terminal, ClearType, Result};

fn print_test_data() {
    for i in 0..100 {
        println!("Test data to test terminal: {}", i);
    }
}

/// Clear all lines in terminal | demonstration
fn clear_all_lines() -> Result<()> {
    let terminal = terminal();

    print_test_data();

    // Clear all lines in terminal;
    terminal.clear(ClearType::All)
}

/// Clear all lines from cursor position X:4, Y:4 down | demonstration
fn clear_from_cursor_down() -> Result<()> {
    let terminal = terminal();

    print_test_data();

    // Set terminal cursor position (see example for more info).
    cursor().goto(4, 8)?;

    // Clear all cells from current cursor position down.
    terminal.clear(ClearType::FromCursorDown)
}

/// Clear all lines from cursor position X:4, Y:4 up | demonstration
fn clear_from_cursor_up() -> Result<()> {
    let terminal = terminal();

    print_test_data();

    // Set terminal cursor position (see example for more info).
    cursor().goto(4, 4)?;

    // Clear all cells from current cursor position down.
    terminal.clear(ClearType::FromCursorUp)
}

/// Clear all lines from cursor position X:4, Y:4 up | demonstration
fn clear_current_line() -> Result<()> {
    let terminal = terminal();

    print_test_data();

    // Set terminal cursor position (see example for more info).
    cursor().goto(4, 3)?;

    // Clear current line cells.
    terminal.clear(ClearType::CurrentLine)
}

/// Clear all lines from cursor position X:4, Y:7 up | demonstration
fn clear_until_new_line() -> Result<()> {
    let terminal = terminal();

    print_test_data();

    // Set terminal cursor position (see example for more info).
    cursor().goto(4, 20)?;

    // Clear all the cells until next line.
    terminal.clear(ClearType::UntilNewLine)
}

/// Print the the current terminal size | demonstration.
fn print_terminal_size() -> Result<()> {
    let terminal = terminal();

    // Get terminal size
    let (width, height) = terminal.size()?;

    // Print results
    print!("X: {}, y: {}", width, height);
    Ok(())
}

/// Set the terminal size to width 10, height: 10 | demonstration.
fn set_terminal_size() -> Result<()> {
    let terminal = terminal();

    terminal.set_size(10, 10)
}

/// Scroll down 10 lines | demonstration.
fn scroll_down() -> Result<()> {
    let terminal = terminal();

    print_test_data();

    // Scroll down 10 lines.
    terminal.scroll_down(10)
}

/// Scroll down 10 lines | demonstration.
fn scroll_up() -> Result<()> {
    let terminal = terminal();

    print_test_data();

    // Scroll up 10 lines.
    terminal.scroll_up(10)
}

/// exit the current proccess.
fn exit() {
    let terminal = terminal();
    terminal.exit();
}

// cargo run --example terminal
fn main() -> Result<()> {
    scroll_down()
}
