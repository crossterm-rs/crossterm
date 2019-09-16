// Remove once the TODO below is fixed
#![allow(unused_variables)]

use crossterm::{Color, Crossterm};

// use the `Crossterm` to get an instance to the cursor module | demonstration.
// cargo run --example crossterm
fn main() {
    // Create the crossterm type to access different modules.
    let crossterm = Crossterm::new();

    // pass a reference to the current screen.
    let cursor = crossterm.cursor();
    let color = crossterm.color();
    let terminal = crossterm.terminal();
    let terminal = crossterm.input();
    let style = crossterm
        .style("Black text on green background")
        .with(Color::Black)
        .on(Color::Green);

    // TODO: perform some actions with the instances above.
}
