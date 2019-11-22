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
        use std::{
            error::Error,
            io::{self, ErrorKind},
        };

        write!($write, "{}", $string)
            .map_err(|e| io::Error::new(ErrorKind::Other, e.description()))
            .map_err($crate::ErrorKind::IoError)
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
                $string.execute_winapi().map_err($crate::ErrorKind::from)
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
///
/// * When `flush` is called manually on the given type implementing `io::Write`.
/// * When the buffer is to full, then the terminal will `flush` for you.
/// * Each line in case of `stdout`, because it is line buffered.
///
/// # Arguments
///
/// - [std::io::Writer](https://doc.rust-lang.org/std/io/trait.Write.html)
///
///     ANSI escape codes are written on the given 'writer', after which they are flushed.
///
/// - [Command](./trait.Command.html)
///
///     One or more commands that you want to execute
///
/// # Examples
///
/// ```rust
/// use std::io::{Write, stdout};
/// use crossterm::{queue, style::Print};
///
/// fn main() {
///     let mut stdout = stdout();
///
///     // `Print` will executed executed when `flush` is called.
///     queue!(stdout, Print("foo".to_string()));
///
///     // some other code (no execution happening here) ...
///
///     // when calling `flush` on `stdout`, all commands will be written to the stdout and therefore executed.
///     stdout.flush();
///
///     // ==== Output ====
///     // foo
/// }
/// ```
///
/// For the full documentation of the command API, please have a look over [here](./#command-api).
///
/// # Notes
///
/// * In case of Windows versions lower than 10, a direct WinApi call will be made.
/// The reason for this is that Windows versions lower than 10 do not support ANSI codes,
/// and can therefore not be written to the given `writer`.
/// Therefore, there is no difference between [execute](macro.execute.html)
/// and [queue](macro.queue.html) for those old Windows versions.
#[macro_export]
macro_rules! queue {
    ($write:expr, $($command:expr), * $(,)?) => {{
        // Silent warning when the macro is used inside the `command` module
        #[allow(unused_imports)]
        use $crate::{Command, handle_command};

        #[allow(unused_assignments)]
        let mut error = Ok(());

        $(
            error = handle_command!($write, $command);
        )*

        error
    }}
}

/// Execute one or more command(s).
///
/// # Arguments
///
/// - [std::io::Writer](https://doc.rust-lang.org/std/io/trait.Write.html)
///
///     ANSI escape codes are written on the given 'writer', after which they are flushed.
///
/// - [Command](./trait.Command.html)
///
///     One or more commands that you want to execute
///
/// # Examples
///
/// ```rust
/// use std::io::{Write, stdout};
/// use crossterm::{execute, style::Print};
///
///  fn main() {
///      // will be executed directly
///      execute!(stdout(), Print("sum:\n".to_string()));
///
///      // will be executed directly
///      execute!(stdout(), Print("1 + 1= ".to_string()), Print((1+1).to_string()));
///
///      // ==== Output ====
///      // sum:
///      // 1 + 1 = 2
///  }
/// ```
///
/// For the full documentation of the command API, please have a look over [here](./#command-api).
///
/// # Notes
///
/// * In the case of UNIX and Windows 10, ANSI codes are written to the given 'writer'.
/// * In case of Windows versions lower than 10, a direct WinApi call will be made.
/// The reason for this is that Windows versions lower than 10 do not support ANSI codes,
/// and can therefore not be written to the given `writer`.
/// Therefore, there is no difference between [execute](macro.execute.html)
/// and [queue](macro.queue.html) for those old Windows versions.
#[macro_export]
macro_rules! execute {
    ($write:expr, $($command:expr), * $(,)? ) => {{
        // Silent warning when the macro is used inside the `command` module
        #[allow(unused_imports)]
        use $crate::{handle_command, Command};

        #[allow(unused_assignments)]
        let mut error = Ok(());

        $(
            if let Err(e) = handle_command!($write, $command) {
                error = Err(e);
            }else {
                $write.flush().map_err($crate::ErrorKind::IoError).unwrap();
            }
        )*

        error
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
