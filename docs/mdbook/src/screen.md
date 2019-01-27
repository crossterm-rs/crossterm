# Basic Usage of Screen
As you may have seen crossterm has a type called `Screen`. This type should be used when working with the alternate or raw screen.

Before we continue, I'll explain what those concepts are.

## Screen Buffer
A screen buffer is a two-dimensional array of characters and color data to be output in a console window. An terminal can have multiple of those screen buffers, and the active screen buffer is the one that is displayed on the screen.

Crossterm allows you to switch between those buffers; the screen you are working in is called the 'main screen'.  We call the other screen the 'alternate screen'.

### Alternate Screen
Normally you are working on the main screen but an alternate screen is somewhat different from a normal screen.
For example, it has the exact dimensions of the terminal window, without any scrollback region. An example of this is vim when it is launched from bash.

Vim uses the entirety of the screen to edit the file, then exits to bash leaving the original buffer unchanged.

Crossterm provides the ability to switch to the alternate screen, make some changes, and then go back to the main screen. The main screen will still have its original data since we made all the edits on the alternate screen.

## Raw screen
To understand the concept of a 'raw screen' let's look at the following points:

**No line buffering.**
Normally the terminals use line buffering. This means that the input will be sent to the terminal line by line. With raw mode, the input will send one byte at a time.

**Input**

 All input has to be written to the screen buffer manually by the programmer.

**Characters**

The characters are not processed by the terminal driver. Also, special character have no meaning. For example, backspace will not be interpreted as backspace but instead will be sent directly to the terminal.

**Escape Characters**
Note that in raw mode `\n` `\r` will move the cursor to a new line but it will be at the same position as it was on the previous line.

_example of what I mean_
 ```
 some text
          some text
 ```

To start at the beginning of the next line, use `\n\r`.

# Crossterm's implementation

When we want to print some text to the alternate screen we can't just write on it using `print!(), println!(), or write!()`.

This is because those functions are writing to the standard output and not to our alternate screen we are currently on.  
The same goes for coloring, cursor movement, input, and terminal actions.
 
Crossterm provides a solution for that by introducing the `Screen` type. 
You can use the 'alternate' or 'raw' screen functionalities by either using the [crossterm](https://crates.io/crates/crossterm) or [crossterm_screen](https://crates.io/crates/crossterm_screen) crate.

Please checkout this [example](screen_example.md) for more information on how to use it.

_Cargo.toml_
```rust
crossterm = { version =  "0.6.0", features = ["screen","terminal","cursor", "style", "input"] }
```

```rust
use crossterm::{cursor, TerminalCursor};
use crossterm::{color, TerminalColor};
use crossterm::{input, TerminalInput};
use crossterm::{terminal, Terminal};

let screen = Screen::default();

if let Ok(alternate) = screen.enable_alternate_modes(false) {

    // by calling 'from_screen' you force crossterm to use the screen of the alternate screen to perform actions on.
    let crossterm = Crossterm::from_screen(&alternate.screen);
    let cursor = crossterm.cursor();
    let terminal =crossterm.terminal();
    let input = crossterm.input();
    let color = crossterm.color();
        
    // you can also create instances directly without `Crossterm`
    let screen = alternate.screen;
    
    let terminal = Terminal::from_output(&screen.stdout);
    let cursor = TerminalCursor::from_output(&screen.stdout);
    let color = TerminalColor::from_output(&screen.stdout);
    let input = TerminalInput::from_output(&screen.stdout);
}
```

The above modules will now all be executed at the 'alternate screen'.

---------------------------------------------------------------------------------------------------------------------------------------------
Next up: [Examples](screen_example.md)