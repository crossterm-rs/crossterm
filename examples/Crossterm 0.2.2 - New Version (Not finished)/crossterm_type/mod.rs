extern crate crossterm;

use crossterm::Crossterm;

/// use the `Crossterm` to get an instance to the cursor module | demonstration.
pub fn use_crossterm_cursor()
{
    let crossterm = Crossterm::new();
    let mut cursor = crossterm.cursor();
    cursor.goto(5,5).print("test");
}

use crossterm::style::Color;

/// use the `Crossterm` to get an instance to the color module | demonstration.
pub fn use_crossterm_color()
{
    let crossterm = Crossterm::new();
    let mut color = crossterm.color();
    color.set_bg(Color::Red);
    color.set_fg(Color::Green);
}

use crossterm::terminal::ClearType;

/// use the `Crossterm` to get an instance to the terminal module | demonstration.
pub fn use_crossterm_terminal()
{
    let crossterm = Crossterm::new();
    let mut terminal = crossterm.terminal();
    terminal.clear(ClearType::All);
    terminal.set_size(40,40);
}

/// paint text with colors using `Crossterm` | demonstration.
pub fn use_crossterm_paint()
{
    let crossterm = Crossterm::new();
    crossterm.paint("Black on BLUE").with(Color::Black).on(Color::Blue);
}

/// write text to terminal using `Crossterm` | demonstration.
pub fn use_crossterm_write()
{
    let crossterm = Crossterm::new();
    crossterm.write("some text \nsome text on new line");
}

/// Switch to alternate screen using the `Context` of `Crossterm` | demonstration.
pub fn create_alternate_screen_from_crossterm()
{
    use crossterm::screen::*;
    use std::convert::From;

    let crossterm = Crossterm::new();

    {
        // move into alternate screen
        let alternate_screen = AlternateScreen::from(crossterm.context());

        // this will move the cursor and print `some text` on the alternate screen.
        crossterm.cursor().goto(10, 10).print("Some text");
    } // <- alternate screen ends here an will be switched back to main screen.

    // print "Some other text" on the mainscreen at x: 0, y: 10
    crossterm.cursor().goto(0,10).print("Some other text");
}
