extern crate crossterm_screen;

use crossterm_screen::Screen;

use std::io::{stdout, Write};
use std::{thread, time};

/// print wait screen on alternate screen, then switch back.
pub fn print_wait_screen_on_alternate_window() {
    let screen = Screen::default();

    // move to alternate screen, 'false' means if the alternate screen should be in raw modes.
    if let Ok(alternate) = screen.enable_alternate_modes(false) {
        // do some stuff on the alternate screen.
    } // <- alternate screen will be disabled when dropped.
}
