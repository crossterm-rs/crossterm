//! This module contains the logic to style an object that contains some state witch can be styled.

use super::{color, from_screen, Color, ObjectStyle};
use Screen;

use std::fmt::{self, Display, Formatter};
use std::io::Write;

#[cfg(unix)]
use super::Attribute;

/// Struct that contains both the style and the content wits can be styled.
pub struct StyledObject<D: Display> {
    pub object_style: ObjectStyle,
    pub content: D,
}

impl<'a, D: Display + 'a> StyledObject<D> {
    /// Set the foreground of the styled object to the passed `Color`.
    ///
    /// ```rust
    /// use self::crossterm::style::{style,Color};
    ///
    /// // create an styled object with the foreground color red.
    /// let styledobject =  style("Some colored text").with(Color::Red);
    /// // create an styled object with the foreground color blue.
    /// let styledobject1 = style("Some colored text").with(Color::Blue);
    ///
    /// // print the styledobject to see the result
    /// println!("{}", styledobject);
    /// println!("{}", styledobject1);
    ///
    /// // print an styled object directly.
    /// println!("{}", style("Some colored text").on(Color::Blue));
    /// ```
    pub fn with(mut self, foreground_color: Color) -> StyledObject<D> {
        self.object_style = self.object_style.fg(foreground_color);
        self
    }

    /// Set the background of the styled object to the passed `Color`.
    ///
    /// #Example
    ///
    /// ```rust
    /// use self::crossterm::style::{style,Color};
    ///
    /// // create an styled object with the background color red.
    /// let styledobject =  style("Some colored text").on(Color::Red);
    /// // create an styled object with the foreground color blue.
    /// let styledobject1 = style("Some colored text").on(Color::Blue);
    ///
    /// // print the styledobject to see the result
    /// println!("{}", styledobject);
    /// println!("{}", styledobject1);
    ///
    /// // print an styled object directly.
    /// println!("{}", style("Some colored text").on(Color::Blue));
    /// ```
    pub fn on(mut self, background_color: Color) -> StyledObject<D> {
        self.object_style = self.object_style.bg(background_color);
        self
    }

    /// Set the attribute of an styled object to the passed `Attribute`.
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate crossterm;
    /// use self::crossterm::style::{style,Attribute};
    ///
    /// println!("{}", style("Some bold text").attr(Attribute::Bold);
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
    pub fn paint(&self, screen: &Screen)
    {
        let colored_terminal = from_screen(&screen);
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
    pub fn into_displayable(self, screen: &'a Screen) -> DisplayableObject<'a, D>
    {
        return DisplayableObject::new(screen, self)
    }
}

impl<D: Display> Display for StyledObject<D> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        let mut colored_terminal = color();
        let mut reset = true;

        if let Some(bg) = self.object_style.bg_color {
            colored_terminal.set_bg(bg);
            reset = true;
        }
        if let Some(fg) = self.object_style.fg_color {
            colored_terminal.set_fg(fg);
            reset = true;
        }

        fmt::Display::fmt(&self.content, f)?;
        std::io::stdout().flush().expect("Flush stdout failed");

        if reset {
            colored_terminal.reset();
        }

        Ok(())
    }
}


/// This is a wrapper for a styled object on alternate screen so that the styled object could be printed on the alternate screen with the standard write functions in rust.
///
/// ```
/// write! ("some normal text, {} <- some colored text", DisplayableObject::new(&screen, styled_object));
/// println! ("some normal text, {} <- some colored text", DisplayableObject::new(&screen, styled_object));
/// ```
pub struct DisplayableObject<'a, D:Display + 'a>
{
    styled_object: StyledObject<D>,
    screen: &'a Screen,
}

impl <'a, D: Display + 'a> DisplayableObject<'a, D>
{
    pub fn new(screen: &'a Screen, styled_object: StyledObject<D>) -> DisplayableObject<'a, D>
    {
        DisplayableObject { screen, styled_object }
    }
}

impl<'a, D: Display + 'a> Display for DisplayableObject<'a, D>
{
    fn fmt(&self, _f: &mut Formatter) -> Result<(), fmt::Error> {
        self.styled_object.paint(&self.screen);
        return Ok(())
    }
}
