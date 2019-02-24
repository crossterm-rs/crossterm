//! This module contains the logic to style an object that contains some 'content' which can be styled.

use super::{color, Color, ObjectStyle};
//use Screen;
use crossterm_utils::{Result, TerminalOutput};
use std::fmt::{self, Display, Formatter};
use std::io::Write;
use std::result;
use std::sync::Arc;

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

    /// This converts an styled object into an `DisplayableObject` witch implements: `Display` and could be used inside the write function of the standard library.
    ///
    /// _StyledObject already implements `Display` right?_
    ///
    /// This is true, however there are some complex issues why this won't work on alternate screen.
    /// That is the reason why this functions exists.
    /// You could just pass in the 'screen' from your alternate screen to this method and your `StyledObject` will be printed to the alternate screen just fine.
    ///
    /// ```
    ///    let screen = Screen::default(); /* represents the alternate screen */
    ///    let styled_object = style("test").with(Color::Yellow);
    ///    let display_object = styled_object.into_displayable(&screen);
    ///    println!("Colored text: {}. Default color", display_object);
    /// ```
    pub fn into_displayable(self, stdout: &'a Arc<TerminalOutput>) -> DisplayableObject<'a, D> {
        DisplayableObject::new(stdout, self)
    }

    /// This could be used to paint the styled object onto the given screen. You have to pass a reference to the screen whereon you want to perform the painting.
    ///
    /// ``` rust
    /// style("Some colored text")
    ///     .with(Color::Blue)
    ///     .on(Color::Black)
    ///     .paint(&screen);
    /// ```
    ///
    /// You should take not that `StyledObject` implements `Display`. You don't need to call paint unless you are on alternate screen.
    /// Checkout `into_displayable()` for more information about this.
    pub fn paint(&self, stdout: &Arc<TerminalOutput>) -> Result<()> {
        let colored_terminal = super::TerminalColor::from_output(stdout);

        let mut reset = true;

        if let Some(bg) = self.object_style.bg_color {
            colored_terminal.set_bg(bg)?;
            reset = true;
        }

        if let Some(fg) = self.object_style.fg_color {
            colored_terminal.set_fg(fg)?;
            reset = true;
        }
        for attr in self.object_style.attrs.iter() {
            stdout.write_string(format!(csi!("{}m"), *attr as i16))?;
            reset = true;
        }
        use std::fmt::Write;
        let mut content = String::new();
        write!(content, "{}", self.content)?;
        stdout.write_string(content)?;
        stdout.flush()?;
        if reset {
            colored_terminal.reset()?;
        }
        Ok(())
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

/// This is a wrapper for a styled object on 'alternate screen' so that the styled object could be printed on the 'alternate screen' with the standard write functions in rust.
///
/// ```
/// write! ("some normal text, {} <- some colored text", DisplayableObject::new(&screen, styled_object));
/// println! ("some normal text, {} <- some colored text", DisplayableObject::new(&screen, styled_object));
/// ```
pub struct DisplayableObject<'a, D: Display + 'a> {
    styled_object: StyledObject<D>,
    output: &'a Arc<TerminalOutput>,
}

impl<'a, D: Display + 'a> DisplayableObject<'a, D> {
    pub fn new(
        screen: &'a Arc<TerminalOutput>,
        styled_object: StyledObject<D>,
    ) -> DisplayableObject<'a, D> {
        DisplayableObject {
            output: screen,
            styled_object,
        }
    }
}

impl<'a, D: Display + 'a> Display for DisplayableObject<'a, D> {
    fn fmt(&self, _f: &mut Formatter) -> result::Result<(), fmt::Error> {
        self.styled_object.paint(self.output).unwrap();
        Ok(())
    }
}
