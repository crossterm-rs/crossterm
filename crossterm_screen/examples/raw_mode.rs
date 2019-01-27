extern crate crossterm_screen;

use crossterm_screen::Screen;

use std::io::{stdout, Write};
use std::{thread, time};

pub fn raw_modes() {
    // create a Screen instance who operates on the default output; io::stdout().
    let screen = Screen::default();

    // create a Screen instance who operates on the default output; io::stdout(). By passing in 'true' we make this screen 'raw'
    let screen = Screen::new(true);

    drop(screen); // <-- by dropping the screen raw modes will be disabled.
}
