//! With this module you can perform actions that are color related.
//! Like styling the font, foreground color and background.

use super::*;
use Construct;
use style::Color;

use std::{ io };
use std::rc::Rc;
use std::sync::Mutex;

/// Struct that stores an specific platform implementation for color related actions.
pub struct TerminalColor {
    color: Option<Box<ITerminalColor>>,
    screen_manager: Rc<Mutex<ScreenManager>>
}

impl TerminalColor {
    /// Create new instance whereon color related actions can be performed.
    pub fn new(screen_manager: Rc<Mutex<ScreenManager>> ) -> TerminalColor {
        #[cfg(target_os = "windows")]
        let color = functions::get_module::<Box<ITerminalColor>>(WinApiColor::new(), AnsiColor::new());

        #[cfg(not(target_os = "windows"))]
        let color = Some(AnsiColor::new() as Box<ITerminalColor>);

        TerminalColor { color: color, screen_manager: screen_manager.clone() }
    }

    /// Set the foreground color to the given color.
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate crossterm;
    ///
    /// use self::crossterm::style::{ color, Color};
    ///
    /// // Get colored terminal instance
    /// let mut colored_terminal = color();
    /// 
    /// // Set foreground color of the font
    /// colored_terminal.set_fg(Color::Red);
    /// // crossterm provides to set the background from &str or String
    /// colored_terminal.set_fg(Color::from("Red"));
    ///
    /// ```
    pub fn set_fg(&mut self, color: Color) {
        if let Some(ref terminal_color) = self.color {
            terminal_color.set_fg(color, self.screen_manager.clone());
        }
    }

    /// Set the background color to the given color.
    ///
    /// #Example
    ///
    /// ```rust
    ///
    /// extern crate crossterm;
    ///
    /// use self::crossterm::style::{ color, Color};
    ///
    /// // Get colored terminal instance
    /// let mut colored_terminal = color();
    /// 
    /// // Set background color of the font
    /// colored_terminal.set_bg(Color::Red);
    /// // crossterm provides to set the background from &str or String
    /// colored_terminal.set_bg(Color::from("Red"));
    ///
    /// ```
    pub fn set_bg(&mut self, color: Color) {
        if let Some(ref terminal_color) = self.color {
            terminal_color.set_bg(color, self.screen_manager.clone());
        }
    }

    /// Reset the terminal colors and attributes to default.
    /// # Example
    ///
    /// ```rust
    /// extern crate crossterm;
    ///
    /// use self::crossterm::style::color;
    ///
    /// // Get colored terminal instance
    /// let mut colored_terminal = color();
    /// 
    /// colored_terminal.reset();
    ///
    /// ```
    pub fn reset(&mut self) {
        if let Some(ref terminal_color) = self.color {
            terminal_color.reset(self.screen_manager.clone());
        }
    }

    /// Get available color count.
    pub fn get_available_color_count(&self) -> io::Result<u16>
    {
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
}

pub fn color(screen_manager: Rc<Mutex<ScreenManager>>) -> Box<TerminalColor> {
    Box::from(TerminalColor::new(screen_manager.clone()))
}
