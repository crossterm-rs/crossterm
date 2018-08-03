extern crate rand;
extern crate crossterm;

mod map;
mod algorithm;
mod messages;
mod variables;

use crossterm::Crossterm;
use crossterm::terminal::ClearType;
use crossterm::style::Color;

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
//    // create new Crossterm instance.
    let mut crossterm = Crossterm::new();
    // set size of terminal so the map we are going to draw is fitting the screen.
    crossterm.terminal().set_size(110,50);

    print_welcome_screen(&mut crossterm);

    start_algorithm(&mut crossterm);

    print_end_screen(&crossterm);
}

fn start_algorithm(crossterm: &mut Crossterm)
{
    // we first want to switch to alternate screen. On the alternate screen we are going to run or firstdepthsearch algorithm
    crossterm.to_alternate_screen();

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

fn print_welcome_screen(crossterm: &mut Crossterm)
{
    // create the handle for the cursor and terminal.

    crossterm.enable_raw_mode();
    let mut terminal = crossterm.terminal();
    let mut cursor = crossterm.cursor();

    // clear the screen and print the welcome message.
    terminal.clear(ClearType::All);
    cursor.goto(0,0);
    terminal.write(WELCOME_MESSAGE.join("\n"));

    cursor.hide();
    cursor.goto(0,10);
    terminal.write(
        "The first depth search algorithm will start in:   Seconds\n\
        Press `q` to abort the program"
    );

    let input = crossterm.input();
    let mut stdin = input.read_async().bytes();

    // print some progress example.
    for i in (1..5).rev() {

        let a = stdin.next();

        if let Some(Ok(b'q')) = a {
            terminal.exit();
        }

        // print the current counter at the line of `Seconds to Go: {counter}`
        cursor
            .goto(48, 10)
            .print(crossterm.paint(format!("{}", i)).with(Color::Red).on(Color::Blue));

        // 1 second delay
        thread::sleep(time::Duration::from_secs(1));
    }
}