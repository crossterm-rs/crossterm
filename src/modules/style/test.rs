//use modules::style::winapi_color::WinApiColor;
//use modules::style::ansi_color::AnsiColor;
//
//use modules::style::ITerminalColor;
//
//use Screen;
//
//* ======================== WinApi =========================== */
//#[test]
//fn goto_winapi()
//{
//    let screen = Screen::default();
//    let color = WinApiColor::new();
//
//    assert_eq!(x, 5);
//    assert_eq!(y, 5);
//}
//
//
//
//* ======================== ANSI =========================== */
//#[test]
//fn reset_safe_ansi()
//{
//    if try_enable_ansi() {
//        let screen = Screen::default();
//        let cursor = AnsiColor::new();
//
//        assert_eq!(x, x_saved);
//        assert_eq!(y, y_saved);
//    }
//}
//
//
//fn try_enable_ansi() -> bool
//{
//    if cfg!(target_os = "windows") {
//        #[cfg(windows)]
//        use kernel::windows_kernel::ansi_support::try_enable_ansi_support;
//
//        if !try_enable_ansi_support()
//            { return false; }
//    }
//
//    return true;
//}
