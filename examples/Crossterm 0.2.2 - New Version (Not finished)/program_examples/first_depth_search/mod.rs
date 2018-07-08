extern crate rand;

mod map;
mod variables;
mod messages;
mod algorithm;

use crossterm::Crossterm;
use crossterm::terminal::ClearType;
use crossterm::style::Color;
use crossterm::screen;

use self::variables::{Size, Position };
use self::messages::WELCOME_MESSAGE;

use std::iter::Iterator;
use std::{thread, time};

/// run the program
pub fn run()
{
    // create new Crossterm instance.
    let mut crossterm = Crossterm::new();

    print_welcome_screen(&crossterm);

    start_algorithm(&mut crossterm);

    print_end_screen(&crossterm);
}

fn start_algorithm(crossterm: &mut Crossterm)
{
    // we first want to switch to alternate screen. On the alternate screen we are going to run or firstdepthsearch algorithm
    let alternate_screen = screen::AlternateScreen::from(crossterm.context());

    // setup the map size and the position to start searching for a path.
    let map_size = Size::new(100,40);
    let start_pos = Position::new(10,10);

    // create and render the map. Or map border is going to have an █ look and inside the map is just a space.
    let mut map = map::Map::new(map_size, '█', ' ');
    map.render_map(crossterm);

    // create the algorithm and start the
    let mut algorithm = algorithm::FirstDepthSearch::new(map, start_pos, &crossterm);
    algorithm.start();
}

fn print_end_screen(crossterm: &Crossterm)
{

}

fn print_welcome_screen(crossterm: &Crossterm)
{
    // create the handle for the cursor and terminal.
    let mut cursor = crossterm.cursor();
    let mut terminal = crossterm.terminal();

    // clear the screen and print the welcome message.
    terminal.clear(ClearType::All);
    terminal.write(WELCOME_MESSAGE.join("\n"));

    cursor.hide();
    cursor.goto(0,10);
    terminal.write(
        "The first depth search algorithm will start in:   Seconds"
    );

    // print some progress example.
    for i in (1..5).rev() {
        // print the current counter at the line of `Seconds to Go: {counter}`
        cursor
            .goto(48, 10)
            .print(terminal.paint(format!("{}", i)).with(Color::Red).on(Color::Blue));

        // 1 second delay
        thread::sleep(time::Duration::from_secs(1));
    }
}