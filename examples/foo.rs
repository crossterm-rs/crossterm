use crossterm::style::Attribute;
use crossterm::terminal;

fn main() {
    println!("{:?}", terminal::size());
}
