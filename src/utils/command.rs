use std::{fmt::Display, io::Write};

use crate::{execute, queue, write_cout};

use super::error::Result;

/// A command is an action that can be performed on the terminal.
///
/// crossterm already delivers a number of commands.
/// There is no need to implement them yourself.
/// Also, you don't have to execute the commands yourself by calling a function.
pub trait Command {
    type AnsiType: Display;

    /// Returns the ANSI code representation of this command.
    /// You can manipulate the terminal behaviour by writing an ANSI escape code to the terminal.
    /// You are able to use ANSI escape codes only for windows 10 and UNIX systems.
    ///
    /// **This method is mainly used internally by crossterm!**
    fn ansi_code(&self) -> Self::AnsiType;

    /// Execute this command.
    ///
    /// On operating systems that do not support ANSI escape codes ( < Windows 10) we need to call WinApi to execute this command.
    ///
    /// **This method is mainly used internally by crossterm!**
    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()>;
}

/// A trait that defines behaviour for a command which can be executed at a later time.
/// This can be used in order to get more performance.
pub trait QueueableCommand<T: Display>: Sized {
    /// Queues the given command for later execution.
    fn queue(&mut self, command: impl Command<AnsiType = T>) -> Result<&mut Self>;
}

/// A trait that defines behaviour for a command which will be executed immediately.
pub trait ExecutableCommand<T: Display>: Sized {
    /// Execute the given command directly.
    fn execute(&mut self, command: impl Command<AnsiType = T>) -> Result<&mut Self>;
}

impl<T, A> QueueableCommand<A> for T
where
    A: Display,
    T: Write,
{
    /// Queue the given command for later execution.
    ///
    /// Queued commands will be executed in the following cases:
    /// - When you manually call `flush` on the given writer.
    /// - When the buffer is to full, then the terminal will flush for you.
    /// - Incase of `stdout` each line, because `stdout` is line buffered.
    ///
    /// # Parameters
    /// - [Command](./trait.Command.html)
    ///
    ///     The command that you want to queue for later execution.
    ///
    /// # Remarks
    /// - In the case of UNIX and windows 10, ANSI codes are written to the given 'writer'.
    /// - In case of Windows versions lower than 10, a direct WinApi call will be made.
    /// This is happening because windows versions lower then 10 do not support ANSI codes, and thus they can't be written to the given buffer.
    /// Because of that there is no difference between `execute` and `queue` for those windows versions.
    /// - Queuing might sound that there is some scheduling going on, however, this means that we write to the stdout without flushing which will cause commands to be stored in the buffer without them being written to the terminal.
    fn queue(&mut self, command: impl Command<AnsiType = A>) -> Result<&mut Self> {
        queue!(self, command)?;
        Ok(self)
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
    /// - In the case of UNIX and windows 10, ANSI codes are written to the given 'writer'.
    /// - In case of Windows versions lower than 10, a direct WinApi call will be made.
    /// This is happening because Windows versions lower then 10 do not support ANSI codes, and thus they can't be written to the given buffer.
    /// Because of that there is no difference between `execute` and `queue` for those windows versions.
    fn execute(&mut self, command: impl Command<AnsiType = A>) -> Result<&mut Self> {
        execute!(self, command)?;
        Ok(self)
    }
}

/// When executed, this command will output the given displayable to the buffer.
///
/// See `crossterm/examples/command.rs` for more information on how to execute commands.
pub struct Output<T: Display + Clone>(pub T);

impl<T: Display + Clone> Command for Output<T> {
    type AnsiType = T;

    fn ansi_code(&self) -> Self::AnsiType {
        self.0.clone()
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        print!("{}", self.0);
        Ok(())
    }
}

impl<T: Display + Clone> Display for Output<T> {
    fn fmt(
        &self,
        f: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        write!(f, "{}", self.ansi_code())
    }
}
