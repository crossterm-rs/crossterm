//! Some actions need to preformed platform independently since they can not be solved `ANSI escape codes`.

use super::TerminalOutput;
use std::io::{self, Write};
use std::sync::Arc;

#[cfg(windows)]
use kernel::windows_kernel::ansi_support::{try_enable_ansi_support, windows_supportable};

#[cfg(windows)]
use kernel::windows_kernel::exit;
#[cfg(windows)]
use kernel::windows_kernel::ScreenBuffer;

#[cfg(windows)]
use kernel::windows_kernel::Cursor;

#[cfg(unix)]
use kernel::unix_kernel::terminal::{exit, pos, terminal_size};

/// Get the terminal size based on the current platform.
#[cfg(unix)]
pub fn get_terminal_size() -> (u16, u16) {
    terminal_size()
}

#[cfg(windows)]
pub fn get_terminal_size() -> (u16, u16) {
    if let Ok(buffer) = ScreenBuffer::current() {
        let size = buffer.info().unwrap().terminal_size();
        (size.width as u16, size.height as u16)
    } else {
        (0, 0)
    }
}

/// Get the cursor position based on the current platform.
#[cfg(unix)]
pub fn get_cursor_position() -> (u16, u16) {
    if let Ok(pos) = pos() {
        pos
    } else {
        (0, 0)
    }
}

#[cfg(windows)]
pub fn get_cursor_position() -> (u16, u16) {
    if let Ok(cursor) = Cursor::new() {
        cursor.position().unwrap().into()
    } else {
        (0, 0)
    }
}

/// exit the current terminal.
pub fn exit_terminal() {
    exit();
}

#[cfg(windows)]
/// Get an module specific implementation of a the generic given type based on the current platform.
/// If the current platform is windows and it supports ansi escape codes it will return the ansi implementation and if not it will return the winapi implementation.
/// If the current platform is unix it will return the ansi implementation.
pub fn get_module<T>(winapi_impl: T, unix_impl: T) -> Option<T> {
    let mut term: Option<T> = None;
    let mut does_support = false;

    if !windows_supportable() {
        //     Try to enable ansi on windows if not than use WINAPI.
        does_support = try_enable_ansi_support();
        //
        //     uncomment this line when you want to use the winapi implementation.
        //        does_support = false;
        if !does_support {
            term = Some(winapi_impl);
        }
    }

    if does_support {
        term = Some(unix_impl);
    }

    term
}

/// This function is used by 'ANSI' modules. Those modules are using an `Option` of `TerminalOutput`.
/// Because it is an option it could be either 'None' or 'Some'.
/// When the `TerminalOutput` is 'None' we write our 'ANSI' escape codes to the default `stdout()` if it is a `Some`
/// - which means we are in alternate screen modes or we have raw screen enabled - we should write to the screen passed by the user.
/// This way our commands or our writes will be done with the passed `TerminalOutput`.
pub fn write(stdout: &Option<&Arc<TerminalOutput>>, string: String) -> io::Result<usize> {
    match stdout {
        None => {
            print!("{}", string.as_str());

            match io::stdout().flush() {
                Ok(_) => Ok(string.len()),
                Err(e) => Err(e),
            }
        }
        Some(output) => output.write_string(string),
    }
}

/// This function is used by 'ANSI' modules. Those modules are using an `Option` of `TerminalOutput`.
/// Because it is an option it could be either 'None' or 'Some'.
/// When the `TerminalOutput` is 'None' we write our 'ANSI' escape codes to the default `stdout()` if it is a `Some`
/// - which means we are in alternate screen modes or we have raw screen enabled - we should write to the screen passed by the user.
/// This way our commands or our writes will be done with the passed `TerminalOutput`.
pub fn write_str(stdout: &Option<&Arc<TerminalOutput>>, string: &str) -> io::Result<usize> {
    match stdout {
        None => match io::stdout().flush() {
            Ok(_) => {
                write!(io::stdout(), "{}", string)?;
                Ok(string.len())
            }
            Err(e) => Err(e),
        },
        Some(output) => output.write_str(string),
    }
}
