# This is the development branch do not use this in production. This code can be broken and contains code that could not function correctly.

Things where I am working on now:

I have implemented the alternate and raw screen features for Unix systems. Now I am trying to get this also to work for windows with WINAPI. 

In the new version you must provide the Context type to the function calls `cursor(), color(), terminal()`. This type is used by Crossterm for managing the state of the terminal and for futures like `AlternateScreen` and `Rawscreen`. 

Like described above the next version will have api braking changes. Why I needed to do that is essential for the functioning of these above features. 

- At first `Terminal state`:

    Because this is a terminal manipulating library there will be made changes to terminal when running an process with my library. If you stop the process you want the terminal back in its original state. Therefore, I need to track the changes made to the terminal. This is done in the `Context` struct so that they can be undone in the end.


- At second `Handle to the console`

    In rust we can call `stdout()` to get an handle to the current default console handle. For example when in unix sytems you want to execute some ANSI escape code you have to write it to terminal. I can write it to stdout (screen ouput) withs is the main screen. 

        //like
        write!(std::io::stdout(), "{}", "some ANSI code".

    But things change when we are in alternate screen. If I execute the code above the ANSI escape code will be written to the main handle and not or alternate handle. This causes things to be written to the main screen and not the alternate screen, and this is not wat we want.

To solve the problem, we need to have one place to store the handle to the console screen. So that we can write to this handle during the lifetime of the program. This handle is stored in a subtype of the Context type. 

The user must create an `Context` type for this library.

      //like
      let context = Context::new();
      
      let cursor = cursor(&context);
      let terminal = terminal(&context);
      let color = color(&context);
      
Now that we have on global `Context` type which can be used to register terminal state changes, and in with we can manage the terminal stdout (screen output). When this `Context` disposes we run code to clean up the changes that are made.

Maybe I am going to make a wrapper for the function calls `cursor, terminal, colour` so that when can avoid passing the context all over the place which makes to code more unreadable to my opinion. I really did not want to make API braking changes, bur for the sake of the futures I want to implement it needed to be done.

      // maybe I am going to create some Envoirment type which can be used for getting acces to diffrent modules that this libary provides.
      let envoirment = Envoirment::new();
      envoirment.color();
      envoirment.cursor();
      envoirment.terminal();     


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

## fixes in crossterm 0.2.1

- Default ANSI escape codes for windows machines, if windows does not support ANSI switsh back to WINAPI.
- method grammer mistake fixed [Issue 3](https://github.com/TimonPost/crossterm/issues/3)
- Some Refactorings in method names see [issue 4](https://github.com/TimonPost/crossterm/issues/4)
- Removed bin refrence from crate [Issue 6](https://github.com/TimonPost/crossterm/issues/6)
- The terminal state will be set to its original state when process ends [issue7](https://github.com/TimonPost/crossterm/issues/7).
- Get position unix fixed [issue 8](https://github.com/TimonPost/crossterm/issues/8)

## fixes in crossterm 0.2.2
- Bug see [issue 15](https://github.com/TimonPost/crossterm/issues/15)

## Features crossterm 0.2.3
- Alternate screen for windows and unix systems.
- Rawscreen for unix systems maybe windows.
- Hiding an showing the cursor.
- Control over blinking of the terminal cursor.

## TODO Features crossterm 0.2.2
- Raw state implementation for windows [Issue 5](https://github.com/TimonPost/crossterm/issues/5).
- Alternate screen for windows

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



