extern crate crossterm;

use crossterm::{Crossterm, Screen};

#[test]
/// use the `Crossterm` to get an instance to the cursor module | demonstration.
pub fn use_crossterm_cursor()
{
    let screen = Screen::new();

    // Create the crossterm type to access different modules.
    let crossterm = Crossterm::new();

    // pass a reference to the current screen.
    let cursor = crossterm.cursor(&screen);
    let color = crossterm.color(&screen);
    let terminal = crossterm.terminal(&screen);

    // perform some actions with the instances above.
}
