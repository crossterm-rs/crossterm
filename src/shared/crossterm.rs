//! This module provides easy access to the functionalities of crossterm.
//! since `crossterm version 0.3.0` an `Context` type is introduced (check that documentation for more info why this type is introduced).
//!
//! You have to provide this `Context` to the modules: `cursor::cursor(), color::color(), terminal::terminal()`.
//!
//!   use crossterm::Context;
//!   use crossterm::cursor;
//!   use crossterm::color;
//!   use crossterm::terminal;
//!
//!   let context = Context::new();
//!   let cursor = cursor::cursor(&context)
//!   let terminal = terminal::terminal(&context);
//!   let color = terminal::color(&context);
//!
//! Because it can seem a little odd to constantly create an Context and provide it to these modules.
//! You can better use `Crossterm` for accessing these modules.
//! `Crossterm` handles the Context internally so jo do not have to bother about it, for example:
//!
//!      let crossterm = Crossterm::new();
//!      let color = crossterm.color();
//!      let cursor = crossterm.cursor();
//!      let terminal = crossterm.terminal();

use super::super::cursor;
use super::super::input::input;
use super::super::style;
use super::super::terminal::terminal;

use Context;

use std::convert::From;
use std::fmt::Display;
use std::mem;
use std::rc::Rc;
use std::sync::Arc;

/// Because it can seem a little odd to constantly create an `Context` and provide it to modules like: `cursor, color and terminal`.
/// You can better use `Crossterm` for accessing these modules.
/// `Crossterm` handles the Context internally so jo do not have to bother about it, for example:
///
/// Check `/examples/Crossterm 0.3.0/program_examples/first_depth_search` in the library for more specific examples.
///
///      let crossterm = Crossterm::new();
///      let color = crossterm.color();
///      let cursor = crossterm.cursor();
///      let terminal = crossterm.terminal();
pub struct Crossterm {
    context: Rc<Context>,
}

/// Create `Crossterm` instance from `Context`
impl From<Rc<Context>> for Crossterm {
    fn from(context: Rc<Context>) -> Self {
        return Crossterm { context: context };
    }
}

impl Crossterm {
    pub fn new() -> Crossterm {
        return Crossterm {
            context: Context::new(),
        };
    }

    /// Get an Terminal implementation whereon terminal related actions can be performed.
    ///
    /// #Example
    ///
    /// ```rust
    ///
    /// extern crate crossterm;
    /// use crossterm::Crossterm;
    /// use crossterm::terminal;
    ///
    /// let crossterm = Crossterm::new();
    /// let mut terminal = crossterm.terminal();
    ///
    /// ```
    pub fn terminal(&self) -> terminal::Terminal {
        return terminal::Terminal::new(self.context.clone());
    }

    /// Get an TerminalCursor implementation whereon cursor related actions can be performed.
    ///
    /// #Example
    ///
    /// ```rust
    ///
    /// extern crate crossterm;
    /// use crossterm::Crossterm;
    /// use crossterm::terminal;
    ///
    /// let crossterm = Crossterm::new();
    /// let mut cursor = crossterm.cursor();
    ///
    /// // move cursor to x: 5 and y:10
    /// cursor.goto(5,10);
    ///
    /// ```
    pub fn cursor(&self) -> cursor::TerminalCursor {
        return cursor::TerminalCursor::new(self.context.clone());
    }

    /// Get an Color implementation whereon color related actions can be performed.
    ///
    /// Check `/examples/version/color` in the library for more specific examples.
    ///
    /// #Example
    ///
    /// ```rust
    ///
    /// extern crate crossterm;
    /// use crossterm::Crossterm;
    /// use crossterm::terminal;
    ///
    /// let crossterm = Crossterm::new();
    /// let mut terminal_color = crossterm.color();
    ///
    /// ```
    pub fn color(&self) -> style::TerminalColor {
        return style::TerminalColor::new(self.context.clone());
    }

    pub fn input(&self) -> input::TerminalInput {
        return input::TerminalInput::new(self.context.clone());
    }

    /// Wraps an displayable object so it can be formatted with colors and attributes.
    ///
    /// Check `/examples/color` in the library for more specific examples.
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate crossterm;
    ///
    /// use self::crossterm::style::{paint,Color};
    /// use self::crossterm::Crossterm;
    ///
    /// fn main()
    /// {
    ///     let crossterm = Crossterm::new();
    ///     // Create an styledobject object from the text 'Unstyled font'
    ///     // Currently it has the default foreground color and background color.
    ///     println!("{}",crossterm.paint("Unstyled font"));
    ///
    ///     // Create an displayable object from the text 'Colored font',
    ///     // Paint this with the `Red` foreground color and `Blue` background color.
    ///     // Print the result.
    ///     let styledobject = crossterm.paint("Colored font").with(Color::Red).on(Color::Blue);
    ///     println!("{}", styledobject);
    ///
    ///     // Or all in one line
    ///     println!("{}", crossterm.paint("Colored font").with(Color::Red).on(Color::Blue));
    /// }
    /// ```
    pub fn paint<'a, D: Display>(&'a self, value: D) -> style::StyledObject<D> {
        self.terminal().paint(value)
    }

    /// Write any displayable value to the current screen weather it will be the main screen or alternate screen.
    ///
    /// #Example
    ///
    /// ```rust
    ///
    /// extern crate crossterm;
    /// use crossterm::Crossterm;
    ///
    /// let mut crossterm = Crossterm::new();
    /// crossterm.write("Some text \n Some text on new line.");
    ///
    /// ```
    pub fn write<D: Display>(&self, value: D) {
        self.terminal().write(value)
    }

    /// Get an copy of the context that `Crossterm` uses internally.
    pub fn context(&self) -> Rc<Context> {
        self.context.clone()
    }
}
