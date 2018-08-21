//! This module contains the logic to style an object that contains some state witch can be styled.

use super::{Color, ObjectStyle};
use Screen;

use std::fmt::Display;

#[cfg(unix)]
use super::Attribute;

/// Struct that contains both the style and the content wits can be styled.
pub struct StyledObject<D: Display> {
    pub object_style: ObjectStyle,
    pub content: D,
}

impl<D: Display> StyledObject<D> {
    /// Set the foreground of the styled object to the passed `Color`
    ///
    /// ```rust
    /// use self::crossterm::style::{style,Color};
    ///
    /// // create an styled object with the foreground color red.
    /// let styledobject =  style("Some colored text").with(Color::Blue);
    /// // create an styled object with the foreground color blue.
    /// let styledobject1 = style("Some colored text").with(Color::Blue);
    ///
    /// let screen = Screen::default();
    ///
    /// // print the styledobject to see the result
    /// styledobject.paint(&screen);
    /// styledobject1.paint(&screen);
    ///
    /// // print an styled object directly.
    /// style("Some colored text").with(Color::Blue).paint(&screen);
    ///
    /// ```
    pub fn with(mut self, foreground_color: Color) -> StyledObject<D> {
        self.object_style = self.object_style.fg(foreground_color);
        self
    }

    /// Set the background of the styled object to the passed `Color`
    ///
    /// #Example
    ///
    /// ```rust
   /// use self::crossterm::style::{style,Color};
    ///
    /// // create an styled object with the background color red.
    /// let styledobject =  style("Some colored text").on(Color::Blue);
    /// // create an styled object with the foreground color blue.
    /// let styledobject1 = style("Some colored text").on(Color::Blue);
    ///
    /// let screen = Screen::default();
    ///
    /// // print the styledobject to see the result
    /// styledobject.paint(&screen);
    /// styledobject1.paint(&screen);
    ///
    /// // print an styled object directly.
    /// style("Some colored text").on(Color::Blue).paint(&screen);
    ///
    /// ```
    pub fn on(mut self, background_color: Color) -> StyledObject<D> {
        self.object_style = self.object_style.bg(background_color);
        self
    }

    /// Set the attribute of an styled object to the passed `Attribute`
    ///
    /// #Example
    ///
    /// ```rust
    ///
    /// extern crate crossterm;
    /// use self::crossterm::style::{style,Attribute};
    ///
    /// style("Some colored text").attr(Attribute::Bold).paint(&screen);
    ///
    /// ```
    #[cfg(unix)]
    pub fn attr(mut self, attr: Attribute) -> StyledObject<D> {
        &self.object_style.add_attr(attr);
        self
    }

    /// Increase the font intensity.
    #[cfg(unix)]
    #[inline(always)]
    pub fn bold(self) -> StyledObject<D> {
        self.attr(Attribute::Bold)
    }
    /// Faint (decreased intensity) (Not widely supported).
    #[cfg(unix)]
    #[inline(always)]
    pub fn dim(self) -> StyledObject<D> {
        self.attr(Attribute::Dim)
    }
    /// Make the font italic (Not widely supported; Sometimes treated as inverse).
    #[cfg(unix)]
    #[inline(always)]
    pub fn italic(self) -> StyledObject<D> {
        self.attr(Attribute::Italic)
    }
    /// Underline font.
    #[cfg(unix)]
    #[inline(always)]
    pub fn underlined(self) -> StyledObject<D> {
        self.attr(Attribute::Underlined)
    }
    /// Slow Blink (less than 150 per minute; not widely supported).
    #[cfg(unix)]
    #[inline(always)]
    pub fn slow_blink(self) -> StyledObject<D> {
        self.attr(Attribute::SlowBlink)
    }
    /// Rapid Blink (MS-DOS ANSI.SYS; 150+ per minute; not widely supported).
    #[cfg(unix)]
    #[inline(always)]
    pub fn rapid_blink(self) -> StyledObject<D> {
        self.attr(Attribute::RapidBlink)
    }
    /// Swap foreground and background colors.
    #[cfg(unix)]
    #[inline(always)]
    pub fn reverse(self) -> StyledObject<D> {
        self.attr(Attribute::Reverse)
    }
    /// Hide text (Not widely supported).
    #[cfg(unix)]
    #[inline(always)]
    pub fn hidden(self) -> StyledObject<D> {
        self.attr(Attribute::Hidden)
    }
    /// Characters legible, but marked for deletion. Not widely supported.
    #[cfg(unix)]
    #[inline(always)]
    pub fn crossed_out(self) -> StyledObject<D> {
        self.attr(Attribute::CrossedOut)
    }

    /// This could be used to paint the styled object on the screen. Pass a reference to the screen whereon you want to perform the painting.
    ///
    /// ``` rust
    /// style("Some colored text")
    ///     .with(Color::Blue)
    ///     .on(Color::Black)
    ///     .paint(&screen);
    /// ```
    pub fn paint(&self, screen: &Screen)
    {
        let colored_terminal = ::color(&screen);
        let mut reset = true;

        if let Some(bg) = self.object_style.bg_color {
            colored_terminal.set_bg(bg);
            reset = true;
        }

        if let Some(fg) = self.object_style.fg_color {
            colored_terminal.set_fg(fg);
            reset = true;
        }

        #[cfg(unix)]
            for attr in self.object_style.attrs.iter() {
            screen.stdout.write_string(format!(csi!("{}m"), *attr as i16));
            reset = true;
        }

        use std::fmt::Write;
        let mut content = String::new();
        write!(content, "{}", self.content).unwrap();
        screen.stdout.write_string(content);
        screen.stdout.flush();

        if reset {
            colored_terminal.reset();
        }
    }
}
