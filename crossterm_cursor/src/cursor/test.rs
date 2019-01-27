use super::AnsiCursor;
use super::ITerminalCursor;

/* ======================== WinApi =========================== */
#[cfg(windows)]
mod winapi_tests {

    use super::super::WinApiCursor;
    use super::*;
    #[test]
    fn goto_winapi() {
        let cursor = WinApiCursor::new();

        cursor.goto(5, 5, &None);
        let (x, y) = cursor.pos();

        assert_eq!(x, 5);
        assert_eq!(y, 5);
    }

    #[test]
    fn reset_safe_winapi() {
        let cursor = WinApiCursor::new();
        let (x, y) = cursor.pos();

        cursor.save_position(&None);
        cursor.goto(5, 5, &None);
        cursor.reset_position(&None);

        let (x_saved, y_saved) = cursor.pos();

        assert_eq!(x, x_saved);
        assert_eq!(y, y_saved);
    }
}

/* ======================== ANSI =========================== */
#[test]
fn reset_safe_ansi() {
    if try_enable_ansi() {
        let cursor = AnsiCursor::new();
        let (x, y) = cursor.pos();

        cursor.save_position(&None);
        cursor.goto(5, 5, &None);
        cursor.reset_position(&None);

        let (x_saved, y_saved) = cursor.pos();

        assert_eq!(x, x_saved);
        assert_eq!(y, y_saved);
    }
}

#[test]
fn goto_ansi() {
    if try_enable_ansi() {
        let cursor = AnsiCursor::new();
        cursor.goto(5, 5, &None);
        let (x, y) = cursor.pos();

        assert_eq!(x, 5);
        assert_eq!(y, 5);
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
                Err(e) => return false,
            }
        }
    }

    return true;
}
