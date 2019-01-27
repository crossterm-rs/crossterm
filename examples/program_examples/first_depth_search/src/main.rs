extern crate crossterm;
extern crate rand;

mod algorithm;
mod map;
mod messages;
mod variables;

use self::crossterm::{terminal, ClearType, Color, Crossterm, Screen};

use self::messages::WELCOME_MESSAGE;
use self::variables::{Position, Size};

use std::io::Read;
use std::iter::Iterator;
use std::{thread, time};

fn main() {
    run();
}

/// run the program
pub fn run() {
    print_welcome_screen();

    // This is represents the current screen.
    let mut screen = Screen::new(true);
    start_algorithm(&screen);
}

fn start_algorithm(screen: &Screen) {
    // we first want to switch to alternate screen. On the alternate screen we are going to run or firstdepthsearch algorithm
    if let Ok(ref alternate_screen) = screen.enable_alternate_modes(true) {
        // setup the map size and the position to start searching for a path.
        let map_size = Size::new(50, 40);
        let start_pos = Position::new(10, 10);

        // create and render the map. Or map border is going to have an █ look and inside the map is just a space.
        let mut map = map::Map::new(map_size, '█', ' ');
        map.render_map(&alternate_screen.screen);

        // create the algorithm and start it on the alternate screen. Make sure to pass the refrence to the AlternateScreen screen.
        let mut algorithm =
            algorithm::FirstDepthSearch::new(map, start_pos, &alternate_screen.screen);
        algorithm.start();
    }
}

fn print_welcome_screen() {
    let mut screen = Screen::new(true);

    let crossterm = Crossterm::from_screen(&screen);

    // create the handle for the cursor and terminal.
    let terminal = crossterm.terminal();
    let cursor = crossterm.cursor();
    let input = crossterm.input();

    // set size of terminal so the map we are going to draw is fitting the screen.
    terminal.set_size(110, 60);

    // clear the screen and print the welcome message.
    terminal.clear(ClearType::All);
    cursor.goto(0, 0);
    terminal.write(WELCOME_MESSAGE.join("\r\n"));

    cursor.hide();
    cursor.goto(0, 10);
    terminal.write("The first depth search algorithm will start in:   Seconds");

    cursor.goto(0, 11);
    terminal.write("Press `q` to abort the program");

    let mut stdin = input.read_async().bytes();

    // print some progress example.
    for i in (1..5).rev() {
        let a = stdin.next();

        if let Some(Ok(b'q')) = a {
            drop(screen);
            terminal.exit();
            break;
        } else {
            // print the current counter at the line of `Seconds to Go: {counter}`
            cursor.goto(48, 10);
            crossterm
                .style(format!("{}", i))
                .with(Color::Red)
                .on(Color::Blue)
                .paint(&screen.stdout);
        }

        // 1 second delay
        thread::sleep(time::Duration::from_secs(1));
    }
}
