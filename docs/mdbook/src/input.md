Crossterm provides a way to work with the terminal input. We will not cover the basic usage but instead asynchronous and synchronous reading of input.
Please check out these [examples](https://github.com/TimonPost/crossterm/blob/master/examples/input/keyboard/input.rs) for reading a line or a character from the user.

## Differences Synchronous and Asynchronous
Crossterm provides two ways to read user input, synchronous and asynchronous. 

### Synchronous reading 

Read the input synchronously from the user, the reads performed will be blocking calls.
Using synchronous over asynchronous reading has the benefit that it is using fewer resources than the asynchronous because background thread and queues are left away."

### Asynchronous reading

Read the input asynchronously, input events are gathered on the background and will be queued for you to read.
Using asynchronous reading has the benefit that input events are queued until you read them. You can poll for occurred events, and the reads won't block your program. 

### Technical details
On UNIX systems crossterm reads from the TTY, on Windows, it uses `ReadConsoleInputW`. For asynchronous reading, a background thread will be fired up to read input events. 
Occurred events will be queued on an MPSC-channel for the user to iterate over.

# Example
In the following example, we will create a small program that will listen for mouse and keyboard input.
On the press of the 'escape' key, the program will be stopped.

So let's start by setting up the basics.

```
use std::{thread, time::Duration};
use crossterm::{TerminalInput, Screen, InputEvent, KeyEvent};

fn main() {
    println!("Press 'ESC' to quit.");

    /* next code here */
}
```

Next, we need to put the terminal into raw mode. We do this because we don't want the user input to be printed to the terminal screen.

```rust
// enable raw modes by passing in 'true'
let screen = RawScreen::into_raw_mode();

// create a input from our screen.
let input = TerminalInput::new();

/* next code here */
```

Now that we have constructed a `TerminalInput` instance we can go ahead an start the reading. 
Do this by calling `input.read_async()`, which returns an [AsyncReader](https://docs.rs/crossterm/0.7.0/crossterm/struct.AsyncReader.html).
This is an iterator over the input events that you could as any other iterator.  

```rust
let mut async_stdin = input.read_async();

loop {
    if let Some(key_event) = async_stdin.next() {
        /* next code here */
    }
    thread::sleep(Duration::from_millis(50));
}
```

The [AsyncReader](https://docs.rs/crossterm/0.7.0/crossterm/struct.AsyncReader.html) iterator will return `None` when nothing is there to read, `Some(InputEvent)` if there are events to read. 
I use a thread delay to prevent spamming the iterator. 

Next up we can start pattern matching to see if there are input events we'd like to catch. 
In our case, we want to catch the `Escape Key`. 

```rust
  match key_event {
    InputEvent::Keyboard(k) => match k {
        KeyEvent::Esc => {
            println!("Program closing ...");
            break
        }
        _ => println!("Key {:?} was pressed!", k)
    }
    _ => { /* Mouse Event */ }
}
```

As you see, we check if the `KeyEvent::Esc` was pressed, if that's true we stop the program by breaking out of the loop.

_final code_
```rust
fn main() {
    println!("Press 'ESC' to quit.");

    // enable raw modes by passing in 'true'
    let screen = Screen::new(true);

    // create a input from our screen.
    let input = TerminalInput::from_output(&screen.stdout);

    // create async reader
    let mut async_stdin = input.read_async();
    
    loop {
        // try to get the next input event.
        if let Some(key_event) = async_stdin.next() {
            match key_event {
                InputEvent::Keyboard(k) => match k {
                    KeyEvent::Esc => {
                        println!("Program closing ...");
                        break;
                    }
                    _ => println!("Key {:?} was pressed!", k)
                }
                _ => { }
            }
        }
        thread::sleep(Duration::from_millis(50));
    }
} // <=== background reader will be disposed when dropped.
 ```
---------------------------------------------------------------------------------------------------------------------------------------------
More robust and complete examples on all input aspects like mouse, keys could be found [here](https://github.com/TimonPost/crossterm/tree/master/examples/).