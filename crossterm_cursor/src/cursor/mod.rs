//! A module that contains all the actions related to cursor movement in the terminal.
//! Like: moving the cursor position; saving and resetting the cursor position; hiding showing and control the blinking of the cursor.
//!
//! Note that positions of the cursor are 0 -based witch means that the coordinates (cells) starts counting from 0

mod cursor;

#[cfg(test)]
mod test;

mod ansi_cursor;
#[cfg(windows)]
mod winapi_cursor;

use self::ansi_cursor::AnsiCursor;
#[cfg(windows)]
use self::winapi_cursor::WinApiCursor;

pub use self::cursor::{cursor, TerminalCursor};

use crossterm_utils::{Command, Result};

///! This trait defines the actions that can be performed with the terminal cursor.
///! This trait can be implemented so that a concrete implementation of the ITerminalCursor can fulfill
///! the wishes to work on a specific platform.
///!
///! ## For example:
///!
///! This trait is implemented for `WinApi` (Windows specific) and `ANSI` (Unix specific),
///! so that cursor related actions can be performed on both UNIX and Windows systems.
trait ITerminalCursor: Sync + Send {
    /// Goto some location (x,y) in the context.
    fn goto(&self, x: u16, y: u16) -> Result<()>;
    /// Get the location (x,y) of the current cursor in the context
    fn pos(&self) -> (u16, u16);
    /// Move cursor n times up
    fn move_up(&self, count: u16) -> Result<()>;
    /// Move the cursor `n` times to the right.
    fn move_right(&self, count: u16) -> Result<()>;
    /// Move the cursor `n` times down.
    fn move_down(&self, count: u16) -> Result<()>;
    /// Move the cursor `n` times left.
    fn move_left(&self, count: u16) -> Result<()>;
    /// Save cursor position so that its saved position can be recalled later. Note that this position is stored program based not per instance of the cursor struct.
    fn save_position(&self) -> Result<()>;
    /// Return to saved cursor position
    fn reset_position(&self) -> Result<()>;
    /// Hide the terminal cursor.
    fn hide(&self) -> Result<()>;
    /// Show the terminal cursor
    fn show(&self) -> Result<()>;
    /// Enable or disable the blinking of the cursor.
    fn blink(&self, blink: bool) -> Result<()>;
}

pub struct Goto(pub u16, pub u16);

impl Command for Goto {
    type AnsiType = String;

    fn get_ansi_code(&self)  -> Self::AnsiType {
        ansi_cursor::get_goto_ansi(self.0, self.1)
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        WinApiCursor::new().goto(self.0, self.1)
    }
}

pub struct UP(pub u16);

impl Command for UP {
    type AnsiType = String;

    fn get_ansi_code(&self) -> Self::AnsiType {
        ansi_cursor::get_move_up_ansi(self.0)
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        WinApiCursor::new().move_up(self.0)
    }
}

pub struct Down(pub u16);

impl Command for Down {
    type AnsiType = String;

    fn get_ansi_code(&self) -> Self::AnsiType {
        ansi_cursor::get_move_down_ansi(self.0)
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        WinApiCursor::new().move_down(self.0)
    }
}

pub struct Left(pub u16);

impl Command for Left {
    type AnsiType = String;

    fn get_ansi_code(&self) -> Self::AnsiType {
        ansi_cursor::get_move_left_ansi(self.0)
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        WinApiCursor::new().move_left(self.0)
    }
}

pub struct Right(pub u16);

impl Command for Right {
    type AnsiType = String;

    fn get_ansi_code(&self) -> Self::AnsiType {
        ansi_cursor::get_move_right_ansi(self.0)
    }

    fn execute_ansi(&self) -> Result<()> {
        write_cout!(self.get_ansi_code())
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        WinApiCursor::new().move_right(self.0)
    }
}

pub struct SavePos;

impl Command for SavePos {
    type AnsiType = &'static str;

    fn get_ansi_code(&self) -> Self::AnsiType {
       ansi_cursor::SAFE_POS_ANSI
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        WinApiCursor::new().save_position()
    }
}

pub struct ResetPos;

impl Command for ResetPos {
    type AnsiType = &'static str;

    fn get_ansi_code(&self) -> Self::AnsiType {
        ansi_cursor::RESET_POS_ANSI
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        WinApiCursor::new().reset_position()
    }
}

pub struct Hide;

impl Command for Hide {
    type AnsiType = &'static str;

    fn get_ansi_code(&self) -> Self::AnsiType {
        ansi_cursor::HIDE_ANSI
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        WinApiCursor::new().hide()
    }
}

pub struct Show;

impl Command for Show {
    type AnsiType = &'static str;

    fn get_ansi_code(&self) -> Self::AnsiType {
        ansi_cursor::SHOW_ANSI
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        WinApiCursor::new().show()
    }
}

pub struct BlinkOn;

impl Command for BlinkOn {
    type AnsiType = &'static str;

    fn get_ansi_code(&self) -> Self::AnsiType {
        ansi_cursor::BLINK_ON_ANSI
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        Ok(())
    }
}

pub struct BlinkOf;

impl Command for BlinkOf {
    type AnsiType = &'static str;

    fn get_ansi_code(&self) -> Self::AnsiType {
        ansi_cursor::BLINK_OFF_ANSI
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        Ok(())
    }
}


