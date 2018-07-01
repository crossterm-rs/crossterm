use Context;
use super::super::cursor;
use super::super::terminal::terminal;
use super::super::style;

use std::mem;
use std::rc::Rc;
use std::fmt::Display;
use std::sync::{ONCE_INIT, Once};
static START: Once = ONCE_INIT;

pub struct Environment
{
    context: Rc<Context>,
    terminal: Box<terminal::Terminal>,
    cursor: Box<cursor::TerminalCursor>,
    color: Box<style::TerminalColor>
}

impl Environment
{
    pub fn new() -> Environment
    {
        return Environment { context: Context::new(), terminal: unsafe{ mem::zeroed()}, cursor: unsafe{ mem::zeroed()}, color: unsafe{ mem::zeroed() }}
    }

    pub fn terminal(&mut self) -> &Box<terminal::Terminal>
    {
        START.call_once(|| {
           self.terminal = terminal::terminal(self.context.clone());

        });

       &self.terminal
    }

    pub fn cursor(&mut self) -> &Box<cursor::TerminalCursor>
    {
        START.call_once(|| {
            self.cursor = cursor::cursor(self.context.clone());

        });

        &self.cursor
    }

    pub fn color(&mut self) -> &Box<style::TerminalColor>
    {
        START.call_once(|| {
            self.color = style::color(self.context.clone());

        });

        &self.color
    }

    pub fn paint<D: Display>(&mut self, value: D) -> style::StyledObject<D>
    {
        self.terminal().paint(value)
    }

    pub fn context(&self) -> Rc<Context>
    {
        return self.context.clone()
    }
}
