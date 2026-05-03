//! Demonstrates OSC 8 hyperlink support.
//!
//! cargo run --example link

use std::io;

use crossterm::{
    execute,
    style::{EndHyperlink, Print, StartHyperlink},
};

fn main() -> io::Result<()> {
    let mut out = io::stdout();

    execute!(
        out,
        Print("Visit: "),
        StartHyperlink::new("https://github.com/crossterm-rs/crossterm"),
        Print("crossterm"),
        EndHyperlink,
        Print("\n"),
    )
}
