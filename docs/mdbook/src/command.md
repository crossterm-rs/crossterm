# Command API
The command api makes the use of crossterm much easier and offers more control over when and how a command such as moving the cursor is executed.

The command API offers:
- Better Performance
- Complete control over when to flush
- Complete control over where the ANSI escape commands are executed to
- Way easier and nicer API

There are two ways to use the API command:

- [By using functions]()

    The functions are able to execute commands on types that implement `Write`. 
    Functions are easier to use and debug. There is a disadvantage, and that is that there is a lot of boilerplate code involved. 
- [By using macros]()

    Macros are generally seen as more difficult, but offer an API with less boiler plate code. 
    If you are not afraid of macros, this is definitely a recommendation.
    
## Commands
Crossterm provides the folowing commands that can be used to perfom actions with:

_cursor commands_
- Goto (x, y)
- UP (number of time)
- Down (number of time)
- Left (number of time)
- Right (number of time)
- SavePos
- ResetPos
- Hide
- Show 
- Blink On
- Blink Off
    
_style commands_
- SetFg (Color)
- SetBg (Color)
- SetAttr (attr)
- Print Styled Text (text)

_terminal command_
- Clear (ClearType)
- Scroll Up (number of time)
- Scroll Down (number of time)
- SetSize (width, height)

_other_
- Output (text)

Each crossterm crate provides its own command when using crossterm you are able to use them all at once. 
When using a single crate or a feature flag, you are able to only use certain command.
 
## Performance
Before crossterm 10.0 was released, crossterm had some performance issues. It did a `flush` after each command (cursor movement). 
A `flush` is a heavy action on the terminal, and if it is done more often the performance will go down quickly.

Linux and Windows 10 systems support ANSI escape codes. 
Those ANSI escape codes are strings or rather a byte sequence.
When we `write` and `flush` those to the terminal we can perform some action. 

### Lazy Execution
Because `flush` is a heavy system call we can instead `write` the commands to the `stdout` without flushing. 
When can do a `flush` we do want to execute the commands.

If you create a terminal editor or TUI, it is wise to use this option. 
For example, you can write commands to the terminal `stdout` and flush the `stdout` at every frame. 
By doing this you can make efficient use of the terrminal buffer and get better performance because you are not calling `flush` after every command. 

 #### Examples
 _functions_
 ```rust
let mut stdout = stdout();

stdout = stdout.queue(Goto(5,5));

// some other code ...

stdout.flush();
 ```
 
 The `queue` function returns it self, therefore you are able to use this to queue another command. 
 Like `stdout.queue(Goto(5,5)).queue(Clear(ClearType::All))`
 
 _macro's_
 ```rust  
let mut stdout = stdout();

queue!(stdout,  Goto(5, 5));

// some other code ...

// flush when you want to execute the 'queued' commands
stdout.flush();
 ```
 
You can pass more than one command into the macro like: `queue!(stdout,  Goto(5, 5), Clear(ClearType::All));`
 
### Direct Execution

If you want to execute commands directly, this is also possible. You don't have to flush the 'stdout', as described above. 
This is fine if you are not executing lot's of commands. 

_functions_
 ```rust 
stdout().execute(Goto(5,5));
 ```
 The `execute` function returns it self, therefore you are able to use this to execute another command 
 like `stdout.execute(Goto(5,5)).execute(Clear(ClearType::All))`
 
_macro's_
```rust
execute!(stdout,  Goto(5, 5));
```

 You can pass more than one command into the macro like: `queue!(stdout,  Goto(5, 5), Clear(ClearType::All));`
 
 ## Short Examples
 
 Print a rectangle colored with magenta and use both direct execution and lazy execution.
 
 _rectangle with command functions_
 ```rust 
use crossterm::{Color, PrintStyledFont, Colorize};
use std::io::stdout();

let mut stdout = stdout();

stdout = stdout.execute(Clear(ClearType::All));

for y in 0..40 {
    for x in 0..150 {
         if (y == 0 || y == 40 - 1) || (x == 0 || x == 150 - 1) {
             stdout = stdout
                 .queue(Goto(x,y))
                 .queue(PrintStyledFont( "█".magenta()));
         }
    }
    stdout.flush();
}
 ```
 
 _rectangle with macro functions_
 ```rust
use crossterm::{Color, PrintStyledFont, Colorize};
use std::io::stdout();

let mut stdout = stdout();

execute!(stdout, Clear(ClearType::All));

for y in 0..40 {
    for x in 0..150 {
         if (y == 0 || y == 40 - 1) || (x == 0 || x == 150 - 1) {
             queue!(stdout, Goto(x,y), PrintStyledFont( "█".magenta()));
         }
    }
    stdout.flush();
} 
 ```