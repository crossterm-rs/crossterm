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

        let mut total_size = 0;

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

/// Schedule one or more commands to be executed in the near future.
/// You are able to pass in a custom writer that implements `std::io::Write`.
/// This writer will be used to write the ANSI commands to so that you are in controll on when to execute the ANSI commands.
/// If no writer is passed the default stdout will be used.
///
/// The executing can happen in two cases:
/// - When you manually flush the writer
/// - When the buffer is to full, and the terminal will flush for you
///
/// # Example
/// ```rust
/// // to be done
/// ```
///
/// # How it works
/// In the case of UNIX and windows 10, ANSI codes are written to the given 'writer'.
/// In case of Windows versions lower than 10, a direct WinApi call will be made if you use this macro.
#[macro_export]
macro_rules! schedule {
    ($write:expr, $($command:expr), *) =>
    {{
        use $crate::write_cout;
        let mut error = None;

        $(
            #[cfg(windows)]
             {
                if $crate::supports_ansi() {
                    match write!($write, "{}",$command.get_ansi_code()) {
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
                match write!($write, "{}",$command.get_ansi_code()) {
                    Err(e) => {
                       error = Some(Err($crate::ErrorKind::from(e)));
                    }
                    _ => {}
                 };
        )*

        if let Some(error) = error {
            error
        }else {
            Ok(())
        }
    }};
    ($($command:expr), *) =>
    {{
       schedule!(::std::io::stdout(), $($command)*)
    }};
}

/// Schedule one or more commands to be executed directly.
/// You are able to pass in a custom writer that implements `std::io::Write`.
/// This writer will be used to write the ANSI commands to so that you are in controll on when to execute the ANSI commands.
/// If no writer is passed the default stdout will be used.
///
/// The executing can happen in two cases:
/// - When you manually flush the writer
/// - When the buffer is to full, and the terminal will flush for you
///
/// # Example
/// ```rust
/// // to be done
/// ```
///
/// # How it works
/// In the case of UNIX and windows 10, ANSI codes are written to the given 'writer'.
/// In case of Windows versions lower than 10, a direct WinApi call will be made if you use this macro.
#[macro_export]
macro_rules! execute {
    ($write:expr, $($command:expr), *) =>
    {{
        use $crate::write_cout;
        let mut error = None;

        $(
            #[cfg(windows)]
             {
                if $crate::supports_ansi() {
                     match  write_cout!($write, $command.get_ansi_code()) {
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
                 match  write_cout!($write, $command.get_ansi_code()) {
                    Err(e) => {
                       error = Some(Err($crate::ErrorKind::from(e)));
                    }
                    _ => {}
                 };
        )*

        if let Some(error) = error {
            error
        }else {
            Ok(())
        }
    }};
    ($($command:expr), *) =>
    {{
       schedule!(::std::io::stdout(), $($command)*)
    }};
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
