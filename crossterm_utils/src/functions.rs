use crate::output::TerminalOutput;
use std::io::{self, Write};
use std::sync::Arc;

#[cfg(windows)]
use crate::sys::winapi::ansi::set_virtual_terminal_processing;

#[cfg(windows)]
/// Get an module specific implementation of a the generic given type based on the current platform.
/// If the current platform is windows and it supports ansi escape codes it will return the ansi implementation and if not it will return the winapi implementation.
/// If the current platform is unix it will return the ansi implementation.
pub fn get_module<T>(winapi_impl: T, ansi_impl: T) -> Option<T> {
    // Some terminals on windows like GitBash can't use WinaApi calls directly so when we try to enable the ANSI-flag for windows this won't work.
    // Because of that we should check first if the TERM-variable is set and see if the current terminal is a terminal who does support ANSI.
    let supports_ansi = is_specific_term();

    match supports_ansi {
        true => {
            return Some(ansi_impl);
        }
        false => {
            // if it is not listed we should try with WinApi to check if we do support ANSI-codes.
            match set_virtual_terminal_processing(true) {
                Ok(_) => {
                    return Some(ansi_impl);
                }
                Err(_) => {
                    return Some(winapi_impl);
                }
            }
        }
    }
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

// checks if the 'TERM' environment variable is set to check if the terminal supports ANSI-codes.
// I got the list of terminals from here: https://github.com/keqingrong/supports-ansi/blob/master/index.js
fn is_specific_term() -> bool {
    const TERMS: [&'static str; 15] = [
        "xterm",  // xterm, PuTTY, Mintty
        "rxvt",   // RXVT
        "eterm",  // Eterm
        "screen", // GNU screen, tmux
        "tmux",   // tmux
        "vt100", "vt102", "vt220", "vt320",   // DEC VT series
        "ansi",    // ANSI
        "scoansi", // SCO ANSI
        "cygwin",  // Cygwin, MinGW
        "linux",   // Linux console
        "konsole", // Konsole
        "bvterm",  // Bitvise SSH Client
    ];

    match std::env::var("TERM") {
        Ok(val) => val != "dumb" || TERMS.contains(&val.as_str()),
        Err(_) => false,
    }
}
