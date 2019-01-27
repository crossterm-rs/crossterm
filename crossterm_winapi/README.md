# Crossterm Winapi | Common WinApi Abstractions
 ![Lines of Code][s7] [![Latest Version][s1]][l1] [![MIT][s2]][l2] [![docs][s3]][l3] ![Lines of Code][s6]

[s1]: https://img.shields.io/crates/v/crossterm_winapi.svg
[l1]: https://crates.io/crates/crossterm_winapi

[s2]: https://img.shields.io/badge/license-MIT-blue.svg
[l2]: LICENSE

[s3]: https://docs.rs/crossterm_winapi/badge.svg
[l3]: https://docs.rs/crossterm_winapi/

[s6]: https://tokei.rs/b1/github/TimonPost/crossterm_winapi?category=code
[s7]: https://travis-ci.org/TimonPost/crossterm_winapi.svg?branch=master

This crate provides some wrappers aground common used WinApi functions. 
The purpose of this library is originally meant for [crossterm](https://github.com/TimonPost/crossterm), 
and it is very unstable right because of that some changes could be expected.

# Features
This crate provides some abstractions over: 

- CONSOLE_SCREEN_BUFFER_INFO (used to extract information like cursor pos, terminal size etc.)
- HANDLE (the handle needed to run functions from WinApi)
- SetConsoleActiveScreenBuffer (activate an other screen buffer)
- Set/GetConsoleMode (e.g. console modes like disabling output)
- SetConsoleTextAttribute (eg. coloring)
- SetConsoleWindowInfo (changing the buffer location e.g. scrolling)
- FillConsoleOutputAttribute, FillConsoleOutputCharacter (used to replace some block of cells with a color or character.)
- SetConsoleInfo

# Example 
Here are some examples do demonstrate how to work whit this crate. 
Please see [examples](https://github.com/TimonPost/crossterm_winapi) for more
## Screenbuffer information
```rust 
use crossterm_winapi::{ScreenBuffer, Handle};

fn print_screen_buffer_information() {
    let screen_buffer = ScreenBuffer::current().unwrap();

    // get console screen buffer information
    let csbi = screen_buffer.info().unwrap();

    println!("cursor post: {:?}", csbi.cursor_pos());
    println!("attributes: {:?}", csbi.attributes());
    println!("terminal window dimentions {:?}", csbi.terminal_window());
    println!("terminal size {:?}", csbi.terminal_size());
}
```
## Handle 
```rust
use crossterm_winapi::{HandleType, Handle};

fn get_different_handle_types() {
    let out_put_handle = Handle::new(HandleType::OutputHandle).unwrap();
    let out_put_handle = Handle::new(HandleType::InputHandle).unwrap();
    let curr_out_put_handle = Handle::new(HandleType::CurrentOutputHandle).unwrap();
    let curr_out_put_handle = Handle::new(HandleType::CurrentInputHandle).unwrap();
}
```


### Inspiration
I wanted to expose some of the api crossterm uses for WinApi. 
1. I thought it would be helpful for other people to, to have a small rust seemable abstraction over the WinApi bindings.
2. I have some future plans for crossterm wherefore I needed to seperate the WinAPi logic out of the currenbt librarie.
