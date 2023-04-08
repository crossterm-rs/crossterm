//! Demonstrates how to match on modifiers like: Control, alt, shift.
//!
//! cargo run --example event-poll-read

use std::{io, time::Duration};

use crossterm::{
    cursor::position,
    event::{poll, read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
};
pub fn main() {
    crossterm::terminal::enable_raw_mode().unwrap();
    let result = crossterm::event::read();
    eprintln!("RESULT: {:?}", result);
}
