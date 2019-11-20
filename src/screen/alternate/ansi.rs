use crate::{csi, utils::Result};

use super::AlternateScreen;
use std::io::{stdout, Write};

pub(crate) const ENTER_ALTERNATE_SCREEN_CSI_SEQUENCE: &str = csi!("?1049h");
pub(crate) const LEAVE_ALTERNATE_SCREEN_CSI_SEQUENCE: &str = csi!("?1049l");

pub(crate) struct AnsiAlternateScreen;

impl AlternateScreen for AnsiAlternateScreen {
    fn enter(&self) -> Result<()> {
        let mut stdout = stdout();
        write!(stdout, "{}", ENTER_ALTERNATE_SCREEN_CSI_SEQUENCE)?;
        stdout.flush()?;
        Ok(())
    }

    fn leave(&self) -> Result<()> {
        let mut stdout = stdout();
        write!(stdout, "{}", LEAVE_ALTERNATE_SCREEN_CSI_SEQUENCE)?;
        stdout.flush()?;
        Ok(())
    }
}
