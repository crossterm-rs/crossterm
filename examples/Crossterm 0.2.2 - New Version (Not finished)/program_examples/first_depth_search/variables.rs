extern crate crossterm;

use self::crossterm::terminal::{terminal, ClearType};
use self::crossterm::Context;
use self::crossterm::style::{Color, StyledObject, ObjectStyle };

#[derive(Copy, Clone)]
pub enum Direction
{
    Up,
    Down,
    Left,
    Right
}

#[derive(Copy, Clone)]
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
}

#[derive(Copy, Clone)]
pub struct Size
{
    pub width: u16,
    pub height: u16
}

impl Size
{
    pub fn new(width: u16, height: u16) -> Size
    {
        Size {width,height}
    }
}

use std::fmt::Display;

pub struct Cell<'a, D: Display>
{
    position: Position,
    style: &'a StyledObject<D>,
    visited: bool
}

use std::rc::Rc;

impl<'a, D: Display> Cell<'a, D>
{
    pub fn new(position: Position, style: &'a StyledObject<D>) -> Cell<D>
    {
        Cell { position: position, style: style, visited: false }
    }
}