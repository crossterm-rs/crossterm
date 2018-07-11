/// Examples of actions that could be performed on the alternatescreen.
/// !! Note that alternate screen only works on Unix and windows 10 systems. I am working on windows 7 support. !!
pub mod alternate_screen;

/// Examples of actions that could be performed on the terminal.
pub mod terminal;

/// Alternate screen is only supported for unix systems. Windows support will come later :).
//#[cfg(target_os = "unix")]
pub mod raw_mode;
