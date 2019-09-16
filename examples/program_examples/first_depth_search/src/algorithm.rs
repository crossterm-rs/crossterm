//! Implementation of the first depth search algorithm

use std::io::Write;
use std::{thread, time};

use crossterm::{execute, Color, Colorize, Goto, Hide, PrintStyledFont, Result, SetBg, SetFg};
use rand;
use rand::distributions::{IndependentSample, Range};

use super::map::Map;
use super::variables::{Direction, Position};

pub struct FirstDepthSearch {
    direction: Direction,
    map: Map,
    stack: Vec<Position>,
    root_pos: Position,
    is_terminated: bool,
}

impl FirstDepthSearch {
    pub fn new(map: Map, start_pos: Position) -> FirstDepthSearch {
        FirstDepthSearch {
            direction: Direction::Up,
            map,
            stack: Vec::new(),
            root_pos: start_pos,
            is_terminated: false,
        }
    }

    pub fn start(&mut self) -> Result<()> {
        self.is_terminated = false;

        // push first position on the stack
        self.stack.push(self.root_pos);

        execute!(
            ::std::io::stdout(),
            Hide,
            SetFg(Color::Green),
            SetBg(Color::Black)
        )?;

        // loop until there are now items left in the stack.
        loop {
            if self.stack.len() == 0 {
                break;
            }

            self.choose_random_neighbor();

            if self.is_terminated {
                break;
            }

            self.update_position();

            let pos = self.root_pos.clone();

            let x = pos.x as u16;
            let y = pos.y as u16;

            execute!(
                ::std::io::stdout(),
                Goto(x, y),
                PrintStyledFont(" ".on_yellow())
            )?;

            thread::sleep(time::Duration::from_millis(1));
        }

        Ok(())
    }

    /// With this function we are choosing an random neighbor that we havent visited yet.
    fn choose_random_neighbor(&mut self) {
        let mut available_directions: Vec<Direction> = Vec::with_capacity(4);

        // check every direction if the direction is not visited we can add it to the list.
        // note that if the y or x is 0 that we don't want to subtract because we get an subtract overflow.
        if self.root_pos.y != 0
            && !self
                .map
                .is_cell_visited(self.root_pos.x, self.root_pos.y - 1)
        {
            available_directions.push(Direction::Up)
        }

        if !&self
            .map
            .is_cell_visited(self.root_pos.x, self.root_pos.y + 1)
        {
            available_directions.push(Direction::Down)
        }

        if self.root_pos.x != 0
            && !self
                .map
                .is_cell_visited(self.root_pos.x - 1, self.root_pos.y)
        {
            available_directions.push(Direction::Left)
        }

        if !&self
            .map
            .is_cell_visited(self.root_pos.x + 1, self.root_pos.y)
        {
            available_directions.push(Direction::Right)
        }

        let directions_count = available_directions.len();

        // if there are no directions left we need to backtrack until we find directions to go to.
        if directions_count != 0 {
            let step = Range::new(0, directions_count);
            let mut rng = rand::thread_rng();
            let choice = step.ind_sample(&mut rng);

            // set the current direction to the new random generated direction.
            self.direction = available_directions[choice];
        } else {
            self.find_first_possible_direction();
        }
    }

    /// Find direction to go to if there is no direction pop the current position of the stack for back tracking to the previous position.
    fn find_first_possible_direction(&mut self) {
        // if there are no elements left in the stack that means we have visited all cell and we van terminate the program.
        if let &Some(previous_cell) = &self.stack.pop() {
            // update root pos to previous cell and continue searching for new neighbours
            self.root_pos = previous_cell;
            self.choose_random_neighbor();
        } else {
            self.is_terminated = true;
        }
    }

    /// update the root position to the new direction we went in
    fn update_position(&mut self) {
        match self.direction {
            Direction::Up => self.root_pos.y -= 1,
            Direction::Down => self.root_pos.y += 1,
            Direction::Left => self.root_pos.x -= 1,
            Direction::Right => self.root_pos.x += 1,
        };

        self.map.set_visited(self.root_pos.x, self.root_pos.y);
        self.stack.push(self.root_pos);
    }
}
