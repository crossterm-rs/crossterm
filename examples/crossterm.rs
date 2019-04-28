extern crate crossterm;

//use crossterm::{Color, Crossterm};

use crossterm::{
    AlternateScreen, Attribute, ClearType, Crossterm, InputEvent, KeyEvent, Styler, TerminalCursor,
};
use std::io::Write;
use std::{io, thread, time};

fn run() -> io::Result<()> {
    let alternate_screen = AlternateScreen::to_alternate(true)?;
    let crossterm = crossterm::Crossterm::new();
    let input = crossterm.input();
    let mut crossterm_events = input.read_sync();
    loop {
        if let Some(event) = crossterm_events.next() {
            let terminal = crossterm.terminal();
            let cursor = crossterm.cursor();
            cursor.goto(1, 1)?;
            terminal.clear(ClearType::UntilNewLine)?;
            if let InputEvent::Keyboard(key) = &event {
                match key {
                    KeyEvent::Ctrl('q') => {
                        println!("quitting...");
                        ::std::io::stdout().flush();
                        break;
                    }
                    _ => {
                        let s = "event";
                        println!(
                            " {}{}{} : {:?}",
                            Attribute::Bold,
                            s,
                            Attribute::Reset,
                            event
                        );
                    }
                }
            } else {
                println!("disregarding unrelevant event: {:?}", event);
            }
        }
    }
    thread::sleep(time::Duration::from_secs(1));
    Ok(())
}

fn main() {
    match run() {
        Ok(_) => {
            println!("ok");
        }
        Err(e) => {
            println!("error {:?}", e);
        }
    }
}

// use the `Crossterm` to get an instance to the cursor module | demonstration.
//pub fn main() {
//    // Create the crossterm type to access different modules.
//    let crossterm = Crossterm::new();
//
//    // pass a reference to the current screen.
//    let cursor = crossterm.cursor();
//    let color = crossterm.color();
//    let terminal = crossterm.terminal();
//    let terminal = crossterm.input();
//    let style = crossterm
//        .style("Black font on green background")
//        .with(Color::Black)
//        .on(Color::Green);
//
//    // TODO: perform some actions with the instances above.
//}
