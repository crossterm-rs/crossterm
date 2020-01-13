use std::{fmt::Display, io::Write};

use crate::{execute, queue};

use super::error::Result;

/// An interface for a command that performs an action on the terminal.
///
/// Crossterm provides a set of commands,
/// and there is no immediate reason to implement a command yourself.
/// In order to understand how to use and execute commands,
/// it is recommended that you take a look at [Command Api](../#command-api) chapter.
pub trait Command {
    type AnsiType: Display;

    /// Returns an ANSI code representation of this command.
    /// An ANSI code can manipulate the terminal by writing it to the terminal buffer.
    /// However, only Windows 10 and UNIX systems support this.
    ///
    /// This method does not need to be accessed manually, as it is used by the crossterm's [Command Api](../#command-api)
    fn ansi_code(&self) -> Self::AnsiType;

    /// Execute this command.
    ///
    /// Windows versions lower than windows 10 do not support ANSI escape codes,
    /// therefore a direct WinAPI call is made.
    ///
    /// This method does not need to be accessed manually, as it is used by the crossterm's [Command Api](../#command-api)
    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()>;

    /// Returns whether the ansi code representation of this command is supported by windows.
    ///
    /// A list of supported ANSI escape codes
    /// can be found [here](https://docs.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences).
    #[cfg(windows)]
    fn is_ansi_code_supported(&self) -> bool {
        super::ansi_support::supports_ansi()
    }
}

impl<T: Command> Command for &T {
    type AnsiType = T::AnsiType;

    #[inline]
    fn ansi_code(&self) -> Self::AnsiType {
        T::ansi_code(self)
    }

    #[inline]
    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        T::execute_winapi(self)
    }

    #[cfg(windows)]
    #[inline]
    fn is_ansi_code_supported(&self) -> bool {
        T::is_ansi_code_supported(self)
    }
}

/// An interface for commands that can be queued for further execution.
pub trait QueueableCommand<T: Display>: Sized {
    /// Queues the given command for further execution.
    fn queue(&mut self, command: impl Command<AnsiType = T>) -> Result<&mut Self>;
}

/// An interface for commands that are directly executed.
pub trait ExecutableCommand<T: Display>: Sized {
    /// Executes the given command directly.
    fn execute(&mut self, command: impl Command<AnsiType = T>) -> Result<&mut Self>;
}

impl<T, A> QueueableCommand<A> for T
where
    A: Display,
    T: Write,
{
    /// Queues the given command for further execution.
    ///
    /// Queued commands will be executed in the following cases:
    ///
    /// * When `flush` is called manually on the given type implementing `io::Write`.
    /// * The terminal will `flush` automatically if the buffer is full.
    /// * Each line is flushed in case of `stdout`, because it is line buffered.
    ///
    /// # Arguments
    ///
    /// - [Command](./trait.Command.html)
    ///
    ///     The command that you want to queue for later execution.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::io::{Write, stdout};
    /// use crossterm::{Result, QueueableCommand, style::Print};
    ///
    ///  fn main() -> Result<()> {
    ///     let mut stdout = stdout();
    ///
    ///     // `Print` will executed executed when `flush` is called.
    ///     stdout
    ///         .queue(Print("foo 1\n".to_string()))?
    ///         .queue(Print("foo 2".to_string()))?;
    ///
    ///     // some other code (no execution happening here) ...
    ///
    ///     // when calling `flush` on `stdout`, all commands will be written to the stdout and therefore executed.
    ///     stdout.flush()?;
    ///
    ///     Ok(())
    ///
    ///     // ==== Output ====
    ///     // foo 1
    ///     // foo 2
    /// }
    /// ```
    ///
    /// Have a look over at the [Command API](./#command-api) for more details.
    ///
    /// # Notes
    ///
    /// * In the case of UNIX and Windows 10, ANSI codes are written to the given 'writer'.
    /// * In case of Windows versions lower than 10, a direct WinApi call will be made.
    ///     The reason for this is that Windows versions lower than 10 do not support ANSI codes,
    ///     and can therefore not be written to the given `writer`.
    ///     Therefore, there is no difference between [execute](./trait.ExecutableCommand.html)
    ///     and [queue](./trait.QueueableCommand.html) for those old Windows versions.
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
    /// Executes the given command directly.
    ///
    /// The given command its ANSI escape code will be written and flushed onto `Self`.
    ///
    /// # Arguments
    ///
    /// - [Command](./trait.Command.html)
    ///
    ///     The command that you want to execute directly.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::io::{Write, stdout};
    /// use crossterm::{Result, ExecutableCommand, style::Print};
    ///
    ///  fn main() -> Result<()> {
    ///      // will be executed directly
    ///       stdout()
    ///         .execute(Print("sum:\n".to_string()))?
    ///         .execute(Print(format!("1 + 1= {} ", 1 + 1)))?;
    ///
    ///       Ok(())
    ///
    ///      // ==== Output ====
    ///      // sum:
    ///      // 1 + 1 = 2
    ///  }
    /// ```
    ///
    /// Have a look over at the [Command API](./#command-api) for more details.
    ///
    /// # Notes
    ///
    /// * In the case of UNIX and Windows 10, ANSI codes are written to the given 'writer'.
    /// * In case of Windows versions lower than 10, a direct WinApi call will be made.
    ///     The reason for this is that Windows versions lower than 10 do not support ANSI codes,
    ///     and can therefore not be written to the given `writer`.
    ///     Therefore, there is no difference between [execute](./trait.ExecutableCommand.html)
    ///     and [queue](./trait.QueueableCommand.html) for those old Windows versions.
    fn execute(&mut self, command: impl Command<AnsiType = A>) -> Result<&mut Self> {
        execute!(self, command)?;
        Ok(self)
    }
}
