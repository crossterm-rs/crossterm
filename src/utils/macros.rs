/// Append a the first few characters of an ANSI escape code to the given string.
#[macro_export]
#[doc(hidden)]
macro_rules! csi {
    ($( $l:expr ),*) => { concat!("\x1B[", $( $l ),*) };
}

/// Write a string to standard output whereafter the stdout will be flushed.
#[doc(hidden)]
#[macro_export]
macro_rules! write_string {
    ($write:expr, $string:expr) => {{
        use std::error::Error;
        #[allow(unused_imports)]
        use std::io::{self, ErrorKind, Write};

        let result = write!($write, "{}", $string)
            .map_err(|e| io::Error::new(ErrorKind::Other, e.description()))
            .map_err(|e| $crate::ErrorKind::IoError(e));

        if let Err(_) = &result {
            Some(result)
        } else {
            None
        }
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! handle_command {
    ($write:expr, $string:expr) => {{
        // Silent warning when the macro is used inside the `command` module
        #[allow(unused_imports)]
        use $crate::{write_string, Command};

        #[cfg(windows)]
        {
            if $crate::supports_ansi() {
                write_string!($write, $string.ansi_code())
            } else {
                if let Err(e) = $string.execute_winapi() {
                    Some(Err($crate::ErrorKind::from(e)))
                } else {
                    None
                }
            }
        }
        #[cfg(unix)]
        {
            write_string!($write, $string.ansi_code())
        }
    }};
}

/// Queue one or more command(s) for execution in the near future.
///
/// Queued commands will be executed in the following cases:
/// - When you manually call `flush` on the given writer.
/// - When the buffer is to full, then the terminal will flush for you.
/// - Incase of `stdout` each line, because `stdout` is line buffered.
///
/// # Parameters
/// - [std::io::Writer](https://doc.rust-lang.org/std/io/trait.Write.html)
///
///     Crossterm will write the ANSI escape codes to this given writer (No flush will be done).
/// - [Command](./trait.Command.html)
///
///     Give one or more commands that you want to queue for execution
///
/// # Example
/// ```rust
/// use std::io::{Write, stdout};
///
/// use crossterm::{queue, Output};
///
/// let mut stdout = stdout();
///
/// // will be executed when flush is called
/// queue!(stdout, Output("foo".to_string()));
///
/// // some other code (no execution happening here) ...
///
/// // when calling flush on stdout, all commands will be written to the stdout and therefor executed.
/// stdout.flush();
/// ```
///
/// # Remarks
/// - In the case of UNIX and windows 10, ANSI codes are written to the given 'writer'.
/// - In case of Windows versions lower than 10, a direct WinApi call will be made.
/// This is happening because windows versions lower then 10 do not support ANSI codes, and thus they can't be written to the given buffer.
/// Because of that there is no difference between `execute` and `queue` for those windows versions.
/// - Queuing might sound that there is some scheduling going on, however, this means that we write to the stdout without flushing which will cause commands to be stored in the buffer without them being written to the terminal.
#[macro_export]
macro_rules! queue {
    ($write:expr, $($command:expr), * $(,)?) => {{
        // Silent warning when the macro is used inside the `command` module
        #[allow(unused_imports)]
        use $crate::{Command, handle_command};

        #[allow(unused_assignments)]
        let mut error = None;

        $(
            error = handle_command!($write, $command);
        )*

        error.unwrap_or(Ok(()))
    }}
}

/// Execute one or more command(s)
///
/// # Parameters
/// - [std::io::Writer](https://doc.rust-lang.org/std/io/trait.Write.html)
///
///     Crossterm will write the ANSI escape codes to this given. (A flush will be done)
/// - [Command](./trait.Command.html)
///
///     Give one or more commands that you want to execute
///
/// # Example
/// ```rust
/// use std::io::Write;
///
/// use crossterm::{execute, Output};
///
/// // will be executed directly
/// execute!(std::io::stdout(), Output("foo".to_string()));
///
/// // will be executed directly
/// execute!(std::io::stdout(), Output("foo".to_string()), Output("bar".to_string()));
/// ```
///
/// # Remarks
/// - In the case of UNIX and windows 10, ANSI codes are written to the given 'writer'.
/// - In case of Windows versions lower than 10, a direct WinApi call will be made.
/// This is happening because Windows versions lower then 10 do not support ANSI codes, and thus they can't be written to the given buffer.
/// Because of that there is no difference between `execute` and `queue` for those windows versions.
#[macro_export]
macro_rules! execute {
    ($write:expr, $($command:expr), * $(,)? ) => {{
        // Silent warning when the macro is used inside the `command` module
        #[allow(unused_imports)]
        use $crate::{handle_command, Command};

        #[allow(unused_assignments)]
        let mut error = None;

        $(
            if let Some(Err(e)) = handle_command!($write, $command) {
                error = Some(Err(e));
            }else {
                $write.flush().map_err(|e| $crate::ErrorKind::IoError(e)).unwrap();
            }
        )*

        error.unwrap_or(Ok(()))
    }}
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_display {
    (for $($t:ty),+) => {
        $(impl ::std::fmt::Display for $t {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::result::Result<(), ::std::fmt::Error> {
                $crate::queue!(f, self).map_err(|_| ::std::fmt::Error)
            }
        })*
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_from {
    ($from:path, $to:expr) => {
        impl From<$from> for ErrorKind {
            fn from(e: $from) -> Self {
                $to(e)
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use std::io::{stdout, Write};

    use crate::utils::command::Command;
    #[cfg(windows)]
    use crate::utils::error::ErrorKind;

    pub struct FakeCommand;

    impl Command for FakeCommand {
        type AnsiType = &'static str;

        fn ansi_code(&self) -> Self::AnsiType {
            ""
        }

        #[cfg(windows)]
        fn execute_winapi(&self) -> Result<(), ErrorKind> {
            Ok(())
        }
    }

    #[test]
    fn test_queue() {
        assert!(queue!(stdout(), FakeCommand,).is_ok());
        assert!(queue!(stdout(), FakeCommand).is_ok());
    }

    #[test]
    fn test_execute() {
        assert!(execute!(stdout(), FakeCommand,).is_ok());
        assert!(execute!(stdout(), FakeCommand).is_ok());
    }
}
