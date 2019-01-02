Crossterm provides a way to work with the terminal input. We will not cover the basic usage but instead asynchronous reading of input.
Please checkout these [examples](https://github.com/TimonPost/crossterm/blob/master/examples/input/keyboard/input.rs) for reading a line or a character from the user.

So what does 'reading async input' mean?
This means that you can catch user input without your program having to wait for it.
The user input will be read from another thread.
UNIX systems will get input from TTY and Windows will get input with '_getwch' and '_getwche'.

This could be useful in a case where you want to perform some logic with a periodic check if the user entered some character.

# Example
In the following example we will run some loop until the user has pressed 'x'.

So lets start by setting up the basics.

```
use std::io::Read;
use crossterm::{input, Screen};
use std::{thread, time::Duration};

fn main() {
    println!("Press 'x' to quit.");

    /* next code here */
}
```

Next we need to put the terminal into raw mode. We do this because whe don't want the user input to be printed to the terminal screen.
Once the user pressed 'x' we manually want to process it and stop the loop.

```rust
// enable raw modes by passing in 'true'
let screen = Screen::new(true);

// create a input from our screen.
let input = input::from_screen(&screen);

/* next code here */
```

Take note that whe need to use our 'Screen' to create an `TerminalInput` instance, check [this](screen.md#important-notice) out for more information why that is.

Next we call `input.read_async()`. This will spin up a thread which will poll all input from the user.
This thread send all input via an 'mpsc' channel to the `AsyncReader` we got back from this function.

By calling `bytes()` on the `AsyncReader` we get an iterator back over the characters (bytes).

```rust
let mut stdin = input.read_async().bytes();

/* next code here */
```

Now we got an iterator back we can call `next()` on it to get the next pressed character (byte).
If an character is pressed we will get `Some(Ok(u8))` back which we compare to see if 'x' is pressed.
If the 'x' is pressed we break the loop.

```rust
loop {
    let pressed_char: Option<Result<u8>> = stdin.next();

    // check if the pressed char is equal to 'c'
    if let Some(Ok(b'x')) = pressed_char {
        println!("The key: `x` was pressed and program is terminated.");
        break;
    }

    thread::sleep(Duration::from_millis(20));
}
```

You should now have a fully functional program waiting for the user to press 'x'.
User input will be recorded on the background so that the main logic of your program can continue.
The code from this tutorial could be found [here](https://github.com/TimonPost/crossterm/blob/master/examples/input/keyboard/async_input.rs#L45).

---------------------------------------------------------------------------------------------------------------------------------------------
More examples could be found at this [link](https://github.com/TimonPost/crossterm/tree/master/examples/input/keyboard).