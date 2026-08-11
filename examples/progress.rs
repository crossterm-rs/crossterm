//! Demonstrates ConEmu progress bar (OSC 9;4) support.
//!
//! cargo run --example progress

use std::io;
use std::thread;
use std::time::Duration;

use crossterm::{execute, terminal::SetProgress};

fn main() -> io::Result<()> {
    let mut out = io::stdout();
    for i in 0..=10 {
        let progress = i * 10;
        execute!(out, SetProgress::Default(progress))?;
        thread::sleep(Duration::from_secs(1));
    }
    execute!(out, SetProgress::Clear)
}
