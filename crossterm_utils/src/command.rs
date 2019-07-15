use crate::{execute, impl_display, schedule, write_cout, ErrorKind, Result};

#[cfg(windows)]
use crate::supports_ansi;

use std::fmt::Display;
use std::fmt::{self, Error, Formatter};
use std::intrinsics::write_bytes;
use std::io::Write;

pub trait Command {
    type AnsiType: Display;

    fn get_ansi_code(&self) -> Self::AnsiType;

    fn execute(&self) -> Result<()> {
        write_cout!(self.get_ansi_code());
        Ok(())
    }

    #[cfg(windows)] // Not sure if these are possible in traits.s
    fn execute_winapi(&self) -> Result<()>;
}

pub trait QueueableCommand<T: Display> {
    fn queue(mut self, command: impl Command<AnsiType = T>) -> Self;
}

pub trait ExecutableCommand<T: Display> {
    fn execute(mut self, command: impl Command<AnsiType = T>) -> Self;
}

impl<T, A> QueueableCommand<A> for T
where
    A: Display,
    T: Write,
{
    fn queue(mut self, command: impl Command<AnsiType = A>) -> Self {
        schedule!(self, command);
        self
    }
}

impl<T, A> ExecutableCommand<A> for T
where
    A: Display,
    T: Write,
{
    fn execute(mut self, command: impl Command<AnsiType = A>) -> Self {
        execute!(self, command);
        self
    }
}

pub struct Output(pub String);

impl Command for Output {
    type AnsiType = String;

    fn get_ansi_code(&self) -> Self::AnsiType {
        return self.0.clone();
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        print!("{}", self.0);
        Ok(())
    }
}

impl_display!(for Output);
