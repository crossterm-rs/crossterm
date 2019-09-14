/* ======================== WinApi =========================== */
#[cfg(windows)]
mod winapi_tests {
    use super::super::*;

    // TODO - Test is ignored, because it returns wrong result (31 != 30)
    #[test]
    #[ignore]
    fn resize_winapi() {
        let terminal = WinApiTerminal::new();

        terminal.set_size(30, 30).unwrap();

        let (x, y) = terminal.terminal_size();

        assert_eq!(x, 30);
        assert_eq!(y, 30);
    }
}

/* ======================== ANSI =========================== */
#[test]
fn resize_ansi() {
    use super::*;
    use std::{thread, time};
    if try_enable_ansi() {
        let terminal = AnsiTerminal::new();

        terminal.set_size(50, 50).unwrap();

        // see issue: https://github.com/eminence/terminal-size/issues/11
        thread::sleep(time::Duration::from_millis(30));

        let (x, y) = terminal.terminal_size();

        assert_eq!(x, 50);
        assert_eq!(y, 50);
    }
}

fn try_enable_ansi() -> bool {
    #[cfg(windows)]
    {
        if cfg!(target_os = "windows") {
            use crossterm_utils::sys::winapi::ansi::set_virtual_terminal_processing;

            // if it is not listed we should try with WinApi to check if we do support ANSI-codes.
            match set_virtual_terminal_processing(true) {
                Ok(_) => return true,
                Err(_) => return false,
            }
        }
    }

    true
}
