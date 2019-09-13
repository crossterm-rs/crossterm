/// Append a the first few characters of an ANSI escape code to the given string.
#[macro_export]
macro_rules! csi {
    ($( $l:expr ),*) => { concat!("\x1B[", $( $l ),*) };
}

/// Write a string to standard output whereafter the stdout will be flushed.
#[macro_export]
macro_rules! write_cout {
    ($write:expr, $string:expr) => {{
        use $crate::ErrorKind;

        if let Err(e) = write!($write, "{}", $string) {
            Err(ErrorKind::IoError(e))
        } else {
            match $write.flush() {
                Ok(size) => Ok(size),
                Err(e) => Err(ErrorKind::IoError(e)),
            }
        }
    }};
    ($string:expr) => {{
        write_cout!(::std::io::stdout(), $string)
    }};
}

/// Queue one or more command(s) for execution in the near future.
///
/// Queued commands will be executed in the following cases:
/// - When you manually call `flush` on the given writer.
/// - When the buffer is to full, then the terminal will flush for you.
/// - Incase of `stdout` each line, because `stdout` is line buffered.
///
/// Check [here](https://timonpost.github.io/crossterm/docs/command.html) for more information and all availible commands.
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
/// extern crate crossterm_cursor;
/// extern crate crossterm_terminal;
///
/// use std::io::{Write, stdout};
///
/// use crossterm_cursor::Goto;
/// use crossterm_terminal::{Clear, ClearType};
/// use crossterm_utils::{queue, Output};
///
/// let mut stdout = stdout();
///
/// // will be executed when flush is called
/// queue!(stdout,  Goto(5, 5), Output("5,5".to_string()));
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
    ($write:expr, $($command:expr), *) => {{
        // Silent warning when the macro is used inside the `command` module
        #[allow(unused_imports)]
        use $crate::Command;
        let mut error = None;

        $(
            #[cfg(windows)]
            {
                if $crate::supports_ansi() {
                    match write!($write, "{}", $command.get_ansi_code()) {
                        Err(e) => {
                            error = Some(Err($crate::ErrorKind::from(e)));
                        }
                        _ => {}
                    };
                } else {
                    match $command.execute_winapi() {
                        Err(e) => {
                            error = Some(Err($crate::ErrorKind::from(e)));
                        }
                        _ => {}
                    };
                };
            }
            #[cfg(unix)]
            match write!($write, "{}", $command.get_ansi_code()) {
                Err(e) => {
                    error = Some(Err($crate::ErrorKind::from(e)));
                }
                _ => {}
             };
        )*

        if let Some(error) = error {
            error
        } else {
            Ok(())
        }
    }}
}

/// Execute one or more command(s)
///
/// Check [here](https://timonpost.github.io/crossterm/docs/command.html) for more information and all availible commands.
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
/// extern crate crossterm_cursor;
/// extern crate crossterm_utils;
/// extern crate crossterm_terminal;
///
/// use std::io::Write;
///
/// use crossterm_terminal::{Clear, ClearType};
/// use crossterm_cursor::Goto;
/// use crossterm_utils::execute;
///
/// // will be executed directly
/// execute!(std::io::stdout(),  Goto(5, 5));
///
/// // will be executed directly
/// execute!(std::io::stdout(),  Goto(10, 10), Clear(ClearType::CurrentLine));
/// ```
///
/// # Remarks
/// - In the case of UNIX and windows 10, ANSI codes are written to the given 'writer'.
/// - In case of Windows versions lower than 10, a direct WinApi call will be made.
/// This is happening because Windows versions lower then 10 do not support ANSI codes, and thus they can't be written to the given buffer.
/// Because of that there is no difference between `execute` and `queue` for those windows versions.
#[macro_export]
macro_rules! execute {
    ($write:expr, $($command:expr), *) => {{
        // Silent warning when the macro is used inside the `command` module
        #[allow(unused_imports)]
        use $crate::{Command, write_cout};
        let mut error = None;

        $(
            #[cfg(windows)]
            {
                if $crate::supports_ansi() {
                    match write_cout!($write, $command.get_ansi_code()) {
                        Err(e) => {
                           error = Some(Err($crate::ErrorKind::from(e)));
                        }
                        _ => {}
                    };
                } else {
                    match $command.execute_winapi() {
                        Err(e) => {
                            error = Some(Err($crate::ErrorKind::from(e)));
                        }
                        _ => {}
                    };
                };
            }
            #[cfg(unix)]
            match write_cout!($write, $command.get_ansi_code()) {
                Err(e) => {
                    error = Some(Err($crate::ErrorKind::from(e)));
                }
                _ => {}
            };
        )*

        if let Some(error) = error {
            error
        } else {
            Ok(())
        }
    }}
}

#[macro_export]
macro_rules! impl_display {
    (for $($t:ty),+) => {
        $(impl ::std::fmt::Display for $t {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::result::Result<(), ::std::fmt::Error> {
                use $crate::Command;
                write!(f, "{}", self.get_ansi_code())
            }
        })*
    }
}
