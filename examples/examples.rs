
//! This bin folder can be used to try the examples out located in the examples directory.
//!
//! All you need to do is:
//!
//! - Download the crossterm source code.
//! - Run program with: `cargo run --example examples`

extern crate crossterm;

// modules that could be test
mod terminal;
mod color;
mod cursor;
mod some_types;
mod input;

use crossterm::{Screen, Crossterm};
use std::{time, thread};
use std::sync::mpsc;
use crossterm::cursor::cursor;

fn main() {
    let nthreads = 5;
    let (tx, rx) = mpsc::channel();


    for i in 0..nthreads {
        let tx = tx.clone();
        thread::spawn(move || {
            let response = Crossterm::new(&Screen::default());
            tx.send(response).unwrap();
        });
    }

    for _ in 0..nthreads {
        let screen: Crossterm = rx.recv().unwrap();
        screen.terminal();

    }

}
