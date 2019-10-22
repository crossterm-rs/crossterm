use crossterm_winapi::{Handle, ScreenBuffer};

use crate::utils::Result;

use super::AlternateScreen;

pub(crate) struct WinApiAlternateScreen;

impl AlternateScreen for WinApiAlternateScreen {
    fn enter(&self) -> Result<()> {
        let alternate_screen = ScreenBuffer::create();
        alternate_screen.show()?;
        Ok(())
    }

    fn leave(&self) -> Result<()> {
        let screen_buffer = ScreenBuffer::from(Handle::output_handle()?);
        screen_buffer.show()?;
        Ok(())
    }
}
