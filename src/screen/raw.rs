use std::io::{Stdout, Write};

use crate::screen::sys;
use crate::utils::Result;

/// A raw screen.
///
/// Be aware that the raw mode is disabled when you drop the `RawScreen` value.
/// Call the [`keep_raw_mode_on_drop`](struct.RawScreen.html#method.keep_raw_mode_on_drop)
/// method to disable this behavior (keep the raw mode enabled).
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// use crossterm::{RawScreen, Result};
///
/// fn main() -> Result<()> {
///     let _raw = RawScreen::into_raw_mode()?;
///     // Do something in the raw mode
///     Ok(())
/// } // `_raw` is dropped here <- raw mode is disabled
/// ```
///
/// Do not disable the raw mode implicitly:
///
/// ```no_run
/// use crossterm::{RawScreen, Result};
///
/// fn main() -> Result<()> {
///     let mut raw = RawScreen::into_raw_mode()?;
///     raw.keep_raw_mode_on_drop();
///     // Feel free to leave `raw` on it's own/drop it, the raw
///     // mode won't be disabled
///
///     // Do something in the raw mode
///
///     // Disable raw mode explicitly
///     RawScreen::disable_raw_mode()
/// }
/// ```
pub struct RawScreen {
    disable_raw_mode_on_drop: bool,
}

impl RawScreen {
    // TODO enable_raw_mode() to keep it synced with enable/disable?
    /// Enables raw mode.
    pub fn into_raw_mode() -> Result<RawScreen> {
        #[cfg(unix)]
        let mut command = sys::unix::RawModeCommand::new();
        #[cfg(windows)]
        let mut command = sys::winapi::RawModeCommand::new();

        command.enable()?;

        Ok(RawScreen {
            disable_raw_mode_on_drop: true,
        })
    }

    /// Disables raw mode.
    pub fn disable_raw_mode() -> Result<()> {
        #[cfg(unix)]
        let mut command = sys::unix::RawModeCommand::new();
        #[cfg(windows)]
        let command = sys::winapi::RawModeCommand::new();

        command.disable()?;
        Ok(())
    }

    /// Keeps the raw mode enabled when `self` is dropped.
    ///
    /// See the [`RawScreen`](struct.RawScreen.html) documentation for more
    /// information.
    pub fn keep_raw_mode_on_drop(&mut self) {
        self.disable_raw_mode_on_drop = false;
    }
}

/// Allows to enable raw mode.
///
/// Why this type must be implemented on writers?
///
/// TTYs has their state controlled by the writer, not the reader. You use the writer to
/// clear the screen, move the cursor and so on, so naturally you use the writer to change
/// the mode as well.
///
/// # Examples
///
/// ```no_run
/// use std::io::stdout;
/// use crossterm::{IntoRawMode, Result};
///
/// fn main() -> Result<()> {
///     let stdout = stdout();
///     let _raw = stdout.into_raw_mode()?;
///
///     // Do something in the raw mode
///
///     Ok(())
/// } // `_raw` dropped here <- raw mode disabled
/// ```
pub trait IntoRawMode: Write + Sized {
    /// Enables raw mode.
    fn into_raw_mode(self) -> Result<RawScreen>;
}

impl IntoRawMode for Stdout {
    fn into_raw_mode(self) -> Result<RawScreen> {
        RawScreen::into_raw_mode()?;
        // this make's sure that raw screen will be disabled when it goes out of scope.
        Ok(RawScreen {
            disable_raw_mode_on_drop: true,
        })
    }
}

impl Drop for RawScreen {
    fn drop(&mut self) {
        if self.disable_raw_mode_on_drop {
            let _ = RawScreen::disable_raw_mode();
        }
    }
}
