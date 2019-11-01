pub(crate) use ansi::AnsiAlternateScreen;
#[cfg(windows)]
pub(crate) use windows::WinApiAlternateScreen;

#[cfg(windows)]
use crate::supports_ansi;
use crate::utils::Result;

pub(crate) mod ansi;
#[cfg(windows)]
pub(crate) mod windows;

pub(crate) trait AlternateScreen: Sync + Send {
    fn enter(&self) -> Result<()>;
    fn leave(&self) -> Result<()>;
}

#[cfg(windows)]
pub(crate) fn alternate_screen() -> Box<dyn AlternateScreen + Send + Sync> {
    if supports_ansi() {
        Box::new(AnsiAlternateScreen)
    } else {
        Box::new(WinApiAlternateScreen)
    }
}

#[cfg(unix)]
pub(crate) fn alternate_screen() -> AnsiAlternateScreen {
    AnsiAlternateScreen
}
