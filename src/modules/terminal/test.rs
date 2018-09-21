use modules::terminal::winapi_terminal::WinApiTerminal;
use modules::terminal::ansi_terminal::AnsiTerminal;

use modules::terminal::ITerminal;

use Screen;

/* ======================== WinApi =========================== */
#[cfg(windows)]
mod winapi_tests {
    use super::*;

    #[test]
    fn resize_winapi()
    {
        let screen = Screen::default();
        let terminal = WinApiTerminal::new();

        terminal.set_size(10, 10, &screen.stdout);

        let (x, y) = terminal.terminal_size(&screen.stdout);

        assert_eq!(x, 10);
        assert_eq!(y, 10);
    }
}

/* ======================== ANSI =========================== */
#[test]
fn resize_ansi()
{
    if try_enable_ansi() {
        let screen = Screen::default();
        let terminal = WinApiTerminal::new();

        terminal.set_size(10,10, &screen.stdout);

        let (x, y) = terminal.terminal_size(&screen.stdout);

        assert_eq!(x, 10);
        assert_eq!(y, 10);
    }
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
