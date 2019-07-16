use crate::{execute, impl_display, schedule, write_cout, ErrorKind, Result};

#[cfg(windows)]
use crate::supports_ansi;

use std::fmt::Display;
use std::fmt::{self, Error, Formatter};
use std::intrinsics::write_bytes;
use std::io::Write;

/// A command is something
pub trait Command {
    type AnsiType: Display;

    /// Returns the ANSI code representation of this command.
    /// You can manipulate the terminal behaviour by writing an ANSI escape code to the terminal.
    /// You are able to use ANSI escape codes only for windows 10 and UNIX systems.
    fn get_ansi_code(&self) -> Self::AnsiType;

    /// Execute this command.
    ///
    /// ANSI escape code is used for systems that support ANSI escape codes ( Windows 10 & UNIX), otherwise WinApi ( < Windows 10) will be used.
    fn execute(&self) -> Result<()> {
        write_cout!(self.get_ansi_code());
        Ok(())
    }

    /// Execute this command.
    ///
    /// On operating systems that do not support ANSI escape codes ( < Windows 10) we need to call WinApi to execute this command.
    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()>;
}

/// A trait that defines behaviour for a command that can be used to be executed at a later time point.
/// This can be used in order to get more performance.
pub trait QueueableCommand<T: Display> {
    /// Queues the given command for later execution.
    fn queue(mut self, command: impl Command<AnsiType = T>) -> Self;
}

/// A trait that defines behaviour for a command that will be executed immediately.
pub trait ExecutableCommand<T: Display> {
    /// Execute the given command directly.
    fn execute(mut self, command: impl Command<AnsiType = T>) -> Self;
}

impl<T, A> QueueableCommand<A> for T
where
    A: Display,
    T: Write,
{
    /// Queue the given command for later execution.
    /// This function will `write` the ANSI escape code to this type without calling `flush`.
    /// This can be very useful when executing a lot of commands because flushing costs a lot of performance.
    ///
    /// If you want to execute a command directly, use: `execute(command)`.
    ///
    /// # Remarks
    /// - WinApi is used for Windowws versions lower then 10
    /// - ANSI escape codes are written to the terminal buffer of Windows 10 and UNIX systems.
    /// - On windows systems lower than 10, commands can't be queued but are executed immediately.
    /// The reason for this is that those are using WinAPI to perform the command action.
    /// Contrary to ANSI escape codes we cannot write them to the terminal buffer but must execute them immediately.
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
    /// Execute the given command directly.
    /// This function will `write` the ANSI escape code to this type and call `flush`.
    ///
    /// In case you have many executions after on and another you can use `queue(command)` to get some better performance.
    /// The `queue` function will not call `flush`.
    ///
    /// # Remarks
    /// - WinApi is used for Windowws versions lower then 10
    /// - ANSI escape codes are written to the terminal buffer of Windows 10 and UNIX systems.
    fn execute(mut self, command: impl Command<AnsiType = A>) -> Self {
        execute!(self, command);
        self
    }
}

/// When executed, this command will output the given string to the terminal.
///
/// See `crossterm/examples/command.rs` for more information on how to execute commands.
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
