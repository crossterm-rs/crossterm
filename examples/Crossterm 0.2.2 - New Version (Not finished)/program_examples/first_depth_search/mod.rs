mod map;
mod variables;

use crossterm;
use crossterm::style::Color;

use self::variables::Size;

fn run()
{
    let mut env = crossterm::Environment::new();
    let map_size = Size::new(20,20);
    let wall_style = env.paint("■").with(Color::Blue).on(Color::Black);
    let map_style = env.paint(" ").with(Color::White).on(Color::White);

    map::Map::init(&mut env, map_size, wall_style, map_style);
}