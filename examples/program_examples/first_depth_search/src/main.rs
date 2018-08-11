extern crate rand;
extern crate crossterm;

mod map;
mod algorithm;
mod messages;
mod variables;

use self::crossterm::{ Crossterm, Screen};
use self::crossterm::terminal::{terminal, ClearType};
use self::crossterm::style::Color;

use self::variables::{Size, Position };
use self::messages::WELCOME_MESSAGE;

use std::io::Read;
use std::iter::Iterator;
use std::{thread, time};

fn main()
{
    run();
}

/// run the program
pub fn run()
{
    // This is represents the current screen.
    let screen = Screen::new();

    // set size of terminal so the map we are going to draw is fitting the screen.
    terminal(&screen).set_size(110,50);

    print_welcome_screen(&screen);

    start_algorithm(&screen);
}

fn start_algorithm(screen: &Screen)
{
    // we first want to switch to alternate screen. On the alternate screen we are going to run or firstdepthsearch algorithm
    if let Ok(ref alternate_screen) = screen.enable_alternate_modes()
    {
        // setup the map size and the position to start searching for a path.
        let map_size = Size::new(100, 40);
        let start_pos = Position::new(10, 10);

        // create and render the map. Or map border is going to have an █ look and inside the map is just a space.
        let mut map = map::Map::new(map_size, '█', ' ');
        map.render_map(&alternate_screen.screen);

        // create the algorithm and start it on the alternate screen. Make sure to pass the refrence to the AlternateScreen screen.
        let mut algorithm = algorithm::FirstDepthSearch::new(map, start_pos, &alternate_screen.screen);
        algorithm.start();
    }
}

fn print_welcome_screen(screen: &Screen)
{
    // create the handle for the cursor and terminal.
    if let Ok(raw) = screen.enable_raw_modes()
        {
            let crossterm = Crossterm::new();
            let terminal = crossterm.terminal(&screen);
            let cursor = crossterm.cursor(&raw.screen);
            let input = crossterm.input(&raw.screen);

            // clear the screen and print the welcome message.
            terminal.clear(ClearType::All);
            cursor.goto(0, 0);
            terminal.write(WELCOME_MESSAGE.join("\n"));

            cursor.hide();
            cursor.goto(0, 10);
            terminal.write(
                "The first depth search algorithm will start in:   Seconds\n\
                Press `q` to abort the program"
            );

            let mut stdin = input.read_async().bytes();

            // print some progress example.
            for i in (1..5).rev() {
                let a = stdin.next();

                if let Some(Ok(b'q')) = a {
                    terminal.exit();
                }

                // print the current counter at the line of `Seconds to Go: {counter}`
                cursor.goto(48, 10);
                crossterm.style(format!("{}", i)).with(Color::Red).on(Color::Blue).paint(&screen);

                // 1 second delay
                thread::sleep(time::Duration::from_secs(1));
            }
        }
}