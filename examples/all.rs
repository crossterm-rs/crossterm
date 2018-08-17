use crossterm::Crossterm;
use crossterm::cursor::TerminalCursor;

fn main() {
    let crossterm = Crossterm::new();
    let cursor = cursor();
    cursor.goto(10,10);
    //let cursor = TerminalCursor::new();
}
