//!    
//! Cursor Examples
//!

extern crate crossterm;

use self::crossterm::cursor::{cursor, TerminalCursor};

/// Set the cursor to position X: 10, Y: 5 in the terminal.
pub fn goto()
{
    // Get the cursor
    let mut cursor = cursor();
    // Set the cursor to position X: 10, Y: 5 in the terminal
    cursor.goto(10,5);    
}

/// Move the cursor 3 up | demonstration.
pub fn move_up()
{
    // Get the cursor
    let mut cursor = cursor();
    // Move the cursor to position 3 times to the up in the terminal
    cursor.move_up(3);
}

/// Move the cursor 3 to the right | demonstration.
pub fn move_right()
{
    // Get the cursor
    let mut cursor = cursor();
    // Move the cursor to position 3 times to the right in the terminal
    cursor.move_right(3);
}

/// Move the cursor 3 down | demonstration.
pub fn move_down()
{
    // Get the cursor
    let mut cursor = cursor();
    // Move the cursor to position 3 times to the down in the terminal
    cursor.move_down(3);
}

/// Move the cursor 3 to the left | demonstration.
pub fn move_left()
{
    // Get the cursor
    let mut cursor = cursor();
    // Move the cursor to position 3 times to the left in the terminal
    cursor.move_left(3);
}

/// Print character at X: 10 Y: 5 | demonstration.
pub fn print()
{
    // To print an some displayable content on an certain position.  
    
    // Get the cursor
    let mut cursor = cursor();
    // Set the cursor to position X: 10, Y: 5 in the terminal
    cursor.goto(10,5);
    // Print the @ symbol at position X: 10, Y: 5 in the terminal
    print!("@");
    // Rust is line buffered inorder to print at an certain position we need to clear the buffer first. 
    use std;
    use std::io::Write;
    std::io::stdout().flush();
    
    /* Because the above method is a little to much code,
       you can use the `print()` method for printing an value at an certain position in the terminal.
       
       Crossterm provides method chaining so that the above points can be inlined.
    */

    cursor.goto(10,5).print("@");
}

/// Save and reset cursor position | demonstration..
pub fn safe_and_reset_position()
{
    let mut cursor = cursor();
    
    // Goto X: 5 Y: 5
    cursor.goto(5,5);
    // Safe cursor position: X: 5 Y: 5
    cursor.save_position();
    // Goto X: 5 Y: 20
    cursor.goto(5,20);
    // Print at X: 5 Y: 20.
    println!("Yea!");
    // Reset back to X: 5 Y: 5.
    cursor.reset_position();
    // Print Back at X: 5 Y: 5.
    println!("Back");

    println!()
}





















