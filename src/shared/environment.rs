///


use super::super::cursor;
use super::super::style;
use super::super::terminal::terminal;
use Context;

use std::fmt::Display;
use std::mem;
use std::rc::Rc;
use std::sync::Arc;
use std::convert::From;

///
pub struct Crossterm {
    context: Rc<Context>
}

impl From<Rc<Context>> for Crossterm
{
    fn from(context: Rc<Context>) -> Self {
        return Crossterm {
            context: context
        }
    }
}

impl Crossterm {
    pub fn new() -> Crossterm {
        return Crossterm { context: Context::new() };
    }

    pub fn terminal(&self) -> terminal::Terminal
    {
        return terminal::Terminal::new(self.context.clone());
    }

    pub fn cursor(&self) -> cursor::TerminalCursor
    {
        return cursor::TerminalCursor::new(self.context.clone())
    }

    pub fn color(&self) -> style::TerminalColor
    {
        return style::TerminalColor::new(self.context.clone());
    }

    pub fn paint<D: Display>(&self, value: D) -> style::StyledObject<D> {
        self.terminal().paint(value)
    }

    pub fn write<D: Display>(&mut self, value: D)
    {
        self.terminal().write(value)
    }

    pub fn context(&self) -> Rc<Context> {
        self.context.clone()
    }
}