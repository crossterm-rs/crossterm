use super::commands::{IAlternateScreenCommand};

use super::screen::{AlternateScreen, Screen};

use super::super::cursor;
use super::super::input;
use super::super::write;
use super::super::style;
 use super::super::terminal;

use std::fmt::Display;
use std::io::Write;
use std::sync::RwLock;
use std::io::Result;
use std::sync::Arc;

#[cfg(not(windows))]
use common::commands::unix_command;

#[cfg(windows)]
use common::commands::win_commands;

use write::Stdout;

pub struct Crossterm { }

impl<'crossterm> Crossterm {
    pub fn new() -> Crossterm {
        Crossterm {}
    }

    pub fn cursor(&self, screen: &'crossterm Screen) -> cursor::TerminalCursor {
        cursor::TerminalCursor::new(&screen.stdout.clone())
    }

    pub fn input(&self, screen: &'crossterm Screen) -> input::TerminalInput {
        return input::TerminalInput::new(&screen.stdout);
    }

     pub fn terminal(&self, screen: &'crossterm Screen) -> terminal::Terminal {
         return terminal::Terminal::new(&screen.stdout);
     }

    pub fn color(&self, screen: &'crossterm Screen) -> style::TerminalColor {
        return style::TerminalColor::new(&screen.stdout);
    }

//     Wraps an displayable object so it can be formatted with colors and attributes.
//
//     Check `/examples/color` in the libary for more spesific examples.
    pub fn style<D>(&self, val: D) -> style::StyledObject<D>
    where
        D: Display,    {
        style::ObjectStyle::new().apply_to(val)
    }
}

//impl Write for Crossterm {M
//    fn write(&mut self, buf: &[u8]) -> Result<usize> {
//        self.active_screen.write_buf(buf)
//    }
//
//    fn flush(&mut self) -> Result<()> {
//        self.active_screen.flush()
//    }
//}
//
//impl Drop for Crossterm {
//    fn drop(&mut self) {
//        if let Some(ref mut screen) = self.alternate_screen {
//            screen.disable(&mut self.active_screen);
//        }
//        if let Some(ref mut raw_terminal) = self.raw_terminal {
//            raw_terminal.disable();
//        }
//    }
//}
