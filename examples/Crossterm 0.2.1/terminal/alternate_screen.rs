 //alternate screen is not working correctly currently


extern crate crossterm;

 use self::crossterm::{ Context, Terminal };
use self::crossterm::cursor::cursor;
use self::crossterm::terminal::{self, ClearType};
use self::crossterm::terminal::screen;

use std::io::{Write, stdout};
use std::{time, thread};

/// this will print an example wait screen.
fn print_wait_screen(terminal: &Terminal)
 {
     // clear the screen and reset the cursor pos
     terminal::terminal(&terminal).clear(ClearType::All);
     let mut cursor = cursor(&terminal);
     cursor.goto(0, 0);

     // we need to create a scope so that or mutex gueard will be dropped. we need the output also some lines future from here.
     let mut screen_manager = &terminal.screen_manager;
     {
         let mut output = screen_manager.lock().unwrap();
         {
             write!(output.stdout(),
                    "Welcome to the wait screen.\n\
                    Please wait a few seconds until we arrive back at the main screen.\n\n
                    Possessing: "
             );
         }
     }

     for i in 0..5
     {
         // 1 second delay
         thread::sleep(time::Duration::from_secs(1));

         // print the current counter at the line of `Seconds to Go: {counter}`
         cursor.goto(11, 5).print(format!("{} of the 5 items initialized!", i));
     }
 }

 /// this will switch the to alternate modes for 3 seconds after that it wil stitch back to main screen.
pub fn switch_to_alternate_screen_and_back()
{
    // create scope for the alternate screen when the scope ends the screen will be switched back to mainscreen.
    let terminal = Terminal::new();
    {
        let alternate_screen = screen::AlternateScreen::from(&terminal);

        // Print the wait screen.
        print_wait_screen(&terminal);
    }

    terminal::terminal(&terminal).clear(ClearType::All);
    cursor::cursor(&terminal).goto(0,0);
    println!("Whe are back at the main screen");
}

 /// This demonstrates how to switch to alternate screen and main screen.
 pub fn stress_test()
 {
     let terminal = Terminal::new();
     {
         // clear main screen
         terminal::terminal(&terminal).clear(ClearType::All);

         // create alternate screen and switch into that mode.
         let mut alternate_screen = screen::AlternateScreen::from(&terminal);

         // clear the alternate screen.
         terminal::terminal(&terminal).clear(ClearType::All);

         // switch to alternate screen and back to main after three seconds
         write!(alternate_screen, "{}", "We are at the alternatescreen \n");
         thread::sleep(time::Duration::from_secs(3));
         alternate_screen.to_main();
         write!(alternate_screen, "{}", "We are back at the main screen 1\n");

         // switch to alternate screen and back to main after three seconds
         thread::sleep(time::Duration::from_secs(3));
         alternate_screen.to_alternate();
         write!(alternate_screen, "{}", "We are at the alternatescreen 2\n");
         thread::sleep(time::Duration::from_secs(3));
         alternate_screen.to_main();
         write!(alternate_screen, "{}", "We are back at the main screen 2\n");

         // switch to alternate screen and back to main after three seconds
         thread::sleep(time::Duration::from_secs(3));
         alternate_screen.to_alternate();
         write!(alternate_screen, "{}", "We are at the alternatescreen 3\n");
         thread::sleep(time::Duration::from_secs(3));
         alternate_screen.to_main();
         write!(alternate_screen, "{}", "We are back at the main screen 3\n");
     }
 }

 pub fn t()
 {
     use self::crossterm::cursor::cursor;
    use self::crossterm::style::Color;
     use std::io;
     let terminal = Terminal::new();
     {
         let mut alternate_screen = screen::AlternateScreen::from(&terminal);
         terminal::terminal(&terminal).clear(ClearType::All);
         write!(alternate_screen, "{}", "We are at the alternatescreen \n");


         // Goto X: 5 Y: 5
         cursor(&terminal).goto(5,5);
         // Safe cursor position: X: 5 Y: 5
         cursor(&terminal).save_position();
         // Goto X: 5 Y: 20
         cursor(&terminal).goto(5,20);
         // Print at X: 5 Y: 20.
         write!(io::stdout(), "{}", terminal.paint("Yea").with(Color::Blue));
         // Reset back to X: 5 Y: 5.
         cursor(&terminal).reset_position();
         // Print Back at X: 5 Y: 5.
         write!(io::stdout(), "{}", terminal.paint("Back").with(Color::Red));

         println!();

         thread::sleep(time::Duration::from_secs(3));
     }
     terminal::terminal(&terminal).clear(ClearType::All);
     println!("Back at the main screen");
 }