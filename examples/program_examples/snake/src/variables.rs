extern crate crossterm;

use self::crossterm::{
    cursor, style, ClearType, Color, Crossterm, ObjectStyle, Screen, StyledObject, TerminalCursor,
};

use std::fmt;
use std::fmt::Debug;

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

    pub fn draw(&self, val: &str, screen: &Screen) {
        let cursor = TerminalCursor::from_output(&screen.stdout);
        cursor.goto(self.x as u16, self.y as u16);

        style(val).with(Color::Red).paint(&screen.stdout);
        screen.stdout.flush();
    }

    pub fn remove(&self, screen: &Screen) {
        let crossterm = Crossterm::from_screen(screen);

        crossterm.cursor().goto(self.x as u16, self.y as u16);
        crossterm.terminal().write("  ");
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
