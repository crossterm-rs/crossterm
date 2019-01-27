extern crate crossterm;
extern crate rand;

mod map;
mod messages;
mod snake;
mod variables;

use self::crossterm::{input, terminal, ClearType, Color, Crossterm, Screen};

use map::Map;
use snake::Snake;
use variables::{Direction, Position, Size};

use std::collections::HashMap;
use std::io::Read;
use std::io::Write;
use std::iter::Iterator;
use std::{thread, time};

fn main() {
    let map_size = title_screen();

    {
        let mut screen = Screen::new(true);
        let crossterm = Crossterm::from_screen(&screen);
        let cursor = crossterm.cursor();
        let mut input = crossterm.input();

        cursor.hide();

        let mut stdin = input.read_async().bytes();

        let mut free_positions: HashMap<String, Position> =
            HashMap::with_capacity((map_size.width * map_size.height) as usize);

        let mut map = Map::new(map_size.clone());
        map.render_map(&screen, &mut free_positions);

        let mut direction = Direction::Right;

        let mut snake = Snake::new(map_size.clone());

        for part in snake.get_parts().iter() {
            free_positions
                .remove_entry(format!("{},{}", part.position.x, part.position.y).as_str());
        }

        map.spawn_food(&free_positions, &screen);

        loop {
            thread::sleep(time::Duration::from_millis(200));
            let pressed_key = stdin.next();

            if let Some(Ok(key)) = pressed_key {
                match key as char {
                    'w' => direction = Direction::Up,
                    'a' => direction = Direction::Left,
                    's' => direction = Direction::Down,
                    'd' => direction = Direction::Right,
                    _ => {}
                }
            }

            snake.move_snake(&direction, &screen, &mut free_positions);

            if map.is_out_of_bounds(snake.snake_parts[0].position) {
                break;
            }

            snake.draw_snake(&screen);

            if snake.has_eaten_food(map.foot_pos) {
                map.spawn_food(&free_positions, &screen);
            }
        }
        drop(screen);
    }

    game_over_screen();
}

fn title_screen() -> Size {
    let crossterm = Crossterm::new();

    let cursor = crossterm.cursor();
    let terminal = crossterm.terminal().clear(ClearType::All);

    println!("{}", messages::SNAKERS.join("\n\r"));
    cursor.goto(0, 15);
    println!("Enter map width:");
    cursor.goto(17, 15);
    let width = crossterm.input().read_line().unwrap();
    println!("\r\nEnter map height:");
    cursor.goto(17, 17);
    let height = crossterm.input().read_line().unwrap();

    let parsed_width = width.parse::<usize>().unwrap();
    let parsed_height = height.parse::<usize>().unwrap();

    let terminal = crossterm.terminal().clear(ClearType::All);
    return Size::new(parsed_width, parsed_height);
}

fn print_game_stats(map_size: Size, snake_lenght: usize, food_aten: usize, screen: &mut Screen) {
    let crossterm = Crossterm::new();

    let cursor = crossterm.cursor();
    let terminal = crossterm.terminal().clear(ClearType::All);

    screen.write(format!("Snake Lenght: {}\n\r", snake_lenght).as_ref());
    screen.write(format!("Food aten: {}\n\r", snake_lenght).as_ref());

    cursor.goto(0, map_size.height as u16);
    cursor.goto(0, map_size.height as u16);
}

fn game_over_screen() {
    let crossterm = Crossterm::new();

    let cursor = crossterm.cursor();
    let terminal = crossterm.terminal();

    terminal.clear(ClearType::All);

    println!(
        "{}",
        crossterm
            .style(format!("{}", messages::END_MESSAGE.join("\n\r")))
            .with(Color::Red)
    );
    //    cursor.goto()
    cursor.show();
}
