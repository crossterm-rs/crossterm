extern crate crossterm;

use crossterm::{Crossterm, Screen};
use crossterm::style::Color;

/// use the `Crossterm` to get an instance to the cursor module | demonstration.
pub fn use_crossterm_cursor()
{
    let screen = Screen::default();

    // Create the crossterm type to access different modules.
    let crossterm = Crossterm::new(&screen);

    // pass a reference to the current screen.
    let cursor = crossterm.cursor();
    let color = crossterm.color();
    let terminal = crossterm.terminal();
    let style = crossterm.style("").with(Color::Black).on(Color::Green);

    // perform some actions with the instances above.
}
