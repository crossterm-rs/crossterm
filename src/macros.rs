/// Append a the first few characters of an ANSI escape code to the given string.
#[macro_export]
#[doc(hidden)]
macro_rules! csi {
    ($( $l:expr ),*) => { concat!("\x1B[", $( $l ),*) };
}

/// Writes an ansi code to the given writer.
#[doc(hidden)]
#[macro_export]
macro_rules! write_ansi_code {
    ($writer:expr, $ansi_code:expr) => {{
        use std::io::{self, ErrorKind};

        write!($writer, "{}", $ansi_code)
            .map_err(|e| io::Error::new(ErrorKind::Other, e))
            .map_err($crate::ErrorKind::IoError)
    }};
}

/// Writes/executes the given command.
#[doc(hidden)]
#[macro_export]
macro_rules! handle_command {
    ($writer:expr, $command:expr) => {{
        // Silent warning when the macro is used inside the `command` module
        #[allow(unused_imports)]
        use $crate::{write_ansi_code, Command};

        #[cfg(windows)]
        {
            let command = $command;
            if command.is_ansi_code_supported() {
                write_ansi_code!($writer, command.ansi_code())
            } else {
                command.execute_winapi().map_err($crate::ErrorKind::from)
            }
        }
        #[cfg(unix)]
        {
            write_ansi_code!($writer, $command.ansi_code())
        }
    }};
}

/// Queues one or more command(s) for further execution.
///
/// Queued commands must be flushed to the underlying device to be executed.
/// This generally happens in the following cases:
///
/// * When `flush` is called manually on the given type implementing `io::Write`.
/// * The terminal will `flush` automatically if the buffer is full.
/// * Each line is flushed in case of `stdout`, because it is line buffered.
///
/// # Arguments
///
/// - [std::io::Writer](https://doc.rust-lang.org/std/io/trait.Write.html)
///
///     ANSI escape codes are written on the given 'writer', after which they are flushed.
///
/// - [Command](./trait.Command.html)
///
///     One or more commands
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
/// Have a look over at the [Command API](./#command-api) for more details.
///
/// # Notes
///
/// In case of Windows versions lower than 10, a direct WinApi call will be made.
/// The reason for this is that Windows versions lower than 10 do not support ANSI codes,
/// and can therefore not be written to the given `writer`.
/// Therefore, there is no difference between [execute](macro.execute.html)
/// and [queue](macro.queue.html) for those old Windows versions.
///
#[macro_export]
macro_rules! queue {
    ($writer:expr $(, $command:expr)* $(,)?) => {
        Ok(()) $(
            .and_then(|()| $crate::handle_command!($writer, $command))
        )*
    }
}

/// Executes one or more command(s).
///
/// # Arguments
///
/// - [std::io::Writer](https://doc.rust-lang.org/std/io/trait.Write.html)
///
///     ANSI escape codes are written on the given 'writer', after which they are flushed.
///
/// - [Command](./trait.Command.html)
///
///     One or more commands
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
/// Have a look over at the [Command API](./#command-api) for more details.
///
/// # Notes
///
/// * In the case of UNIX and Windows 10, ANSI codes are written to the given 'writer'.
/// * In case of Windows versions lower than 10, a direct WinApi call will be made.
///     The reason for this is that Windows versions lower than 10 do not support ANSI codes,
///     and can therefore not be written to the given `writer`.
///     Therefore, there is no difference between [execute](macro.execute.html)
///     and [queue](macro.queue.html) for those old Windows versions.
#[macro_export]
macro_rules! execute {
    ($writer:expr $(, $command:expr)* $(,)? ) => {
        // Queue each command, then flush
        $crate::queue!($writer $(, $command)*).and_then(|()| {
            $writer.flush().map_err($crate::ErrorKind::IoError)
        })
    }
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
    // TODO: windows tests. This will involve mocking up a struct that
    // gets modified by our faked execute_winapi function.

    #[cfg(not(windows))]
    mod unix {
        use std::io::{self, Write};
        use std::str;

        use crate::command::Command;

        pub struct FakeCommand;

        impl Command for FakeCommand {
            type AnsiType = &'static str;

            fn ansi_code(&self) -> Self::AnsiType {
                "cmd"
            }
        }

        // Helper for execute tests to confirm flush
        #[derive(Default)]
        pub struct FakeWrite {
            buffer: String,
            flushed: bool,
        }

        impl io::Write for FakeWrite {
            fn write(&mut self, content: &[u8]) -> io::Result<usize> {
                let content = str::from_utf8(content)
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
                self.buffer.push_str(content);
                self.flushed = false;
                Ok(content.len())
            }

            fn flush(&mut self) -> io::Result<()> {
                self.flushed = true;
                Ok(())
            }
        }

        #[test]
        fn test_queue_one() {
            let mut result = FakeWrite::default();
            queue!(&mut result, FakeCommand).unwrap();
            assert_eq!(&result.buffer, "cmd");
            assert!(!result.flushed);
        }

        #[test]
        fn test_queue_many() {
            let mut result = FakeWrite::default();
            queue!(&mut result, FakeCommand, FakeCommand).unwrap();
            assert_eq!(&result.buffer, "cmdcmd");
            assert!(!result.flushed);
        }

        #[test]
        fn test_queue_trailing_comma() {
            let mut result = FakeWrite::default();
            queue!(&mut result, FakeCommand, FakeCommand,).unwrap();
            assert_eq!(&result.buffer, "cmdcmd");
            assert!(!result.flushed);
        }

        #[test]
        fn test_execute_one() {
            let mut result = FakeWrite::default();
            execute!(&mut result, FakeCommand).unwrap();
            assert_eq!(&result.buffer, "cmd");
            assert!(result.flushed);
        }

        #[test]
        fn test_execute_many() {
            let mut result = FakeWrite::default();
            execute!(&mut result, FakeCommand, FakeCommand).unwrap();
            assert_eq!(&result.buffer, "cmdcmd");
            assert!(result.flushed);
        }

        #[test]
        fn test_execute_trailing_comma() {
            let mut result = FakeWrite::default();
            execute!(&mut result, FakeCommand, FakeCommand,).unwrap();
            assert_eq!(&result.buffer, "cmdcmd");
            assert!(result.flushed);
        }
    }
}
