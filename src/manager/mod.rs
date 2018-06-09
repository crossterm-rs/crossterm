pub mod manager;
pub mod terminal;

#[cfg(target_os = "windows")]
mod win_manager;
mod ansi_manager;

#[cfg(target_os = "windows")]
use self::win_manager::WinApiScreenManager;
use self::ansi_manager::AnsiScreenManager;

pub use self::manager::{ ScreenManager };

pub trait IScreenManager<Output>
{
    fn stdout(&mut self) -> &mut Output;
    fn register_output(&mut self, output: Output, is_alternate_screen: bool);
    fn write_ansi(&mut self, string: String);
    fn write_ansi_str(&mut self, string: &str);
}