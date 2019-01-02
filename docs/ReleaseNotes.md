# Changes crossterm 0.6.0
- WinApi rewrite and correctly error handled [PR 67](https://github.com/TimonPost/crossterm/pull/67)
- Windows attribute support [PR 62](https://github.com/TimonPost/crossterm/pull/62)
- Readline bug fix windows systems [PR 62](https://github.com/TimonPost/crossterm/pull/62)
- Error handling improvement.
- General refactoring, all warnings removed.
- Documentation improvement.

# Changes crossterm 0.5.1
- Documentation refactor.
- Fixed broken API documentation [PR 53](https://github.com/TimonPost/crossterm/pull/53).

# Changes crossterm 0.5.0
- Added ability to pause the terminal [issue](https://github.com/TimonPost/crossterm/issues/39)
- RGB support for Windows 10 systems
- ANSI color value (255) color support
- More convenient API, no need to care about `Screen` unless working with when working with alternate or raw screen [PR](https://github.com/TimonPost/crossterm/pull/44)
- Implemented Display for styled object

# Changes crossterm to 0.4.3
- Fixed bug [issue 41](https://github.com/TimonPost/crossterm/issues/41)

# Changes crossterm to 0.4.2
- Added functionality to make a styled object writable to screen [issue 33](https://github.com/TimonPost/crossterm/issues/33)
- Added unit tests.
- Bugfix with getting terminal size unix.
- Bugfix with returning written bytes [pull request 31](https://github.com/TimonPost/crossterm/pull/31)
- removed methods calls: `as_any()` and `as_any_mut()` from `TerminalOutput`

# Bug fix crossterm to 0.4.1
- Fixed resizing of ansi terminal with and height where in the wrong order.

# Features / Fixes in crossterm 0.4.0
- Input support (read_line, read_char, read_async, read_until_async)
- Styling module improved
- Everything is multithreaded (`Send`, `Sync`)
- Performance enhancements: removed mutexes, removed state manager, removed context type removed unnecessarily RC types.
- Bug fix resetting console color.
- Bug fix whit undoing raw modes.
- More correct error handling.
- Overall commend improvement.
- Overall refactor of code.

# Features in crossterm 0.3.0

This version has some braking changes check [upgrade manual](UPGRADE%20Manual.md) for more information about what is changed. 
I think you should not switch to version `0.3.0` if you aren't going to use the AlternateScreen feature.
Because you will have some work to get to the new version of crossterm depending on your situation. 

Some Features crossterm 0.3.0
- Alternate Screen for windows and unix systems.
- Raw screen for unix and windows systems [Issue 5](https://github.com/TimonPost/crossterm/issues/5)..
- Hiding an showing the cursor.
- Control over blinking of the terminal cursor (only some terminals are supporting this).
- The terminal state will be set to its original state when process ends [issue7](https://github.com/TimonPost/crossterm/issues/7).
- exit the current process.

## Alternate screen
This create supports alternate screen for both windows and unix systems. You can use 

*Nix style applications often utilize an alternate screen buffer, so that they can modify the entire contents of the buffer, without affecting the application that started them.
The alternate buffer is exactly the dimensions of the window, without any scrollback region.
For an example of this behavior, consider when vim is launched from bash.
Vim uses the entirety of the screen to edit the file, then returning to bash leaves the original buffer unchanged.

I Highly recommend you to check the `examples/Crossterm 0.3.0/program_examples/first_depth_search` for seeing this in action. 

## Raw screen
This crate now supports raw screen for both windows and unix systems. 
What exactly is raw state:
- No line buffering.
   Normally the terminals uses line buffering. This means that the input will be send to the terminal line by line.
   With raw mode the input will be send one byte at a time.
- Input
  All input has to be written manually by the programmer.
- Characters
  The characters are not processed by the terminal driver, but are sent straight through.
  Special character have no meaning, like backspace will not be interpret as backspace but instead will be directly send to the terminal.
With these modes you can easier design the terminal screen.

## Some functionalities added 
- Hiding and showing terminal cursor
- Enable or disabling blinking of the cursor for unix systems (this is not widely supported)
- Restoring the terminal to original modes.
- Added a [wrapper](https://github.com/TimonPost/crossterm/blob/master/src/shared/crossterm.rs) for managing all the functionalities of crossterm `Crossterm`.
- Exit the current running process

## Examples
Added [examples](https://github.com/TimonPost/crossterm/tree/master/examples) for each version of the crossterm version. 
Also added a folder with some [real life examples](https://github.com/TimonPost/crossterm/tree/master/examples/Crossterm%200.3.0/program_examples).

## Context

What is the `Context`  all about? This `Context` has several reasons why it is introduced into `crossterm version 0.3.0`.
These points are related to the features like `Alternatescreen` and managing the terminal state.

- At first `Terminal state`:

    Because this is a terminal manipulating library there will be made changes to terminal when running an process. 
    If you stop the process you want the terminal back in its original state. 
    Therefore, I need to track the changes made to the terminal. 
 
- At second `Handle to the console`

    In Rust we can use `stdout()` to get an handle to the current default console handle. 
    For example when in unix systems you want to print something to the main screen you can use the following code: 

        write!(std::io::stdout(), "{}", "some text").

    But things change when we are in alternate screen modes. 
    We can not simply use `stdout()` to get a handle to the alternate screen, since this call returns the current default console handle (handle to mainscreen).
    
    Because of that we need to store an handle to the current screen. 
    This handle could be used to put into alternate screen modes and back into main screen modes.
    Through this stored handle Crossterm can execute its command and write on and to the current screen whether it be alternate screen or main screen.
    
    For unix systems we store the handle gotten from `stdout()` for windows systems that are not supporting ANSI escape codes we store WinApi `HANDLE` struct witch will provide access to the current screen. 
    
So to recap this `Context` struct is a wrapper for a type that manges terminal state changes. 
When this `Context` goes out of scope all changes made will be undone.
Also is this `Context` is a wrapper for access to the current console screen.

Because Crossterm needs access to the above to types quite often I have chosen to add those two in one struct called `Context` so that this type could be shared throughout library. 
Check this link for more info: [cleanup of rust code](https://stackoverflow.com/questions/48732387/how-can-i-run-clean-up-code-in-a-rust-library). 
More info over writing to alternate screen buffer on windows and unix see this [link](https://github.com/TimonPost/crossterm/issues/17)

__Now the user has to pass an context type to the modules of Crossterm like this:__
      
      let context = Context::new();
      
      let cursor = cursor(&context);
      let terminal = terminal(&context);
      let color = color(&context);
    
Because this looks a little odd I will provide a type withs will manage the `Context` for you. You can call the different modules like the following:

      let crossterm = Crossterm::new();
      let color = crossterm.color();
      let cursor = crossterm.cursor();
      let terminal = crossterm.terminal();
     
      
### Alternate screen
When you want to switch to alternate screen there are a couple of things to keep in mind for it to work correctly. 
First off some code of how to switch to Alternate screen, for more info check the [alternate screen example](https://github.com/TimonPost/crossterm/blob/master/examples/Crossterm%200.3.0/terminal/alternate_screen.rs).

_Create alternate screen from `Context`_

        // create context.
        let context = crossterm::Context::new();
        // create instance of Alternatescreen by the given context, this wil also switch to it.
        let mut screen = crossterm::AlternateScreen::from(context.clone());        
        // write to the alternate screen.
        write!(screen,  "test");
        
_Create alternate screen from `Crossterm`:_

        // create context.
        let crossterm = ::crossterm::Crossterm::new();
        // create instance of Alternatescreen by the given refrence to crossterm, this wil also switch to it.
        let mut screen = crossterm::AlternateScreen::from(&crossterm);        
        // write to the alternate screen.
        write!(screen,  "test");
         
like demonstrated above, to get the functionalities of `cursor(), color(), terminal()` also working on alternate screen.
You need to pass it the same `Context` as you have passed to the previous three called functions,
If you don't use the same `Context` in `cursor(), color(), terminal()` than these modules will be using the main screen and you will not see anything at the alternate screen. If you use the [Crossterm](https://github.com/TimonPost/crossterm/blob/master/src/shared/crossterm.rs) type you can get the `Context` from it by calling the crossterm.get_context() whereafter you can create the AlternateScreen from it. 

# Fixes in crossterm 0.2.2
- Bug see [issue 15](https://github.com/TimonPost/crossterm/issues/15)

# Fixes in crossterm 0.2.1

- Default ANSI escape codes for windows machines, if windows does not support ANSI switch back to WinApi.
- method grammar mistake fixed [Issue 3](https://github.com/TimonPost/crossterm/issues/3)
- Some Refactorings in method names see [issue 4](https://github.com/TimonPost/crossterm/issues/4)
- Removed bin reference from crate [Issue 6](https://github.com/TimonPost/crossterm/issues/6)
- Get position unix fixed [issue 8](https://github.com/TimonPost/crossterm/issues/8)

# Features crossterm 0.2

- 256 color support. 
- Text Attributes like: bold, italic, underscore and crossed word ect. 
- Custom ANSI color code input to set fore- and background color for unix.
- Storing the current cursor position and resetting to that stored cursor position later. 
- Resizing the terminal.
