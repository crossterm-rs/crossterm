use modules::output::winapi_output::WinApiOutput;
use modules::output::ansi_output::AnsiOutput;

use modules::output::IStdout;

use Screen;

#[cfg(windows)]
mod winapi_tests {

    use super::*;
    /* ======================== WinApi =========================== */
    #[test]
    fn write_winapi()
    {
        let screen = Screen::default();
        let output = WinApiOutput::new();

        let bytes = "test".as_bytes();
        let result = output.write(bytes);
        is_valid_write(result, bytes.len());
    }

    #[test]
    fn write_str_winapi()
    {
        let screen = Screen::default();
        let output = WinApiOutput::new();

        let bytes = "test".as_bytes();
        let result = output.write_str("test");
        is_valid_write(result, bytes.len());
    }
}

/* ======================== ANSI =========================== */
#[test]
fn write_ansi()
{
    let screen = Screen::default();
    let output = AnsiOutput::new();

    let bytes = "test".as_bytes();
    let result = output.write(bytes);
    is_valid_write(result, bytes.len());
}

#[test]
fn write_str_ansi()
{
    let screen = Screen::default();
    let output = AnsiOutput::new();

    let bytes = "test".as_bytes();
    let result = output.write_str("test");
    is_valid_write(result, bytes.len());
}

fn is_valid_write(result: ::std::io::Result<usize>, str_length: usize)
{
    match result {
        Err(_) => assert!(false),
        Ok(length) => if str_length == length { assert!(true) } else { assert!(false) }
    };
}

fn try_enable_ansi() -> bool
{
    if cfg!(target_os = "windows") {
        #[cfg(windows)]
        use kernel::windows_kernel::ansi_support::try_enable_ansi_support;

        if !try_enable_ansi_support()
            { return false; }
    }

    return true;
}
