extern crate crossterm;
extern crate rand;

mod algorithm;
mod map;
mod messages;
mod variables;

use self::crossterm::{
    color, cursor, input, terminal, AlternateScreen, ClearType, Color, Colored, Crossterm,
    InputEvent, KeyEvent, RawScreen,
};
use self::variables::{Position, Size};

use std::iter::Iterator;
use std::{thread, time};

fn main() {
    run();
}

/// run the program
pub fn run() {
    //    let screen = RawScreen::into_raw_mode().expect("failed to enable raw modes");
    print_welcome_screen();
    start_algorithm();
    exit();
}

fn start_algorithm() {
    // we first want to switch to alternate screen. On the alternate screen we are going to run or firstdepthsearch algorithm
    if let Ok(ref alternate_screen) = AlternateScreen::to_alternate(true) {
        // setup the map size and the position to start searching for a path.
        let map_size = Size::new(50, 40);
        let start_pos = Position::new(10, 10);

        // create and render the map. Or map border is going to have an █ look and inside the map is just a space.
        let mut map = map::Map::new(map_size, '█', ' ');
        map.render_map();

        // create the algorithm and start it on the alternate screen. Make sure to pass the refrence to the AlternateScreen screen.
        let mut algorithm = algorithm::FirstDepthSearch::new(map, start_pos);
        algorithm.start();
    }
}

fn print_welcome_screen() {
    // we have to keep this screen arround to prevent te
    let _screen = RawScreen::into_raw_mode();
    let crossterm = Crossterm::new();

    // create the handle for the cursor and terminal.
    let terminal = terminal();
    let cursor = cursor();
    let input = input();

    // set size of terminal so the map we are going to draw is fitting the screen.
    terminal.set_size(110, 60);

    // clear the screen and print the welcome message.
    terminal.clear(ClearType::All);
    cursor.goto(0, 0);

    print!(
        "{}",
        crossterm
            .style(format!("{}", messages::WELCOME_MESSAGE.join("\n\r")))
            .with(Color::Cyan)
    );

    cursor.hide();
    cursor.goto(0, 10);
    terminal.write("The first depth search algorithm will start in:   Seconds");

    cursor.goto(0, 11);
    terminal.write("Press `q` to abort the program");

    let mut stdin = input.read_async();

    // print some progress example.
    for i in (1..5).rev() {
        if let Some(InputEvent::Keyboard(KeyEvent::Char('q'))) = stdin.next() {
            exit();
            terminal.exit();
            break;
        } else {
            // print the current counter at the line of `Seconds to Go: {counter}`
            cursor.goto(48, 10);
            print!(
                "{}{}{}",
                Colored::Fg(Color::Red),
                Colored::Bg(Color::Blue),
                i
            );
        }

        color().reset();

        // 1 second delay
        thread::sleep(time::Duration::from_secs(1));
    }
}

fn exit() {
    RawScreen::disable_raw_mode().expect("failed to disable raw modes.");
    cursor().show();
    color().reset();
}
