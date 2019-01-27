//! A module that contains all the actions related to cursor movement in the terminal.
//! Like: moving the cursor position; saving and resetting the cursor position; hiding showing and control the blinking of the cursor.

use super::*;
use std::sync::Arc;

use crossterm_utils::{Result, TerminalOutput};

#[cfg(windows)]
use crossterm_utils::get_module;

/// Allows you to preform actions with the terminal cursor.
///
/// # Features:
///
/// - Moving n times Up, Down, Left, Right
/// - Goto a certain position
/// - Get cursor position
/// - Storing the current cursor position and resetting to that stored cursor position later
/// - Hiding an showing the cursor
/// - Control over blinking of the terminal cursor (only some terminals are supporting this)
///
/// Note that positions of the cursor are 0 -based witch means that the coordinates (cells) starts counting from 0
///
/// Check `/examples/cursor` in the library for more specific examples.
///
/// # Remarks
///
/// When you want to use 'cursor' on 'alternate screen' use the 'crossterm_screen' crate.
pub struct TerminalCursor<'stdout> {
    terminal_cursor: Box<ITerminalCursor + Sync + Send>,
    stdout: Option<&'stdout Arc<TerminalOutput>>,
}

impl<'stdout> TerminalCursor<'stdout> {
    /// Create new `TerminalCursor` instance whereon cursor related actions can be performed.
    pub fn new() -> TerminalCursor<'stdout> {
        #[cfg(target_os = "windows")]
        let cursor = get_module::<Box<ITerminalCursor + Sync + Send>>(
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
    /// # Remarks
    ///
    /// Use this function when you want your terminal to operate with a specific output.
    /// This could be useful when you have a screen which is in 'alternate mode',
    /// and you want your actions from the `TerminalCursor`, created by this function, to operate on the 'alternate screen'.
    ///
    /// You should checkout the 'crossterm_screen' crate for more information about this.
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
        let cursor = get_module::<Box<ITerminalCursor + Sync + Send>>(
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
    /// # Remarks
    /// position is 0-based, which means we start counting at 0.
    pub fn goto(&self, x: u16, y: u16) -> Result<()> {
        self.terminal_cursor.goto(x, y, &self.stdout)
    }

    /// Get current cursor position (x,y) in the terminal.
    ///
    /// # Remarks
    /// position is 0-based, which means we start counting at 0.
    pub fn pos(&self) -> (u16, u16) {
        self.terminal_cursor.pos()
    }

    /// Move the current cursor position `n` times up.
    pub fn move_up(&mut self, count: u16) -> &mut TerminalCursor<'stdout> {
        self.terminal_cursor.move_up(count, &self.stdout).unwrap();
        self
    }

    /// Move the current cursor position `n` times right.
    pub fn move_right(&mut self, count: u16) -> &mut TerminalCursor<'stdout> {
        self.terminal_cursor
            .move_right(count, &self.stdout)
            .unwrap();
        self
    }

    /// Move the current cursor position `n` times down.
    pub fn move_down(&mut self, count: u16) -> &mut TerminalCursor<'stdout> {
        self.terminal_cursor.move_down(count, &self.stdout).unwrap();
        self
    }

    /// Move the current cursor position `n` times left.
    pub fn move_left(&mut self, count: u16) -> &mut TerminalCursor<'stdout> {
        self.terminal_cursor.move_left(count, &self.stdout).unwrap();
        self
    }

    /// Save cursor position for recall later.
    ///
    /// Note that this position is stored program based not per instance of the `Cursor` struct.
    pub fn save_position(&self) -> Result<()> {
        self.terminal_cursor.save_position(&self.stdout)
    }

    /// Return to saved cursor position
    pub fn reset_position(&self) -> Result<()> {
        self.terminal_cursor.reset_position(&self.stdout)
    }

    /// Hide de cursor in the console.
    pub fn hide(&self) -> Result<()> {
        self.terminal_cursor.hide(&self.stdout)
    }

    /// Show the cursor in the console.
    pub fn show(&self) -> Result<()> {
        self.terminal_cursor.show(&self.stdout)
    }

    /// Enable or disable blinking of the terminal.
    ///
    /// # Remarks
    /// Not all terminals are supporting this functionality. Windows versions lower than windows 10 also are not supporting this version.
    pub fn blink(&self, blink: bool) -> Result<()> {
        self.terminal_cursor.blink(blink, &self.stdout)
    }
}

/// Get a `TerminalCursor` instance whereon cursor related actions can be performed.
pub fn cursor() -> TerminalCursor<'static> {
    TerminalCursor::new()
}
