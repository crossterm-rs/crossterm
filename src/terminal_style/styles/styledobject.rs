use std;
use std::fmt;
use std::io::Write;

use terminal_style::{Color, ObjectStyle};

/// Struct that contains both the style and the content wits is styled.
pub struct StyledObject<D> {
    pub object_style: ObjectStyle,
    pub content: D,
}

impl<D> StyledObject<D> {
    /// Paints the foreground color with the passed `Color`
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate crossterm;
    ///
    /// use self::crossterm::terminal_style::{paint,Color};
    ///
    /// fn main()
    /// {
    ///    // create an styled object with the foreground color red.
    ///    let styledobject = paint("I am colored red").with(Color::Red);
    ///    // create an styled object with the foreground color blue.
    ///    let styledobject1 = paint("I am colored blue").with(Color::Blue);
    ///
    ///    // print the styled objects
    ///    println!("{}", styledobject);
    ///    println!("{}", styledobject1);
    ///    // or print an styled object directly.
    ///    println!("{}", paint("I am colored green").with(Color::Green))
    /// }
    /// ```
    pub fn with(mut self, foreground_color: Color) -> StyledObject<D> {
        self.object_style = self.object_style.fg(foreground_color);
        self
    }

    /// Paints the background color with the passed `Color`
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate crossterm;
    ///
    /// use self::crossterm::terminal_style::{paint,Color};
    ///
    /// fn main()
    /// {
    ///    // create an styled object with the background color red.
    ///    let styledobject = paint("I am colored red").on(Color::Red);
    ///    // create an styled object with the background color blue.
    ///    let styledobject1 = paint("I am colored blue").on(Color::Blue);
    ///
    ///    // print the styled objects
    ///    println!("{}", styledobject);
    ///    println!("{}", styledobject1);
    ///    // or print an styled object directly.
    ///    println!("{}", paint("I am colored green").on(Color::Green))
    /// }
    /// ```
    pub fn on(mut self, background_color: Color) -> StyledObject<D> {
        self.object_style = self.object_style.bg(background_color);
        self
    }
}

/// This is used to make StyledObject able to be displayed.
/// This macro will set the styled stored in Styled Object

macro_rules! impl_fmt
{
    ($name:ident) => {
        impl<D: fmt::$name> fmt::$name for StyledObject<D> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
            {
                let mut colored_terminal = super::super::colored_terminal();
                let mut reset = true;

                if let Some(bg) = self.object_style.bg_color
                {
                    colored_terminal.set_bg(bg);
                    reset = true;
                }
                if let Some(fg) = self.object_style.fg_color
                {
                   colored_terminal.set_fg(fg);
                   reset = true;
                }
                fmt::$name::fmt(&self.content, f)?;
                std::io::stdout().flush().expect("Flush stdout failed");

                if reset
                {
                    colored_terminal.reset();
                }

                Ok(())
            }
        }
    }
}

/// This inplements Display for StyledObject
/// Notice that more implementations can be maked.
/// # Example
/// ```rust
/// example impl_fmt!(Debug);
/// ```
impl_fmt!(Debug);
impl_fmt!(Display);
