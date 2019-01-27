use super::{AnsiOutput, IStdout, WinApiOutput};

#[cfg(windows)]
mod winapi_tests {
    use super::*;

    /* ======================== WinApi =========================== */
    #[test]
    fn write_winapi() {
        let output = WinApiOutput::new();

        let bytes = "test".as_bytes();
        let result = output.write(bytes);
        is_valid_write(result, bytes.len());
    }

    #[test]
    fn write_str_winapi() {
        let output = WinApiOutput::new();

        let bytes = "test".as_bytes();
        let result = output.write_str("test");
        is_valid_write(result, bytes.len());
    }
}

/* ======================== ANSI =========================== */
#[test]
fn write_ansi() {
    let output = AnsiOutput::new();

    let bytes = "test".as_bytes();
    let result = output.write(bytes);
    is_valid_write(result, bytes.len());
}

#[test]
fn write_str_ansi() {
    let output = AnsiOutput::new();

    let bytes = "test".as_bytes();
    let result = output.write_str("test");
    is_valid_write(result, bytes.len());
}

fn is_valid_write(result: ::std::io::Result<usize>, str_length: usize) {
    match result {
        Err(_) => assert!(false),
        Ok(length) => {
            if str_length == length {
                assert!(true)
            } else {
                assert!(false)
            }
        }
    };
}

fn try_enable_ansi() -> bool {
    #[cfg(windows)]
    {
        if cfg!(target_os = "windows") {
            use crate::sys::winapi::ansi::set_virtual_terminal_processing;

            // if it is not listed we should try with WinApi to check if we do support ANSI-codes.
            match set_virtual_terminal_processing(true) {
                Ok(_) => return true,
                Err(e) => return false,
            }
        }
    }

    return true;
}
