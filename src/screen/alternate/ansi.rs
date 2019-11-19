use crate::{csi, utils::Result, write_cout};

use super::AlternateScreen;

pub(crate) const ENTER_ALTERNATE_SCREEN_CSI_SEQUENCE: &str = csi!("?1049h");
pub(crate) const LEAVE_ALTERNATE_SCREEN_CSI_SEQUENCE: &str = csi!("?1049l");

pub(crate) struct AnsiAlternateScreen;

impl AlternateScreen for AnsiAlternateScreen {
    fn enter(&self) -> Result<()> {
        write_cout!(ENTER_ALTERNATE_SCREEN_CSI_SEQUENCE)?;
        Ok(())
    }

    fn leave(&self) -> Result<()> {
        write_cout!(LEAVE_ALTERNATE_SCREEN_CSI_SEQUENCE)?;
        Ok(())
    }
}
