#[macro_export]
macro_rules! csi {
    ($( $l:expr ),*) => { concat!("\x1B[", $( $l ),*) };
}

/// Write to the current output.
///
/// What do we have to write to the current output?
///
/// This macro is used by different 'ANSI' modules from cursor, color, terminal, input to execute an ANSI escape code on the current output.
/// Those modules are having an `Option` of `TerminalOutput`.
/// When this is `Some` it means that the user specified an output, the output is specified in the cases of `alternate screen` or `raw screen` (not sure what this is? check out the `crossterm_screen` crate).
///
/// In order for the ANSI escape codes to take effect on the alternate screen they should be printed on the output that is in alternate mode.
/// That's exactly what this macro does.
#[macro_export]
macro_rules! write_cout {
    ($string:expr) => {
        {
           let mut size = 0;

            let result = write!(::std::io::stdout(), "{}", $string);

            match result {
                Ok(size) => size,
                Err(e) => return Err(crossterm_utils::ErrorKind::IoError(e))
            };

            match ::std::io::stdout().flush() {
                Ok(_) => Ok(size),
                Err(e) => Err(crossterm_utils::ErrorKind::IoError(e)),
            }
        }
    };
}

use std::io::Write;
