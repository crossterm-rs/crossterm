//! 
//! Terminal Examples
//!

extern crate crossterm;

use crossterm::crossterm_terminal::{get, Terminal, ClearType};
use crossterm::crossterm_cursor;

fn print_test_data()
{
    for i in 0..100 {
        println!("abcdefghijTest data to test terminal: {}",i);
    }
}

/// Clear all lines in terminal | demonstration
pub fn clear_all_lines()
{
    // Get terminal
    let mut terminal = get();
    
    print_test_data();
    
    // Clear all lines in terminal;
    terminal.clear(ClearType::All);
}

/// Clear all lines from cursor position X:4, Y:4 down | demonstration
pub fn clear_from_cursor_down()
{
    // Get terminal
    let mut terminal = get();

    print_test_data();

    // Set terminal cursor position (see example for more info).
    crossterm_cursor::get().goto(4,8);

    // Clear all cells from current cursor position down.
    terminal.clear(ClearType::FromCursorDown);
}

/// Clear all lines from cursor position X:4, Y:4 up | demonstration
pub fn clear_from_cursor_up()
{
    // Get terminal
    let mut terminal = get();

    print_test_data();

    // Set terminal cursor position (see example for more info).
    crossterm_cursor::get().goto(4,8);

    // Clear all cells from current cursor position down.
    terminal.clear(ClearType::FromCursorUp);
}

/// Clear all lines from cursor position X:4, Y:4 up | demonstration
pub fn clear_current_line()
{
    // Get terminal
    let mut terminal = get();

    print_test_data();

    // Set terminal cursor position (see example for more info).
    crossterm_cursor::get().goto(4,4);

    // Clear current line cells.
    terminal.clear(ClearType::CurrentLine);
}

/// Clear all lines from cursor position X:4, Y:4 up | demonstration
pub fn clear_until_new_line()
{
    // Get terminal
    let mut terminal = get();

    print_test_data();

    // Set terminal cursor position (see example for more info).
    crossterm_cursor::get().goto(4,4);

    // Clear all the cells until next line.
    terminal.clear(ClearType::UntilNewLine);
}

pub fn print_terminal_size()
{
    // Get terminal 
    let mut terminal = get();
    // Get terminal size
    let terminal_size = terminal.terminal_size().unwrap();
    // Print results
    print!("X: {}, y: {}", terminal_size.0, terminal_size.1);
}

// scroll down 10 lines
pub fn scroll_down()
{
    print_test_data();
    // Get terminal 
    let mut terminal = get();
    // Scroll down 10 lines.
    let terminal_size = terminal.scroll_down(10);
}

// scroll down 10 lines
pub fn scroll_up()
{
    print_test_data();
    
    // Get terminal 
    let mut terminal = get();
    // Scroll up 10 lines.
    let terminal_size = terminal.scroll_up(10);
}
