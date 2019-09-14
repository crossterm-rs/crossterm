//!
//! Examples of actions that could be performed with the cursor.
//!
#![allow(unused_must_use, dead_code)]

extern crate crossterm_cursor;

use std::io::Write;
use std::time::Instant;

use crossterm_cursor::cursor;

use self::crossterm_cursor::{queue, Goto, Hide, Output, QueueableCommand};

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

fn benchmark_cursor_goto() -> f32 {
    let mut stdout = ::std::io::stdout();

    let instant1 = Instant::now();
    for _ in 0..10 {
        for x in 0..200 {
            for y in 0..50 {
                queue!(stdout, Goto(x, y), Hide, Output(y.to_string()));
            }
        }
    }

    let new_api = instant1.elapsed();
    let cursor = cursor();
    let instant2 = Instant::now();
    for _ in 0..10 {
        for x in 0..200 {
            for y in 0..50 {
                cursor.goto(x, y);
                print!("{}", y.to_string());
            }
        }
    }
    let old_api = instant2.elapsed();

    let speed_improvement = ((old_api.as_millis() as f32 - new_api.as_millis() as f32)
        / old_api.as_millis() as f32)
        * 100.;

    speed_improvement
}

fn start_goto_benchmark() {
    let mut performance_metrics = Vec::new();
    for _ in 1..=20 {
        performance_metrics.push(benchmark_cursor_goto());
    }

    println!(
        "Average Performance Improvement mesearued 10 times {:.2} %",
        performance_metrics.iter().sum::<f32>() / 20.
    );
}

fn main() {
    let stdout = ::std::io::stdout();

    stdout
        .queue(Goto(5, 5))
        .queue(Output("#".to_string()))
        .flush();

    println!("out: {}", Output("1".to_string()));
}
