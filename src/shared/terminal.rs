use std::sync::Mutex;
use std::rc::Rc;

use { Context, ScreenManager};

//use super::super::terminal;
//use super::super::cursor;
use super::super::style;

use std::fmt;

pub struct Terminal
{
    pub screen_manager: Rc<Mutex<ScreenManager>>,
    pub context: Mutex<Context>
}

impl Terminal
{
    pub fn new() -> Terminal
    {
        Terminal {
            screen_manager: Rc::new(Mutex::new(ScreenManager::new())),
            context: Mutex::new(Context::new())
        }
    }

    /// Get an TerminalColor implementation whereon color related actions can be performed.
    ///
    /// # Example
    ///
    /// ```rust
    /// extern crate crossterm;
    ///
    /// use self::crossterm::style::{color, Color};
    ///
    /// // Get colored terminal instance
    /// let mut colored_terminal = color();
    ///
    /// // preform some actions on the colored terminal
    /// colored_terminal.set_fg(Color::Red);
    /// colored_terminal.set_bg(Color::Blue);
    /// colored_terminal.reset();
    /// ```
    pub fn color(&self) -> Box<style::TerminalColor> {
        Box::from(style::TerminalColor::new(self.screen_manager.clone()))
    }

    /// Wraps an displayable object so it can be formatted with colors and attributes.
    ///
    /// Check `/examples/color` in the libary for more spesific examples.
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate crossterm;
    ///
    /// use self::crossterm::style::{paint,Color};
    ///
    /// fn main()
    /// {
    ///     // Create an styledobject object from the text 'Unstyled font'
    ///     // Currently it has the default foregroundcolor and backgroundcolor.
    ///     println!("{}",paint("Unstyled font"));
    ///
    ///     // Create an displayable object from the text 'Colored font',
    ///     // Paint this with the `Red` foreground color and `Blue` backgroundcolor.
    ///     // Print the result.
    ///     let styledobject = paint("Colored font").with(Color::Red).on(Color::Blue);
    ///     println!("{}", styledobject);
    ///
    ///     // Or all in one line
    ///     println!("{}", paint("Colored font").with(Color::Red).on(Color::Blue));
    /// }
    /// ```
    pub fn paint<D>(&self, val: D) -> style::StyledObject<D>
        where
            D: fmt::Display,
    {
        style::ObjectStyle::new().apply_to(val, self.screen_manager.clone())
    }
}
