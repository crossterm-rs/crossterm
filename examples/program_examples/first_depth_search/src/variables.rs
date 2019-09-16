use crossterm::Color;

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

#[derive(Copy, Clone, Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Position {
        Position { x, y }
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

pub struct Cell {
    pub position: Position,
    pub color: Color,
    pub look: char,
    pub visited: bool,
}

impl Cell {
    pub fn new(position: Position, color: Color, look: char, visited: bool) -> Cell {
        Cell {
            position,
            color,
            look,
            visited,
        }
    }
}
