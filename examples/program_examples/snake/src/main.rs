extern crate crossterm;
extern crate rand;

mod map;
mod messages;
mod snake;
mod variables;

use self::crossterm::{
    AsyncReader, ClearType, Color, Colorize, Crossterm, InputEvent, KeyEvent, Screen,
};

use map::Map;
use snake::Snake;
use variables::{Direction, Position, Size};

use std::collections::HashMap;
use std::io::Write;
use std::iter::Iterator;
use std::{thread, time};

fn main() {
    let map_size = ask_size();

    // screen has to be in raw mode in order for the key presses not to be printed to the screen.
    let screen = Screen::new(true);
    let crossterm = Crossterm::from_screen(&screen);

    crossterm.cursor().hide();

    // initialize free positions for the game map.
    let mut free_positions: HashMap<String, Position> =
        HashMap::with_capacity((map_size.width * map_size.height) as usize);

    // render the map
    let mut map = Map::new(map_size);
    map.render_map(&screen, &mut free_positions);

    let mut snake = Snake::new();

    // remove snake coords from free positions.
    for part in snake.get_parts().iter() {
        free_positions.remove_entry(format!("{},{}", part.position.x, part.position.y).as_str());
    }

    map.spawn_food(&free_positions, &screen);

    let mut stdin = crossterm.input().read_async();
    let mut snake_direction = Direction::Right;

    // start the game loop; draw, move snake and spawn food.
    loop {
        if let Some(new_direction) = update_direction(&mut stdin) {
            snake_direction = new_direction;
        }

        snake.move_snake(&snake_direction, &screen, &mut free_positions);

        if map.is_out_of_bounds(snake.snake_parts[0].position) {
            break;
        }

        snake.draw_snake(&screen);

        if snake.has_eaten_food(map.foot_pos) {
            map.spawn_food(&free_positions, &screen);
        }

        thread::sleep(time::Duration::from_millis(400));
    }
    game_over_screen();
}

fn update_direction(reader: &mut AsyncReader) -> Option<Direction> {
    let pressed_key = reader.next();

    if let Some(InputEvent::Keyboard(KeyEvent::Char(character))) = pressed_key {
        return Some(match character {
            'w' => Direction::Up,
            'a' => Direction::Left,
            's' => Direction::Down,
            'd' => Direction::Right,
            _ => return None,
        });
    } else if let Some(InputEvent::Keyboard(key)) = pressed_key {
        return Some(match key {
            KeyEvent::Up => Direction::Up,
            KeyEvent::Left => Direction::Left,
            KeyEvent::Down => Direction::Down,
            KeyEvent::Right => Direction::Right,
            _ => return None,
        });
    }

    None
}

fn ask_size() -> Size {
    let crossterm = Crossterm::new();
    crossterm.terminal().clear(ClearType::All);

    let cursor = crossterm.cursor();

    println!(
        "{}",
        crossterm
            .style(format!("{}", messages::END_MESSAGE.join("\n\r")))
            .with(Color::Cyan)
    );

    // read width
    cursor.goto(0, 15);
    println!("{}", "Enter map width:".green().on_yellow());
    cursor.goto(17, 15);

    // read height
    let width = crossterm.input().read_line().unwrap();
    println!("{}", "\r\nEnter map height:".green().on_yellow());
    cursor.goto(17, 17);

    let height = crossterm.input().read_line().unwrap();

    // parse input
    let parsed_width = width.parse::<usize>().unwrap();
    let parsed_height = height.parse::<usize>().unwrap();

    crossterm.terminal().clear(ClearType::All);

    return Size::new(parsed_width, parsed_height);
}

fn game_over_screen() {
    let crossterm = Crossterm::new();

    crossterm.terminal().clear(ClearType::All);

    println!(
        "{}",
        crossterm
            .style(format!("{}", messages::END_MESSAGE.join("\n\r")))
            .with(Color::Red)
    );

    crossterm.cursor().show();
}
