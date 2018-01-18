//! With this module you can perform actions that are cursor related.
//! Like changing and displaying the position of the cursor in terminal.

use std::fmt::Display;

use Construct;
use super::base_cursor::ITerminalCursor;

#[cfg(unix)]
use super::AnsiCursor;
#[cfg(windows)]
use super::WinApiCursor;

/// Struct that stores an specific platform implementation for cursor related actions.
pub struct TerminalCursor {
    terminal_cursor: Option<Box<ITerminalCursor>>,
}

impl TerminalCursor {
    /// Instantiates an platform specific cursor implementation whereon cursor related actions can be performed.
    pub fn init(&mut self) {
        if let None = self.terminal_cursor {
            self.terminal_cursor = get_cursor_options();
        }
    }

    /// Goto some position (x,y) in the terminal.
    ///
    /// #Example
    ///
    /// ```rust
    ///
    /// extern crate crossterm;
    ///
    /// use self::crossterm::crossterm_cursor;
    ///     
    /// crossterm_cursor::get().goto(10,10);
    /// 
    /// ```
    pub fn goto(&mut self, x: u16, y: u16) -> &mut TerminalCursor {
        &self.init();
        if let Some(ref terminal_cursor) = self.terminal_cursor {
            terminal_cursor.goto(x, y);
        }
        self
    }

    /// Get current cursor position (x,y) in the terminal.
    ///
    /// #Example
    ///
    /// ```rust
    ///
    /// extern crate crossterm;
    ///
    /// use self::crossterm::crossterm_cursor;
    ///          
    /// let pos = crossterm_cursor::get().pos();
    /// println!("{:?}", pos);
    /// 
    /// ```
    pub fn pos(&mut self) -> (i16, i16) {
        &self.init();
        if let Some(ref terminal_cursor) = self.terminal_cursor {
            terminal_cursor.pos()
        } else {
            (0, 0)
        }
    }

    /// Move the current cursor position `n` times up.
    ///
    /// #Example
    ///
    /// ```rust
    /// 
    /// extern crate crossterm;
    ///
    /// use self::crossterm::crossterm_cursor;
    ///      
    /// // Move 1 time up
    /// crossterm_cursor::get().move_up(1);
    /// 
    /// // Move 2 times up
    /// crossterm_cursor::get().move_up(2);   
    /// 
    /// ```
    pub fn move_up(&mut self, count: u16) -> &mut TerminalCursor {
        &self.init();
        if let Some(ref terminal_cursor) = self.terminal_cursor {
            terminal_cursor.move_up(count);
        }
        self
    }

    /// Move the current cursor position `n` times right.
    ///
    /// #Example
    ///
    /// ```rust
    ///
    /// extern crate crossterm;
    ///
    /// use self::crossterm::crossterm_cursor;
    ///
    ///
    /// // move 1 time right
    /// crossterm_cursor::get().move_right(1);
    /// 
    /// // move 2 times right
    /// crossterm_cursor::get().move_right(2);
    /// 
    /// ```
    pub fn move_right(&mut self, count: u16) -> &mut TerminalCursor {
        &self.init();
        if let Some(ref terminal_cursor) = self.terminal_cursor {
            terminal_cursor.move_right(count);
        }
        self
    }

    /// Move the current cursor position `n` times down.
    ///
    /// #Example
    ///
    /// ```rust
    ///
    /// extern crate crossterm;
    ///
    /// use self::crossterm::crossterm_cursor;
    /// 
    /// // move 1 time down 
    /// crossterm_cursor::get().move_down(1);
    /// 
    /// // move 2 times down
    /// crossterm_cursor::get().move_down(2);
    ///
    /// ```
    pub fn move_down(&mut self, count: u16) -> &mut TerminalCursor {
        &self.init();
        if let Some(ref terminal_cursor) = self.terminal_cursor {
            terminal_cursor.move_down(count);
        }
        self
    }

    /// Move the current cursor position `n` times left.
    ///
    /// #Example
    ///
    /// ```rust
    ///
    /// extern crate crossterm;
    ///
    /// use self::crossterm::crossterm_cursor;
    /// 
    /// // move 1 time left
    /// crossterm_cursor::get().move_left(1);
    /// 
    /// // move 2 time left
    /// crossterm_cursor::get().move_left(2);
    /// 
    /// ```
    pub fn move_left(&mut self, count: u16) -> &mut TerminalCursor {
        &self.init();
        if let Some(ref terminal_cursor) = self.terminal_cursor {
            terminal_cursor.move_left(count);
        }
        self
    }

    /// Print an value at the current cursor position.
    ///
    /// This method prints an value with `print!()` and clears the buffer afterwards. 
    /// Rust's standard output is line-buffered. So your text gets sent to the console one line at a time.
    /// If you set the curosr position and try to `print!()` at that position and do not clear the buffer, than the character will not be printed at that position. 
    /// But will be printed when the next `println()` will be done.
    ///
    /// With this method you can print any displayable value at a certain position and the output buffer will be cleared afterwards.
    ///
    /// For more information see the cursor example in /examples/cursor
    ///
    /// #Example
    ///
    /// ```rust
    ///
    /// extern crate crossterm;
    ///
    /// use self::crossterm::crossterm_cursor;
    /// use std;
    /// use std::io::Write;
    ///
    /// // of course we can just do this.
    /// crossterm_cursor::get().goto(10,10);
    /// print!("@");
    /// std::io::stdout().flush();
    /// 
    /// // but now we can chain the methods so it looks cleaner and it automatically flushes the buffer.  
    /// crossterm_cursor::get()
    /// .goto(10,10)
    /// .print("@");
    /// 
    /// ```
    pub fn print<D: Display>(&mut self, value: D) -> &mut TerminalCursor {
        print!("{}", value);
        use std;
        use std::io::Write;
        // rust is line buffered so we need to flush the buffer in order to print it at the current cursor position. 
        std::io::stdout().flush();
        self
    }
}

/// Get the concrete ITerminalCursor implementation based on the current operating system.
fn get_cursor_options() -> Option<Box<ITerminalCursor>> {
    #[cfg(unix)]
    return Some(AnsiCursor::new());
        
    #[cfg(windows)]
    return Some(WinApiCursor::new());
}

/// Get an TerminalCursor implementation whereon cursor related actions can be performed.
///
/// Check `/examples/cursor` in the libary for more spesific examples.
/// 
/// #Example
///
/// ```rust
///
/// extern crate crossterm;
///
/// use self::crossterm::crossterm_cursor;
/// 
/// // Get cursor and goto pos X: 5, Y: 10
/// let mut cursor = crossterm_cursor::get();
/// cursor.goto(5,10);
///     
/// //Or you can do it in one line.
/// crossterm_cursor::get().goto(5,10);
///
/// ```
pub fn get() -> Box<TerminalCursor> {
    Box::from(TerminalCursor {
        terminal_cursor: get_cursor_options(),
    })
}
