use std::io::{stdout, Write};

use crossterm::{queue, Color, Crossterm, Goto, PrintStyledFont, Result};

use super::variables::{Cell, Position, Size};

pub struct Map {
    pub map: Vec<Vec<Cell>>,
    pub size: Size,
}

impl Map {
    pub fn new(map_size: Size, wall_cell_char: char, map_cell_char: char) -> Map {
        let mut map: Vec<Vec<Cell>> = Vec::new();

        // initialize the map shown on the screen. Each cell of terminal should have a value that could be changed by the algorithm
        // create n rows with n cells.
        for y in 0..map_size.height {
            let mut row: Vec<Cell> = Vec::new();

            for x in 0..map_size.width {
                if (y == 0 || y == map_size.height - 1) || (x == 0 || x == map_size.width - 1) {
                    row.push(Cell::new(
                        Position::new(x, y),
                        Color::Black,
                        wall_cell_char,
                        true,
                    ));
                } else {
                    row.push(Cell::new(
                        Position::new(x, y),
                        Color::Black,
                        map_cell_char,
                        false,
                    ));
                }
            }
            map.push(row);
        }

        Map {
            map,
            size: Size::new(map_size.width, map_size.height),
        }
    }

    // render the map on the screen.
    pub fn render_map(&mut self) -> Result<()> {
        let crossterm = Crossterm::new();

        for row in self.map.iter_mut() {
            for column in row.iter_mut() {
                // we only have to render the walls
                if (column.position.y == 0 || column.position.y == self.size.height - 1)
                    || (column.position.x == 0 || column.position.x == self.size.width - 1)
                {
                    queue!(
                        stdout(),
                        Goto(column.position.x as u16, column.position.y as u16),
                        PrintStyledFont(crossterm.style(column.look).on(column.color))
                    )?;
                }
            }
        }

        Ok(())
    }

    // check if position in the map at the given coords is visted.
    pub fn is_cell_visited(&self, x: usize, y: usize) -> bool {
        self.map[y][x].visited
    }

    // change a position in the map to visited.
    pub fn set_visited(&mut self, x: usize, y: usize) {
        self.map[y][x].visited = true;
    }
}
