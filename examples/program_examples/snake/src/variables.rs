use std::io::stdout;
use std::io::Write;

use crossterm::{style, Color, Crossterm, Result, TerminalCursor};

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Position {
        Position { x, y }
    }

    pub fn draw(&self, val: &str) -> Result<()> {
        let cursor = TerminalCursor::new();
        cursor.goto(self.x as u16, self.y as u16)?;

        print!("{}", style(val).with(Color::Red));
        stdout().flush()?;
        Ok(())
    }

    pub fn remove(&self) -> Result<()> {
        let crossterm = Crossterm::new();

        crossterm.cursor().goto(self.x as u16, self.y as u16)?;
        crossterm.terminal().write("  ")?;
        Ok(())
    }
}

#[derive(Copy, Clone)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}

impl Size {
    pub fn new(width: usize, height: usize) -> Size {
        Size { width, height }
    }
}
