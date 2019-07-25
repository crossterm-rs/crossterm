extern crate crossterm_screen;

use crossterm_screen::{IntoRawMode, RawScreen};

use std::io::{stdout, Write};
use std::{thread, time};

pub fn raw_modes() {
    // create a Screen instance that operates on the default output: io::stdout(). By passing in 'true', we make this screen 'raw'
    let screen = RawScreen::into_raw_mode();
    let screen = stdout().into_raw_mode();

    // raw screen will be disabled when it goes out of scope.
}
