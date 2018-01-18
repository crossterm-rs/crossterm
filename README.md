# Crossterm | crossplatform terminal library written in rust.

Ever got disappointed when a terminal library for rust was only written for unix systems? 
Crossterm provides the same terminal functionality for both windows and unix systems.

Crossterm aims to be simple and easy to call in code. 
True the simplicty of crossterm you do not have to worry about the platform your working with.
You can just call some module and unther water it will check what to do based on the current platform.

## Getting Started

Add the crossterm package to your `Cargo.toml` file.

```
[dependencies]
crossterm = "*"

```

Add the crate to your solution.
And use the crossterm modules withs you want to use.

```
extern crate crossterm;

// this module is used for styling the terminal
use self::crossterm::crossterm_style::*;
// this module is used for cursor related actions
use self::crossterm::crossterm_cursor::*;
// this mudule is used for terminal related actions
use self::crossterm::crossterm_terminal::*;

```
## Documentation

Documentation for the code can be found here ...

## Examples

For detailed examples of all crossterm functionalities check the `./examples/` direcory.

### Styled font
```rust    
    use crossterm_style::{paint, Color};
    
    // Crossterm provides method chaining so that you can style the font nicely.
    // You can either store the styled font.
    let mut styledobject = paint("Stored styled font").with(Color::Red).on(Color::Blue);
    println!("{}",styledobject);
    
    // Or you can print it directly.
    println!("{}", paint("Red font on blue background color").with(Color::Red).on(Color::Blue));     
    println!("{}", paint("Red font on default background color").with(Color::Red));
    println!("{}", paint("Default font color and Blue background color").on(Color::Blue));
```
### Cursor
```rust 
    use crossterm_cursor::get;
    
    let mut cursor = get();
    
    // Set the cursor to position X: 10, Y: 5 in the terminal
    cursor.goto(10,5);   
    
    // Move the cursor to position 3 times to the up in the terminal
    cursor.move_up(3)
    
     // Move the cursor to position 3 times to the right in the terminal
    cursor.move_right(3);
    
    // Move the cursor to position 3 times to the down in the terminal
    cursor.move_down(3);
    
     // Move the cursor to position 3 times to the left in the terminal
    cursor.move_left(3);
    
    // Print an character at X: 10, Y: 5 (see examples for more explanation why to use this method).
    cursor.goto(10,5).print("@");    
    
```

### Terminal
```rust 
    use crossterm_terminal::get;
    
    let mut cursor = get();
    
    // Clear all lines in terminal;
    terminal.clear(ClearType::All);  
    // Clear all cells from current cursor position down.
    terminal.clear(ClearType::FromCursorDown);
    // Clear all cells from current cursor position down.
    terminal.clear(ClearType::FromCursorUp);
     // Clear current line cells.
    terminal.clear(ClearType::CurrentLine);
    // Clear all the cells until next line.
    terminal.clear(ClearType::UntilNewLine);
    
    // Get terminal size
    let terminal_size = terminal.terminal_size().unwrap();
    // Print results
    print!("X: {}, y: {}", terminal_size.0, terminal_size.1);
     
    // Scroll down 10 lines.
    terminal.scroll_down(10);
    
    // Scroll up 10 lines.
    terminal.scroll_up(10);
```

## Features crossterm 0.1

- Cursor movement.
    - Up, Down, Left, Right.
    - Goto an certain position.
- Styled output
    - Foreground color (16 base colors)
    - Background color (16 base colors)
- Terminal
    - Clearing
    - Scrolling
    - Size
- Detailed documentation on every item.
- Full examples for every call.

## Tested terminals

- Windows Powershell
    - Windows 10 (pro)
- Windows CMD
    - Windows 10 (pro)
- Ubuntu Desktop Terminal
    - Ubuntu 17.10


The above terminals have been tested.
Crossterm should works also for windows 7, 8 consoles and all ansi suportable consoles. 
But these are yet to be tested.
If you have used this library for an terminal other than the above list without issues feel free to add it to the above list.
    

## Notice 
This library is not totaly stable **yet**. There will not be changed mutch in the code design so do not worry to mutch. 
If there are any changes that affect previous versions I will describe what to change when upgrading crossterm to new version.

## Todo crossterm 0.2

- Handling mouse events 
- Inplementing 256 colors for terminals that support those colors.
- Handling key events
- Tests

## Contributing

Please read [CONTRIBUTING.md](https://gist.github.com/PurpleBooth/b24679402957c63ec426) for details on our code of conduct, and the process for submitting pull requests to us.

## Versioning

The current version is crossterm 0.1, every merge I do the version go's up like 0.1.0 -> 0.1.1 -> 0.1.2. 

When new features arive the packages will go up like 0.1 -> 0.2 -> 0.3

## Authors

* **Timon Post** - *Project Owner*

See also the list of [contributors](https://github.com/your/project/contributors) who participated in this project.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details



