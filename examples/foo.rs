use crossterm::queue;
use std::io::stdout;

fn main() {
    queue!(stdout(), "a");
}
