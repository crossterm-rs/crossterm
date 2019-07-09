/// Append a the first few characters of an ANSI escape code to the given string.

#[macro_export]
macro_rules! csi {
    ($( $l:expr ),*) => { concat!("\x1B[", $( $l ),*) };
}

/// Write a string to standard output whereafter the screen will be flushed.
#[macro_export]
macro_rules! write_cout {
    ($string:expr) => {{
        use $crate::ErrorKind;

        let stdout = ::std::io::stdout();
        let mut stdout = stdout.lock();
        let mut total_size = 0;

        //        let result = stdout.write($string.as_bytes());

        if let Err(e) = write!(stdout, "{}", $string) {
            return Err(ErrorKind::IoError(e));
        }

        match stdout.flush() {
            Ok(size) => Ok(size),
            Err(e) => Err(ErrorKind::IoError(e)),
        }
    }};
}

//fn a () {vec![]}

/// Write a string to standard output whereafter the screen will be flushed.
#[macro_export]
macro_rules! schedule {
    ($write:expr, $($command:expr), *) =>
    {{

            let mut write_ansi =  |mut ansi_code| -> $crate::Result<()> {

            };

        $(
             {
                 #[cfg(windows)]
                if $crate::supports_ansi() {
                     match write!($write, "{}",$command.get_ansi_code()) {
                        Err(e) => {
                           return Err($crate::ErrorKind::from(e))
                        }
                        _ => {}
                     };
                } else {
                  match $command.execute_winapi() {
                    Err(e) => {
                        return Err($crate::ErrorKind::from(e))
                    }
                    _ => {}
                   };
                };

                 #[cfg(unix)]
                 write_ansi($command.get_ansi_code())?;
            }
        )*

        Ok(())
    }};
}
