//! A module that contains all the actions related to cursor movement in the terminal.
//! Like: moving the cursor position; saving and resetting the cursor position; hiding showing and control the blinking of the cursor.
//!
//! Note that positions of the cursor are 0 -based witch means that the coordinates (cells) starts counting from 0

use super::*;
use common::error::Result;
use Screen;

/// Struct that stores a platform-specific implementation for cursor related actions.
///
/// Check `/examples/cursor` in the library for more specific examples.
///
/// ```rust
/// extern crate crossterm;
/// use self::crossterm::cursor;
/// use self::crossterm::Screen;
///
/// let mut cursor = cursor();
///
/// // Get cursor and goto pos X: 5, Y: 10
/// cursor.goto(5,10);
///
/// cursor.show();
/// cursor.hide();
/// cursor.blink(true);
/// cursor.move_left(2);
/// ```
///
/// When you want to use 'cursor' on 'alternate screen' use the `Screen` type instead and pass it to the `cursor::from_screen()` function.
/// By doing that cursor actions will be performed on the alternate screen.
pub struct TerminalCursor<'stdout> {
    terminal_cursor: Box<ITerminalCursor + Sync + Send>,
    stdout: Option<&'stdout Arc<TerminalOutput>>,
}

impl<'stdout> TerminalCursor<'stdout> {
    /// Create new `TerminalCursor` instance whereon cursor related actions can be performed.
    pub fn new() -> TerminalCursor<'stdout> {
        #[cfg(target_os = "windows")]
        let cursor = functions::get_module::<Box<ITerminalCursor + Sync + Send>>(
            WinApiCursor::new(),
            AnsiCursor::new(),
        )
        .unwrap();

        #[cfg(not(target_os = "windows"))]
        let cursor = AnsiCursor::new() as Box<ITerminalCursor + Sync + Send>;

        TerminalCursor {
            terminal_cursor: cursor,
            stdout: None,
        }
    }

    /// Create a new instance of `TerminalCursor` whereon cursor related actions could be preformed on the given output.
    ///
    /// **Note**
    ///
    /// Use this function when you want your terminal to operate with a specific output.
    /// This could be useful when you have a screen which is in 'alternate mode'.
    /// And you want your actions from the `TerminalCursor`, created by this function, to operate on the 'alternate screen'.
    ///
    /// # Example
    /// ```
    /// let screen = Screen::default();
    //
    /// if let Ok(alternate) = screen.enable_alternate_modes(false) {
    ///    let terminal = TerminalCursor::from_output(&alternate.screen.stdout);
    /// }
    /// ```
    pub fn from_output(stdout: &'stdout Arc<TerminalOutput>) -> TerminalCursor<'stdout> {
        #[cfg(target_os = "windows")]
        let cursor = functions::get_module::<Box<ITerminalCursor + Sync + Send>>(
            WinApiCursor::new(),
            AnsiCursor::new(),
        )
        .unwrap();

        #[cfg(not(target_os = "windows"))]
        let cursor = AnsiCursor::new() as Box<ITerminalCursor + Sync + Send>;

        TerminalCursor {
            terminal_cursor: cursor,
            stdout: Some(stdout),
        }
    }

    /// Goto some position (x,y) in the terminal.
    ///
    /// ```rust
    /// let cursor = cursor();
    ///
    /// // change the cursor to position, x: 4 and y: 5
    /// cursor.goto(4,5);
    ///
    /// ```
    pub fn goto(&self, x: u16, y: u16) -> Result<()> {
        self.terminal_cursor.goto(x, y, &self.stdout)
    }

    /// Get current cursor position (x,y) in the terminal.
    ///
    /// ```rust
    /// let cursor = cursor();
    ///
    /// // get the current cursor pos
    /// let (x,y) = cursor.pos();
    /// ```
    pub fn pos(&self) -> (u16, u16) {
        self.terminal_cursor.pos()
    }

    /// Move the current cursor position `n` times up.
    ///
    /// ```rust
    /// let cursor = cursor();
    ///
    /// // Move the cursor to position 3 times to the up in the terminal
    /// cursor.move_up(3);
    /// ```
    pub fn move_up(&mut self, count: u16) -> &mut TerminalCursor<'stdout> {
        self.terminal_cursor.move_up(count, &self.stdout).unwrap();
        self
    }

    /// Move the current cursor position `n` times right.
    ///
    /// ```rust
    /// let cursor = cursor();
    ///
    /// // Move the cursor to position 3 times to the right in the terminal
    /// cursor.move_right(3);
    /// ```
    pub fn move_right(&mut self, count: u16) -> &mut TerminalCursor<'stdout> {
        self.terminal_cursor.move_right(count, &self.stdout).unwrap();
        self
    }

    /// Move the current cursor position `n` times down.
    ///
    /// ```rust
    /// let cursor = cursor();
    ///
    /// // Move the cursor to position 3 times to the down in the terminal
    /// cursor.move_down(3);
    /// ```
    pub fn move_down(&mut self, count: u16) -> &mut TerminalCursor<'stdout> {
        self.terminal_cursor.move_down(count, &self.stdout).unwrap();
        self
    }

    /// Move the current cursor position `n` times left.
    ///
    /// ```rust
    /// let cursor = cursor();
    ///
    ///  // Move the cursor to position 3 times to the left in the terminal
    ///  cursor.move_left(3);
    /// ```
    pub fn move_left(&mut self, count: u16) -> &mut TerminalCursor<'stdout> {
        self.terminal_cursor.move_left(count, &self.stdout).unwrap();
        self
    }

    /// Save cursor position for recall later.
    ///
    /// Note that this position is stored program based not per instance of the `Cursor` struct.
    ///
    /// ```rust
    /// let cursor = cursor();
    ///
    /// cursor.safe_position();
    /// ```
    pub fn save_position(&self) -> Result<()> {
        self.terminal_cursor.save_position(&self.stdout)
    }

    /// Return to saved cursor position
    ///
    /// Note that this method reset to the position set by `save_position()` and that this position is stored program based not per instance of the `Cursor` struct.
    ///
    /// ```rust
    /// let cursor = cursor();
    ///
    /// cursor.reset_position();
    /// ```
    pub fn reset_position(&self) -> Result<()> {
        self.terminal_cursor.reset_position(&self.stdout)
    }

    /// Hide de cursor in the console.
    ///
    /// ```rust
    /// let cursor = cursor();
    /// cursor.hide();
    /// ```
    pub fn hide(&self) -> Result<()> {
        self.terminal_cursor.hide(&self.stdout)
    }

    /// Show the cursor in the console.
    ///
    /// ```rust
    ///
    /// let cursor = cursor();
    /// cursor.show();
    ///
    /// ```
    pub fn show(&self) -> Result<()> {
        self.terminal_cursor.show(&self.stdout)
    }

    /// Enable or disable blinking of the terminal.
    ///
    /// Not all terminals are supporting this functionality. Windows versions lower than windows 10 also are not supporting this version.
    ///
    /// ```rust
    /// let cursor = cursor();
    /// cursor.blink(true);
    /// cursor.blink(false);
    /// ```
    pub fn blink(&self, blink: bool) -> Result<()> {
        self.terminal_cursor.blink(blink, &self.stdout)
    }
}

/// Get a `TerminalCursor` instance whereon cursor related actions can be performed.
pub fn cursor() -> TerminalCursor<'static> {
    TerminalCursor::new()
}

/// Get a `TerminalCursor` instance whereon cursor related actions can be performed.
/// Pass the reference to any `Screen` you want this type to perform actions on.
pub fn from_screen(screen: &Screen) -> TerminalCursor {
    TerminalCursor::from_output(&screen.stdout)
}
