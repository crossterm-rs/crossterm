What is the `Context`  all about? This `Context` has several reasons why it is introduced into `crossterm version 0.2.3`.
These points are related to the features like `Alternatescreen` and managing the terminal state.

- At first `Terminal state`:

    Because this is a terminal manipulating library there will be made changes to terminal when running an process. 
    If you stop the process you want the terminal back in its original state. 
    Therefore, I need to track the changes made to the terminal. 
 
- At second `Handle to the console`

    In Rust we can call `stdout()` to get an handle to the current default console handle. 
    For example when in unix systems you want to print something to the main screen you can use the following code: 

        write!(std::io::stdout(), "{}", "some text").

    But things change when we are in alternate screen modes. 
    We can not simply use `stdout()` to get a handle to the alternate screen, since this call returns the current default console handle (mainscreen).
    
    Instead we need to store an handle to the screen output. 
    This handle could be used to put into alternate screen modes and back into main screen modes.
    Through this stored handle Crossterm can execute its command on the current screen whether it be alternate screen or main screen.
    
    For unix systems we store the handle gotten from `stdout()` for windows systems that are not supporting ANSI escape codes we store WinApi `HANDLE` struct witch will provide access to the current screen. 
    
So to recap this `Context` struct is a wrapper for a type that manges terminal state changes. 
When this `Context` goes out of scope all changes made will be undone.
Also is this `Context` is a wrapper for access to the current console screen.

Because Crossterm needs access to the above to types quite often I have chosen to add those two in one struct called `Context` so that this type could be shared throughout library. 
Check this link for more info:  [cleanup of the changes](https://stackoverflow.com/questions/48732387/how-can-i-run-clean-up-code-in-a-rust-library).

Now the user has to pass an context type to the modules of Crossterm like this:
      
      let context = Context::new();
      
      let cursor = cursor(&context);
      let terminal = terminal(&context);
      let color = color(&context);
    
Check the documentation of `AlternateScreen` for more info about how to properly manage the `Context` of the terminal. 
If you don't use alternate screen functionalist's please checkout the `Crossterm` documentation whits will make things easier for you.

Because this looks a little odd I will provide a type withs will manage the `Context` for you. You can call the different modules like the following:

      let crossterm = Crossterm::new();
      let color = envoirment.color();
      let cursor = envoirment.cursor();
      let terminal = envoirment.terminal();     