use std::fmt;

use crossterm::Result;

use super::types::{Direction, Position};

/// A snake fragment kind.
///
/// Describes how a snake fragment is visualized.
#[derive(Debug, Clone, Copy)]
enum FragmentKind {
    Horizontal,
    Vertical,
    Left,
    Right,
    Up,
    Down,
    UpToRight,
    UpToLeft,
    DownToRight,
    DownToLeft,
    LeftToUp,
    LeftToDown,
    RightToUp,
    RightToDown,
}

impl FragmentKind {
    /// Creates a snake fragment kind from the snake head direction.
    fn with_head_direction(direction: Direction) -> Self {
        match direction {
            Direction::Up => FragmentKind::Up,
            Direction::Down => FragmentKind::Down,
            Direction::Left => FragmentKind::Left,
            Direction::Right => FragmentKind::Right,
        }
    }
}

impl fmt::Display for FragmentKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr = match self {
            FragmentKind::Horizontal => "━",
            FragmentKind::Vertical => "┃",
            FragmentKind::Up => "╻",
            FragmentKind::Down => "╹",
            FragmentKind::Left => "╺",
            FragmentKind::Right => "╸",
            FragmentKind::UpToRight | FragmentKind::LeftToDown => "┏",
            FragmentKind::UpToLeft | FragmentKind::RightToDown => "┓",
            FragmentKind::DownToRight | FragmentKind::LeftToUp => "┗",
            FragmentKind::DownToLeft | FragmentKind::RightToUp => "┛",
        };
        write!(f, "{}", repr)
    }
}

/// A snake fragment.
#[derive(Debug)]
struct Fragment {
    /// The fragment position in the terminal.
    position: Position,
    /// The fragment kind.
    kind: FragmentKind,
}

impl Fragment {
    /// Creates a `Fragment` from the given `position` and `kind`.
    fn new<P: Into<Position>>(position: P, kind: FragmentKind) -> Fragment {
        Fragment {
            position: position.into(),
            kind,
        }
    }

    /// Draws the fragment.
    fn draw(&self) -> Result<()> {
        self.position.draw(self.kind)
    }

    /// Clears the fragment from the screen.
    fn clear(&self) -> Result<()> {
        self.position.clear_char()
    }

    /// Creates a new head `Fragment` from the given snake `direction`.
    ///
    /// Assumes that the `self` is the current snake head.
    fn with_head_direction(&self, direction: Direction) -> Fragment {
        let position = match direction {
            Direction::Up => (self.position.x, self.position.y - 1),
            Direction::Down => (self.position.x, self.position.y + 1),
            Direction::Left => (self.position.x - 1, self.position.y),
            Direction::Right => (self.position.x + 1, self.position.y),
        };

        Fragment {
            position: position.into(),
            kind: FragmentKind::with_head_direction(direction),
        }
    }

    /// Updates the fragment kind with the given snake `direction` and redraws
    /// it.
    ///
    /// Assumes that the `self` is the current snake head (soon to be the old
    /// one = 2nd fragment).
    ///
    /// # Examples
    ///
    /// Let's say we have a snake `━╸` that is moving to the `Right` and the new
    /// direction is `Up`. We can't create / draw a new head only, because then
    /// the snake will look like this one:
    ///
    /// ```
    ///  ╻
    /// ━╸
    /// ```
    ///
    /// We have to change the current head kind `╻` to `┛` to get a snake like this
    /// one:
    ///
    /// ```
    ///  ╻
    /// ━┛
    /// ```
    fn update_with_head_direction(&mut self, direction: Direction) -> Result<()> {
        let new_kind = match (self.kind, direction) {
            (FragmentKind::Left, Direction::Left) => FragmentKind::Horizontal,
            (FragmentKind::Right, Direction::Right) => FragmentKind::Horizontal,
            (FragmentKind::Up, Direction::Up) => FragmentKind::Vertical,
            (FragmentKind::Down, Direction::Down) => FragmentKind::Vertical,
            (FragmentKind::Left, Direction::Up) => FragmentKind::LeftToUp,
            (FragmentKind::Left, Direction::Down) => FragmentKind::LeftToDown,
            (FragmentKind::Right, Direction::Up) => FragmentKind::RightToUp,
            (FragmentKind::Right, Direction::Down) => FragmentKind::RightToDown,
            (FragmentKind::Up, Direction::Left) => FragmentKind::UpToLeft,
            (FragmentKind::Up, Direction::Right) => FragmentKind::UpToRight,
            (FragmentKind::Down, Direction::Left) => FragmentKind::DownToLeft,
            (FragmentKind::Down, Direction::Right) => FragmentKind::DownToRight,
            (kind, _) => kind,
        };
        self.kind = new_kind;
        self.draw()
    }
}

/// A snake.
#[derive(Debug)]
pub struct Snake {
    /// The snake fragments. Head index is always 0.
    fragments: Vec<Fragment>,
    /// The current direction.
    direction: Direction,
    /// Says if the snake ate some food or not.
    ate_food: bool,
}

impl Snake {
    /// Creates a new snake.
    pub fn new(map_width: u16, map_height: u16) -> Snake {
        let center_x = map_width / 2;
        let center_y = map_height / 2;

        let parts = vec![
            Fragment::new((center_x, center_y), FragmentKind::Right),
            Fragment::new((center_x - 1, center_y), FragmentKind::Left),
        ];

        Snake {
            fragments: parts,
            direction: Direction::Right,
            ate_food: false,
        }
    }

    /// The current snake length.
    pub fn len(&self) -> usize {
        self.fragments.len()
    }

    /// The current snake direction.
    pub fn direction(&self) -> Direction {
        self.direction
    }

    /// Updates the snake direction.
    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    /// Sets if the snake ate food.
    ///
    /// If set to `true`, the next `update()` call will move the head, but
    /// won't move the tail.
    pub fn set_ate_food(&mut self, ate_food: bool) {
        self.ate_food = ate_food;
    }

    /// The snake head position.
    pub fn head_position(&self) -> Position {
        self.fragments[0].position
    }

    /// Returns `true` if there's an existing snake fragment at the
    /// given `position`.
    pub fn fragment_exists_at_position(&self, position: Position) -> bool {
        for fragment in &self.fragments {
            if fragment.position == position {
                return true;
            }
        }
        false
    }

    /// Moves the snake and redraws updated fragments only.
    ///
    /// Returns `Ok(true)` if the snake was updated. Returns `Ok(false)` if the
    /// new head position collides with the existing snake fragments.
    pub fn update(&mut self) -> Result<bool> {
        // Get the current head, update fragment kind and redraw it.
        // Let's say that the snake is moving down (╻). We want to draw a full
        // vertical line (┃) at the current head position.
        let current_head = self.fragments.first_mut().unwrap();
        current_head.update_with_head_direction(self.direction)?;

        // Create & draw the new head.
        let new_head = current_head.with_head_direction(self.direction);
        new_head.draw()?;

        // Check if the new head collides with existing snake fragments
        if self.fragment_exists_at_position(new_head.position) {
            // Collision, new snake head collides with existing fragment
            return Ok(false);
        }

        self.fragments.insert(0, new_head);

        if self.ate_food {
            // Snake ate some food, we just set the state to false and
            // do nothing (no tail movement)
            self.ate_food = false;
        } else {
            // Snake didn't eat any food, tail is moving, which means that
            // we are going to clear the fragment from the screen and drop it
            let tail = self.fragments.pop().unwrap();
            tail.clear()?;
        }

        Ok(true)
    }

    /// Redraws the snake.
    pub fn draw(&self) -> Result<()> {
        for fragment in &self.fragments {
            fragment.draw()?;
        }
        Ok(())
    }
}
