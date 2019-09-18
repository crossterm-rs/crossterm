use std::convert::TryFrom;
use std::fmt::Display;
use std::io::{stdout, Write};

use crossterm::{style, Color, Crossterm, KeyEvent, Result, TerminalCursor};

/// Position in the terminal window.
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct Position {
    /// The position column index (0 based).
    pub x: u16,
    /// The position row index (0 based).
    pub y: u16,
}

impl Position {
    /// Creates a new position from the given `x` & `y`.
    pub fn new(x: u16, y: u16) -> Position {
        Position { x, y }
    }

    /// Draws the given `value` at this position.
    pub fn draw<D: Display + Clone>(&self, value: D) -> Result<()> {
        let cursor = TerminalCursor::new();
        cursor.goto(self.x, self.y)?;

        print!("{}", style(value).with(Color::Red));
        stdout().flush()?;
        Ok(())
    }

    /// Clears character (writes single space) at this position.
    pub fn clear_char(&self) -> Result<()> {
        let crossterm = Crossterm::new();
        crossterm.cursor().goto(self.x, self.y)?;
        crossterm.terminal().write(" ")?;
        Ok(())
    }
}

/// Crates a `Position` from a `(u16, u16)` tuple.
impl From<(u16, u16)> for Position {
    fn from(pos: (u16, u16)) -> Self {
        Position::new(pos.0, pos.1)
    }
}

/// A snake direction.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    /// Returns `true` if the direction is vertical (`Up` or `Down`).
    fn is_vertical(&self) -> bool {
        self == &Direction::Up || self == &Direction::Down
    }

    /// Returns `true` if the direction can be changed to the given
    /// `direction`.
    ///
    /// It's allowed to change direction from vertical to horizontal
    /// and vice versa. It's not allowed to change the `Right` direction
    /// to either `Left` or `Right`, but it's allowed to change it
    /// to either `Up` or `Down`.
    pub fn can_change_to(&self, direction: Direction) -> bool {
        self.is_vertical() != direction.is_vertical()
    }
}

/// Tries to create a `Direction` from the `KeyEvent` (arrow keys).
impl TryFrom<KeyEvent> for Direction {
    type Error = ();

    fn try_from(value: KeyEvent) -> std::result::Result<Self, Self::Error> {
        match value {
            KeyEvent::Up => Ok(Direction::Up),
            KeyEvent::Left => Ok(Direction::Left),
            KeyEvent::Down => Ok(Direction::Down),
            KeyEvent::Right => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

/// Tries to create a `Direction` from the `char` (WASD keys).
impl TryFrom<char> for Direction {
    type Error = ();

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        match value {
            'w' => Ok(Direction::Up),
            'a' => Ok(Direction::Left),
            's' => Ok(Direction::Down),
            'd' => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}
