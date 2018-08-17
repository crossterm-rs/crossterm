//! With this module you can perform actions that are color related.
//! Like styling the font, foreground color and background.

use super::*;
use std::io;

/// Struct that stores an specific platform implementation for color related actions.
///
/// Check `/examples/color` in the library for more specific examples.
///
/// ```rust
/// use crossterm::{Screen}
/// use crossterm::color::color;
///
/// let screen = Screen::default();
/// let colored_terminal = color(&screen);
///
/// // set foreground color
/// colored_terminal.set_fg(Color::Red);
/// // set background color
/// colored_terminal.set_bg(Color::Red);
/// // reset color to default
/// colored_terminal.reset();
/// ```

pub type TerminalColor = Box<IterminalColor + Send + Sync>;

pub fn color() -> TerminalColor {
    #[cfg(target_os = "windows")]
    let color = functions::get_module::<Box<ITerminalColor>>(
        Box::from(WinApiColor::new()),
        Box::from(AnsiColor::new()),
    ).unwrap();

    #[cfg(not(target_os = "windows"))]
    let color = Box::from(AnsiColor::new()) as Box<ITerminalColor>;
    color
}

/// Get available color count.
pub fn get_available_color_count() -> io::Result<u16> {
    use std::env;

    Ok(match env::var_os("TERM") {
        Some(val) => {
            if val.to_str().unwrap_or("").contains("256color") {
                256
            } else {
                8
            }
        }
        None => 8,
    })
}
