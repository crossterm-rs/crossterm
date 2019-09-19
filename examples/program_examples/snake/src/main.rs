//! The snake game.
//!
//! This is not a properly designed game! Mainly game loop, input events
//! handling, UI separation, ... The main purpose of this example is to
//! test the `crossterm` crate and demonstrate some of the capabilities.
use std::convert::TryFrom;
use std::io::{stdout, Write};
use std::iter::Iterator;
use std::{thread, time};

use crossterm::{
    execute, input, style, AsyncReader, Clear, ClearType, Color, Crossterm, Goto, InputEvent,
    KeyEvent, PrintStyledFont, RawScreen, Result, Show,
};

use map::Map;
use snake::Snake;
use types::Direction;

mod map;
mod messages;
mod snake;
mod types;

/// An input (user) event.
#[derive(Debug)]
pub enum Event {
    /// User wants to change the snake direction.
    UpdateSnakeDirection(Direction),
    /// User wants to quite the game.
    QuitGame,
}

fn main() -> Result<()> {
    // Print the welcome screen and ask for the map size.
    let crossterm = Crossterm::new();
    let (map_width, map_height) = ask_for_map_size(crossterm.terminal().size()?)?;

    // Switch screen to the raw mode to avoid printing key presses on the screen
    // and hide the cursor.
    let _raw = RawScreen::into_raw_mode();
    crossterm.cursor().hide()?;

    // Draw the map border.
    let mut map = Map::new(map_width, map_height);
    map.draw_border()?;

    // Create a new snake, draw it and spawn some food.
    let mut snake = Snake::new(map_width, map_height);
    snake.draw()?;
    map.spawn_food(&snake)?;

    // Game loop
    let mut stdin = crossterm.input().read_async();
    loop {
        // Handle the next user input event (if there's any).
        match next_event(&mut stdin, snake.direction()) {
            Some(Event::UpdateSnakeDirection(direction)) => snake.set_direction(direction),
            Some(Event::QuitGame) => break,
            _ => {}
        };

        // Update the snake (move & redraw). If it returns `false` -> new head
        // collides with the snake body -> can't eat self -> quit the game loop.
        if !snake.update()? {
            break;
        }

        // Check if the snake ate some food.
        if snake.head_position() == map.food_position() {
            // Tell the snake to grow ...
            snake.set_ate_food(true);
            // ... and spawn new food.
            map.spawn_food(&snake)?;
        }

        // Check if the snake head position is out of bounds.
        if map.is_position_out_of_bounds(snake.head_position()) {
            break;
        }

        // Wait for some time.
        thread::sleep(time::Duration::from_millis(200));
    }

    show_game_over_screen(snake.len())
}

/// Returns a next user event (if there's any).
fn next_event(reader: &mut AsyncReader, snake_direction: Direction) -> Option<Event> {
    // The purpose of this loop is to consume events that are not actionable. Let's
    // say that the snake is moving to the right and the user hits the right arrow
    // key three times and then the up arrow key. The up arrow key would be handled
    // in the 4th iteration of the game loop. That's not what we really want and thus
    // we are consuming all events here till we find an actionable one or none.
    while let Some(event) = reader.next() {
        match event {
            InputEvent::Keyboard(KeyEvent::Char(character)) => {
                if let Ok(new_direction) = Direction::try_from(character) {
                    if snake_direction.can_change_to(new_direction) {
                        return Some(Event::UpdateSnakeDirection(new_direction));
                    }
                }
            }
            InputEvent::Keyboard(KeyEvent::Esc) => return Some(Event::QuitGame),
            InputEvent::Keyboard(key) => {
                if let Ok(new_direction) = Direction::try_from(key) {
                    if snake_direction.can_change_to(new_direction) {
                        return Some(Event::UpdateSnakeDirection(new_direction));
                    }
                }
            }
            _ => {}
        };
    }
    None
}

/// Asks the user for a single map dimension. If the input can't be parsed or is outside
/// of the `min..=default_max` range, `min` or `default_max` is returned.
fn ask_for_map_dimension(name: &str, min: u16, default_max: u16, pos: (u16, u16)) -> Result<u16> {
    let message = format!(
        "Enter map {} (min: {}, default/max: {}):",
        name, min, default_max
    );
    let message_len = message.chars().count() as u16;

    execute!(
        stdout(),
        Goto(pos.0, pos.1),
        PrintStyledFont(style(message).with(Color::Green)),
        Goto(pos.0 + message_len + 1, pos.1)
    )?;

    let dimension = input()
        .read_line()?
        .parse::<u16>()
        .map(|x| {
            if x > default_max {
                default_max
            } else if x < min {
                min
            } else {
                x
            }
        })
        .unwrap_or(default_max);

    Ok(dimension)
}

/// Prints the welcome screen and asks the user for the map size.
fn ask_for_map_size(terminal_size: (u16, u16)) -> Result<(u16, u16)> {
    let mut row = 0u16;

    execute!(
        stdout(),
        Clear(ClearType::All),
        Goto(0, row),
        PrintStyledFont(style(format!("{}", messages::SNAKE.join("\n\r"))).with(Color::Cyan))
    )?;

    row += messages::SNAKE.len() as u16 + 2;
    let width = ask_for_map_dimension("width", 10, terminal_size.0, (0, row))?;
    row += 2;
    let height = ask_for_map_dimension("height", 10, terminal_size.1, (0, row))?;

    execute!(stdout(), Clear(ClearType::All))?;

    Ok((width, height))
}

/// Prints the game over screen.
fn show_game_over_screen(score: usize) -> Result<()> {
    execute!(
        stdout(),
        Clear(ClearType::All),
        Goto(0, 0),
        PrintStyledFont(style(format!("{}", messages::GAME_OVER.join("\n\r"))).with(Color::Red)),
        Goto(0, messages::GAME_OVER.len() as u16 + 2),
        PrintStyledFont(
            style(format!("Your score is {}. You can do better!", score)).with(Color::Red)
        ),
        Show,
        Goto(0, messages::GAME_OVER.len() as u16 + 4)
    )
}
