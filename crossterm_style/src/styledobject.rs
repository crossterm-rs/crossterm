//! This module contains the logic to style an object that contains some 'content' which can be styled.

use super::{color, Color, ObjectStyle};
//use Screen;
use crossterm_utils::Result;
use std::fmt::{self, Display, Formatter};
use std::io::Write;
use std::result;

use super::Attribute;
use crate::Colorize;
use crate::Styler;

/// Struct that contains both the style and the content wits can be styled.
pub struct StyledObject<D: Display> {
    pub object_style: ObjectStyle,
    pub content: D,
}

impl<'a, D: Display + 'a> StyledObject<D> {
    /// Set the foreground of the styled object to the passed `Color`.
    ///
    /// # Remarks
    ///
    /// This methods consumes 'self', and works like a builder.
    /// By having this functionality you can do: `with().on().attr()`
    pub fn with(mut self, foreground_color: Color) -> StyledObject<D> {
        self.object_style = self.object_style.fg(foreground_color);
        self
    }

    /// Set the background of the styled object to the passed `Color`.
    ///
    /// # Remarks
    ///
    /// This methods consumes 'self', and works like a builder.
    /// By having this functionality you can do: `with().on().attr()`
    pub fn on(mut self, background_color: Color) -> StyledObject<D> {
        self.object_style = self.object_style.bg(background_color);
        self
    }

    /// Set the attribute of an styled object to the passed `Attribute`.
    ///
    /// # Remarks
    ///
    /// This methods consumes 'self', and works like a builder.
    /// By having this functionality you can do: `with().on().attr()`
    pub fn attr(mut self, attr: Attribute) -> StyledObject<D> {
        self.object_style.add_attr(attr);
        self
    }
}

impl<D: Display> Display for StyledObject<D> {
    fn fmt(&self, f: &mut Formatter) -> result::Result<(), fmt::Error> {
        let colored_terminal = color();
        let mut reset = true;

        if let Some(bg) = self.object_style.bg_color {
            colored_terminal.set_bg(bg).unwrap();
            reset = true;
        }
        if let Some(fg) = self.object_style.fg_color {
            colored_terminal.set_fg(fg).unwrap();
            reset = true;
        }

        for attr in self.object_style.attrs.iter() {
            write!(f, "{}", format!(csi!("{}m"), *attr as i16))?;
            reset = true;
        }

        fmt::Display::fmt(&self.content, f)?;
        std::io::stdout().flush().unwrap();

        if reset {
            colored_terminal.reset().unwrap();
            std::io::stdout().flush().unwrap();
        }

        Ok(())
    }
}

impl<D: Display> Colorize<D> for StyledObject<D> {
    // foreground colors
    def_color!(fg_color: black => Color::Black);
    def_color!(fg_color: red => Color::Red);
    def_color!(fg_color: dark_red => Color::DarkRed);
    def_color!(fg_color: green => Color::Green);
    def_color!(fg_color: dark_green => Color::DarkGreen);
    def_color!(fg_color: yellow => Color::Yellow);
    def_color!(fg_color: dark_yellow => Color::DarkYellow);
    def_color!(fg_color: blue => Color::Blue);
    def_color!(fg_color: dark_blue => Color::DarkBlue);
    def_color!(fg_color: magenta => Color::Magenta);
    def_color!(fg_color: dark_magenta => Color::DarkMagenta);
    def_color!(fg_color: cyan => Color::Cyan);
    def_color!(fg_color: dark_cyan => Color::DarkCyan);
    def_color!(fg_color: white => Color::White);
    def_color!(fg_color: grey => Color::Grey);

    // background colors
    def_color!(bg_color: on_black => Color::Black);
    def_color!(bg_color: on_red => Color::Red);
    def_color!(bg_color: on_dark_red => Color::DarkRed);
    def_color!(bg_color: on_green => Color::Green);
    def_color!(bg_color: on_dark_green => Color::DarkGreen);
    def_color!(bg_color: on_yellow => Color::Yellow);
    def_color!(bg_color: on_dark_yellow => Color::DarkYellow);
    def_color!(bg_color: on_blue => Color::Blue);
    def_color!(bg_color: on_dark_blue => Color::DarkBlue);
    def_color!(bg_color: on_magenta => Color::Magenta);
    def_color!(bg_color: on_dark_magenta => Color::DarkMagenta);
    def_color!(bg_color: on_cyan => Color::Cyan);
    def_color!(bg_color: on_dark_cyan => Color::DarkCyan);
    def_color!(bg_color: on_white => Color::White);
    def_color!(bg_color: on_grey => Color::Grey);
}

impl<D: Display> Styler<D> for StyledObject<D> {
    def_attr!(reset => Attribute::Reset);
    def_attr!(bold => Attribute::Bold);
    def_attr!(underlined => Attribute::Underlined);
    def_attr!(reverse => Attribute::Reverse);
    def_attr!(dim => Attribute::Dim);
    def_attr!(italic => Attribute::Italic);
    def_attr!(negative => Attribute::Reverse);
    def_attr!(slow_blink => Attribute::SlowBlink);
    def_attr!(rapid_blink => Attribute::RapidBlink);
    def_attr!(hidden => Attribute::Hidden);
    def_attr!(crossed_out => Attribute::CrossedOut);
}
