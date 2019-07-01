extern crate crossterm_screen;

use crossterm_screen::AlternateScreen;

use std::io::{stdout, Write};
use std::{thread, time};

/// print wait screen on alternate screen, then switch back.
pub fn print_wait_screen_on_alternate_window() {
    // move to the alternate screen, 'false' determines if the alternate screen should be in raw mode.
    if let Ok(alternate) = AlternateScreen::to_alternate(false) {
        // do some stuff on the alternate screen.
    } // <- alternate screen will be disabled when dropped.
}
