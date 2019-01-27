use super::variables::{Direction, Position, Size};
use crossterm::{Color, Crossterm, Screen};

use std::collections::HashMap;

pub struct Part {
    pub position: Position,
}

impl Part {
    pub fn new(x: usize, y: usize) -> Part {
        Part {
            position: Position::new(x, y),
        }
    }
}

pub struct Snake {
    pub snake_parts: Vec<Part>,
    pub parent_pos: Position,
    map_size: Size,
}

impl Snake {
    pub fn new(map_size: Size) -> Snake {
        return Snake {
            map_size,
            snake_parts: vec![Part::new(9, 10), Part::new(8, 10)],
            parent_pos: Position::new(0, 0),
        };
    }

    pub fn move_snake(
        &mut self,
        direction: &Direction,
        screen: &Screen,
        free_positions: &mut HashMap<String, Position>,
    ) {
        let crossterm = Crossterm::from_screen(screen);
        let cursor = crossterm.cursor();
        let terminal = crossterm.terminal();

        let count = self.snake_parts.len();
        let is_food_eaten = false;

        for (index, ref mut snake_part) in self.snake_parts.iter_mut().enumerate() {
            if index == count - 1 {
                snake_part.position.remove(screen);
                free_positions.insert(
                    format!("{},{}", snake_part.position.x, snake_part.position.y),
                    snake_part.position,
                );
            }

            if index == 0 {
                self.parent_pos = snake_part.position.clone();

                match direction {
                    &Direction::Up => snake_part.position.y -= 1,
                    &Direction::Down => snake_part.position.y += 1,
                    &Direction::Left => snake_part.position.x -= 1,
                    &Direction::Right => snake_part.position.x += 1,
                }

                free_positions.remove_entry(
                    format!("{},{}", snake_part.position.x, snake_part.position.y).as_str(),
                );
            } else {
                let new_pos = self.parent_pos.clone();
                self.parent_pos = snake_part.position.clone();
                snake_part.position = new_pos;
            }
        }
    }

    pub fn draw_snake(&mut self, screen: &Screen) {
        for (index, ref mut snake_part) in self.snake_parts.iter_mut().enumerate() {
            snake_part.position.draw("■", screen);
        }
    }

    pub fn has_eaten_food(&mut self, food_pos: Position) -> bool {
        if self.snake_parts[0].position.x == food_pos.x
            && self.snake_parts[0].position.y == food_pos.y
        {
            self.snake_parts.push(Part::new(1, 1));
            return true;
        }

        return false;
    }

    pub fn get_parts(&self) -> &Vec<Part> {
        return &self.snake_parts;
    }
}

pub enum SnakeState {
    MovedOutMap,
    Moved,
    AteFood,
}
