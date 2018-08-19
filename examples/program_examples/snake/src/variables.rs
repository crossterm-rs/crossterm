extern crate crossterm;

use self::crossterm::terminal::{terminal, ClearType};
use self::crossterm::style::{Color, StyledObject, ObjectStyle, style };
use self::crossterm::cursor::cursor;
use self::crossterm::Screen;

use std::fmt::Debug;
use std::fmt;

#[derive(Copy, Clone,Debug)]
pub enum Direction
{
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct Position
{
    pub x: usize,
    pub y: usize
}

impl Position
{
    pub fn new(x: usize, y: usize) -> Position
    {
        Position { x, y }
    }

    pub fn draw(&self, val: &str, screen: &Screen)
    {
        cursor(screen).goto(self.x as u16, self.y as u16);
        style(val).with(Color::Red).paint(&screen);
        screen.stdout.flush();
    }

    pub fn remove(&self, screen: &Screen)
    {
        cursor(screen).goto(self.x as u16, self.y as u16);
        terminal(&screen).write(" ");
    }
}

#[derive(Copy, Clone)]
pub struct Size
{
    pub width: usize,
    pub height: usize
}

impl Size
{
    pub fn new(width: usize, height: usize) -> Size
    {
        Size {width,height}
    }
}