# This is the development branch do not use this in production. This code can be broken and contains code that could not function correctly.

Things where I am working on now:

I have inplemented the alternate and raw screen features for unix systems. Now I am trying to get this also to work for windows with WINAPI. 

The next version will have api braking changes. Why I needed to do that is essential for the functioning of these above features. 

- At first:
I needed to create some `Context` types withs can manage the terminal state. So that when changes are made to the terminal the can be reverted. This is handy when using raw terminal mode and enabling some mode on the terminal like ansi escape codes for windows. When the `Context` dispposes all changes made will be reverted so that the user terminal is back in its starting state.

When in unix sytems you want to execute some ANSI escape code you have to write it to terminal stdout (screen ouput). 

    //like
    write!(std::io::stdout(), "{}", "some ANSI code".
    
But when using `std::io::stdout` you will have an handle to the current screen. And not the alternate screen. And when using alternate screen you don't want to write to the mainscreen stdout. But to the alternate screen stdout. For this we also have the `Context` type withs has contains an type to manage the screen. 

So that is why I have created the `Context` type. To mange the terminal state changes and to run cleanup code. And for managegin the screen output.

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
- Examples for every client callable code.

## Features crossterm 0.2

- 256 color support. 
- Text Attributes like: bold, italic, underscore and crossed word ect. 
- Custom ANSI color code input to set fore- and background color for unix.
- Storing the current cursor position and resetting to that stored cursor position later. 
- Resizing the terminal.

### fixes in crossterm 0.2.1

- Default ANSI escape codes for windows machines, if windows does not support ANSI switsh back to WINAPI.
- method grammer mistake fixed [Issue 3](https://github.com/TimonPost/crossterm/issues/3)
- Some Refactorings in method names see [issue 4](https://github.com/TimonPost/crossterm/issues/4)
- Removed bin refrence from crate [Issue 6](https://github.com/TimonPost/crossterm/issues/6)
- The terminal state will be set to its original state when process ends [issue7](https://github.com/TimonPost/crossterm/issues/7).
- Get position unix fixed [issue 8](https://github.com/TimonPost/crossterm/issues/8)


## TODO Features crossterm 0.3
- Raw state implementation [Issue 5](https://github.com/TimonPost/crossterm/issues/5).
- Alternate screen implementation.
- Tests

## Tested terminals

- Windows Powershell
    - Windows 10 (pro)
- Windows CMD
    - Windows 10 (pro)
- Ubuntu Desktop Terminal
    - Ubuntu 17.10


The above terminals have been tested. Crossterm should works also for windows 7, 8 consoles and all ansi supportable consoles.
But these are yet to be tested. 
If you have used this library for an terminal other than the above list without issues feel free to add it to the above list.
    
## How it works

Crossterm is using `WINAPI` for windows systems and `ANSI escape codes` for unix systems. Crossterm provides one base trait with can be implemented for a platform specific instance. For example, there is an implementation for windows (`WINAPI`) and unix(`ANSI`) for the `cursor module`. To call the platform specific implementation there is one module that rules them all. Thrue this module the client calls some action and the module will deside what to do based on the current platform. And it will execute that action.

## Notice 
This library is library is stable. There will not be changed mutch in the code design so do not worry to mutch. If there are any changes that affect previous versions I will describe what to change when upgrading crossterm to an newer version.

## Todo

- This library does not support any kind of raw terminal. When an terminal changes some core state of the terminal this state should be revered when the process ends from this library. Currently there are not made any changed to the core state of the terminal with this library. But when some fearures in the furure will be inplemented this will be the case. So there should come an kind of raw state for the terminal and reversable options to redo all the changes made to the core state when the process ends. More information can be found at this [thread](https://www.reddit.com/r/rust/comments/7tg6n2/looking_for_feedback_onmy_cross_platform_terminal/dtf4ilo/)

- Handling mouse events 
- Handling key events
- Tests

## Contributing

If you would like to contribute to crossterm, than please design the code as it is now. Each module contains the same structures so we can easely extend to multible platforms. As you study the code you will quiqly see what the architecture is. Maybe later there will be an documentation for how crossterm is design.

## Versioning

The current version is crossterm 0.1, every commit I merge the version go's up like 0.1.0 -> 0.1.1 -> 0.1.2.

When new features arrives the packages will go up like 0.1 -> 0.2 -> 0.3

## Authors

* **Timon Post** - *Project Owner & creator*

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details



