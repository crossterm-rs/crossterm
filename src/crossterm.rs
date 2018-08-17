use common::commands::IAlternateScreenCommand;

use common::screen::AlternateScreen;

use std::fmt::Display;
use std::io::Write;
use std::io::Result;
use std::sync::Arc;
use TerminalOutput;
use std::collections::HashMap;

#[cfg(not(windows))]
use common::commands::unix_command;

#[cfg(windows)]
use common::commands::win_commands;

/// This type could be used to access the `cursor, terminal, color, input, styling` module more easily.
/// You need to pass a reference to the screen where on you want to perform the actions to the `Crossterm` type.
///
/// If you want to use the default screen you could do it like this:
///
/// ```rust
/// extern crate crossterm;
/// use crossterm::{Crossterm, Screen};
///
/// let crossterm = Crossterm::new(&Screen::default());
/// let cursor = crossterm.cursor();
/// ```
///
/// If you want to perform actions on the `AlternateScreen` make sure to pass a reference to the screen of the `AlternateScreen`.
///
/// ```
/// extern crate crossterm;
/// use crossterm::{Crossterm, Screen};
///
/// let main_screen = Screen::default();
///
/// if let Ok(alternate_srceen) = main_screen.enable_alternate_modes(false)
/// {
///    let crossterm = Crossterm::new(&alternate_screen.screen);
///    let cursor = crossterm.cursor();
/// }
/// ```
pub struct Crossterm<I: PartialEq, Hash> {
    pub main_output: Arc<TerminalOutput>,
    pub alternate_screens: HashMap<I, Arc<TerminalOutput>>,
    pub current_output: Arc<TerminalOutput>,
    pub raw_mode: bool,
}

impl<I: PartialEq + Hash> Crossterm<I> {
    /// Create a new instance of `Crossterm`
    pub fn new() -> Crossterm {
        let main = Arc::new(terminal_output());
        Crossterm {
            main_output: main.clone(),
            alternate_screens: HashMap::new(),
            current_output: main,
            raw_mode: false,
        }
    }

    pub fn enable_raw_mode(&mut self) -> io::Result<()> {
        RawScreen::into_raw_mode()?;
        self.raw_mode = true;
        Ok(())
    }
    
    pub fn disable_raw_mode(&mut self) -> io::Result<()> {
        RawScreen::disable_raw_modes()?;
        self.raw_mode = false;
        Ok(())
    }
    
    pub fn create_alternate_screen(&mut self, identifier: I) {
        self.alternate_screens.insert(identifier, terminal_output());
    }
    
    pub fn to_alternate_screen(&mut self, identifier: I) -> io::Result<()> {
        let target = self.alternate_screens.entry(identifier).or_insert_with(|| terminal_output());
        AlternateScreen::to_alternate_screen(target)?;
        Ok(())
    }
    
    pub fn to_main_screen(&mut self) -> io::Result<()> {
        if self.current_output != self.main_output {
            to_main_screen(self.current_output)?;
            self.current_output = self.main_output.clone();
        }
        Ok(())
    }
}

impl<I> Default for Crossterm<I> {
    fn default() -> Self {
        Self::new()
    }
}

impl<I> Drop for Crossterm<I> {
    /// If the current screen is in raw mode whe need to disable it when the instance goes out of scope.
    fn drop(&mut self) {
        if self.raw_mode {
            self.disable_raw_mode().expect("Failed to disable raw mode when destroying `Crossterm`");
        }
        self.to_main_screen();
    }
}
