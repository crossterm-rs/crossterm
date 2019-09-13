use std::io::stdout;

use crossterm_screen::{IntoRawMode, RawScreen};

#[allow(unused_variables)]
fn main() {
    // create a Screen instance that operates on the default output: io::stdout(). By passing in 'true', we make this screen 'raw'
    let screen = RawScreen::into_raw_mode();
    let screen = stdout().into_raw_mode();

    // raw screen will be disabled when it goes out of scope.
}
