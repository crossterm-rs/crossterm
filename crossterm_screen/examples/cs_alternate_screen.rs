use crossterm_screen::AlternateScreen;

/// print wait screen on alternate screen, then switch back.
#[allow(unused_variables)]
fn main() {
    // move to the alternate screen, 'false' determines if the alternate screen should be in raw mode.
    if let Ok(alternate) = AlternateScreen::to_alternate(false) {
        // do some stuff on the alternate screen.
    } // <- alternate screen will be disabled when dropped.
}
