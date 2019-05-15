/// Append a the first few characters of an ANSI escape code to the given string.
#[macro_export]
macro_rules! csi {
    ($( $l:expr ),*) => { concat!("\x1B[", $( $l ),*) };
}

pub static mut flush_count: u32 = 0;

/// Write a string to standard output whereafter the screen will be flushed.
#[macro_export]
macro_rules! write_cout {
    ($string:expr) => {{
        let stdout = ::std::io::stdout();
        let mut stdout = stdout.lock();
        let mut size = 0;

        let result = stdout.write($string.as_bytes());

        size += match result {
            Ok(size) => size,
            Err(e) => return Err(crossterm_utils::ErrorKind::IoError(e)),
        };

        match stdout.flush() {
            Ok(_) => Ok(size),
            Err(e) => Err(crossterm_utils::ErrorKind::IoError(e)),
        }
    }};
}
