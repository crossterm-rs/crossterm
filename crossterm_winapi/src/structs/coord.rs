//! This module provides a type that represents some location/coordination.
//! For example, in WinAPi we have `COORD` which looks and feels inconvenient.
//! This module provides also some trait implementations who will make parsing and working whit `COORD` easier.

use winapi::um::wincon::COORD;

/// This is type represents the position of something on a certain 'x' and 'y'.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Coord {
    /// the position on the x axis
    pub x: i16,
    /// the position on the y axis
    pub y: i16,
}

impl Coord {
    /// Create a new size instance by passing in the width and height.
    pub fn new(x: i16, y: i16) -> Coord {
        Coord { x, y }
    }
}

impl From<COORD> for Coord {
    fn from(coord: COORD) -> Self {
        Coord::new(coord.X, coord.Y)
    }
}

impl From<Coord> for COORD {
    fn from(location: Coord) -> Self {
        COORD {
            X: location.x,
            Y: location.y,
        }
    }
}

impl Into<(u16, u16)> for Coord {
    fn into(self) -> (u16, u16) {
        (self.x as u16, self.y as u16)
    }
}
