//! This module provides one place to work with the screen.
//! For example whe can write to the console true this module.

pub mod manager;

#[cfg(target_os = "windows")]
mod win_manager;
mod ansi_manager;

#[cfg(target_os = "windows")]
pub use self::win_manager::WinApiScreenManager;
pub use self::ansi_manager::AnsiScreenManager;

pub use self::manager::{ ScreenManager };
use std::io;
use std::any::Any;

pub trait IScreenManager
{
    /// get the stdout of the screen. This can be used to write to the
//    fn stdout(&mut self) -> &mut Self::Output;
    fn toggle_is_alternate_screen(&mut self, is_alternate_screen: bool);
    /// Write ansi code as String to the current stdout.
    fn write_ansi(&mut self, string: String);
    /// Write a &str to the current stdout.
    fn write_ansi_str(&mut self, string: &str);

    fn write(&mut self, buf: &[u8]) -> io::Result<usize>;

    fn flush(&mut self) -> io::Result<()>;

    fn as_any(&mut self) -> &mut Any;
}