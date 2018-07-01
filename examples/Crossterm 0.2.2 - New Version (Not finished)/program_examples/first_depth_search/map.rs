use super::variables::{Cell, Position, Size };
use crossterm::terminal::terminal;
use crossterm::Environment;
use crossterm::style::{ObjectStyle, StyledObject};
use crossterm::Context;
use std::rc::Rc;

use std::fmt::Display;

pub struct Map<D: Display>
{
    map: Vec<Vec<Cell<D>>>,
    wall_style: StyledObject<D>,
    map_style: StyledObject<D>,
}

impl<D: Display> Map<D>
{
    pub fn new(context: Rc<Context>) -> Map<D>
    {
        Map { map: Vec::new(), wall_style: ObjectStyle::apply_  to("■", context.clone() )}
    }

    pub fn init(&self, environment: &mut Environment, map_size: Size) -> Map<D>
    {
        let mut map: Vec<Vec<Cell<D>>> = Vec::new();

        for y in 0..map[0].len()
        {
            for x in 0..map[1].len()
            {
                if (y == 0 || y == map.len() - 1) || (x == 0 || x == map[0].len())
                {
                    map[y][x] = Cell::new(Position::new(x,y), wall_style.apply_to(environment.context(), "■"));
                }
                else{
                    map[y][x] = Cell::new(Position::new(x,y), map_style);
                }
            }
        }

        Map { map }
    }

    fn render_map()
    {

    }
}