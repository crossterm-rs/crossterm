Let's build some basic programs who are using the alternate and raw screen.

# Raw Screen
_setup the basics_
```rust
extern crate crossterm;

use crossterm::Screen;

fn main() { 
    /* next code here */
}
```

We use the `Screen` type to enable raw mode for the terminal.
When creating a screen you have the option to pass it a boolean, this boolean specifies whether the screen will be in raw or normal mode.

Let's play around a bit to see what raw screen does.

```rust
// Create raw screen by passing in true.
let screen = Screen::new(true);

println!("Some text");
println!("Some text");
```
When you run this program you will directly see that the output is a little strange like:
```
Some text
         Some text
```

Another fun thing to do is reading the input. This should not work since the input is not recorded by the terminal when in raw mode.

_Take note this will cause your terminal to freeze. 
Since `read_line` expects some line input but it will never be recorded because of raw mode. 
You should just restart the terminal when you are stuck_.
```rust
// Create raw screen by passing in true.
let screen = Screen::new(true);
let text = std::io::stdin().read_line(&mut string);
println!("{:?}", text);
```

Note that we spoke about the reason why this is [previously](screen.md#raw-screen).
However, if you want to read input in raw mode you should checkout [Async Input](input.md).

# Alternate Screen
_setup the basics_
```rust
extern crate crossterm;

use crossterm::Screen;
use std::{thread, time};
use std::io::Write;

fn main() { 
    /* next code here */
}
```

As we spoke of [previously](screen.md#alternate-screen), with `Screen` we can manage alternate screen and raw screen.
Let's make some simple program who is switching from the main to alternate screen whereafter it will wait for 2 seconds.
When those seconds are over we will go back to the main screen.
  
First off, create an instance of `Screen`. We can call `enable_alternate_modes()` on this screen, this will switch the screen buffer to the alternate buffer.
When we call this function we will get an `AlternateScreen` instance back which represents the alternate screen.
We should use this instance when we want to do any writing, styling, cursor, and terminal related things on the alternate screen ([Important Notice](screen.md##important-notice)).

```rust
let screen = Screen::default();

if let Ok(mut alternate) = screen.enable_alternate_modes(false) {
    /* next code here */
}
```

Next, we use the instance we got back and write our message to it whereafter we wait for 2 seconds.
We wait 2 seconds to give us some time to see the alternate screen.
If the `AlternateScreen` goes out of scope it will automatically switch back to the main screen.

```rust
write!(alternate.screen, "{}", "Welcome to the wait screen.\n Please wait a 2 seconds until we arrive back at the main screen.");
thread::sleep(time::Duration::from_secs(2));
```


By now you should be able to execute the program, you will see that directly you are being redirected to another screen with no scrollback region. 
You will see this screen open whereafter it closes after 2 seconds.
When the program finishes you will notice you are on the main screen again with it's contents unmodified.

# Perform Actions on Alternate Screen.
Now we have covered the basics of alternate screen let's make a program who styles some text on the 'raw' alternate screen.

_setup the basics_
```rust
extern crate crossterm;

use crossterm::Screen;
use std::{thread, time};
use std::io::Write;

fn main() { 
    // switch to alternate screen and make it raw by passing in true. 
    if let Ok(mut alternate) = screen.enable_alternate_modes(true) {
        /* next code here */
    }
}
```
Some basics steps the following code will do:
1. Create [Crossterm]() type to manage the cursor and styling
2. Set the position of the cursor to x: 0 and y: 0
3. Write the styled text, you can use the two described ways
4. Set the position of the cursor to x: 0 and y: 1
5. Write other text and flush it to the screen
6. Sleep two seconds to see the results

```rust
let alternate_screen = &mut alternate.screen;

// by calling 'from_screen' the cursor will be moved at the passed screen. 
let crossterm = Crossterm::from_screen(alternate_screen);
let cursor = crossterm.cursor();

cursor.goto(0,0);

// 1) print the 'styled object' by converting it into a type that is displayable for alternate screen.
println!("{}", crossterm.style("Welcome to the wait screen.")
                            .with(Color::Red)
                            .on(Color::Blue)
                            .into_displayable(alternate_screen)
);

// 2) use the `paint` method to print it to the alternate screen.
crossterm.style("Welcome to the wait screen.")
    .with(Color::Red)
    .on(Color::Blue)
    .paint(alternate_screen);

cursor.goto(0,1);

write!(alternate_screen, "{}", "Please wait a few seconds until we arrive back at the main screen.");
alternate_screen.flush();

thread::sleep(time::Duration::from_secs(2));

```

As you might have noticed, you need to to manually move the cursor, flush the buffer. This is because the terminal is in raw modes.
Also, by printing with `paint` or calling `into_displayable` you pass in a reference to the alternate screen.
Tip: Take a look at [how](screen.md#Crossterm's implementation) you should use other modules crossterm provides on the alternate screen.

---------------------------------------------------------------------------------------------------------------------------------------------
More examples could be found at this [link](https://github.com/TimonPost/crossterm/tree/master/examples/terminal).

