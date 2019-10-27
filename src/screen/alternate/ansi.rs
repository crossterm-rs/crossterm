use crate::{csi, write_cout};
use crate::utils::Result;

use super::AlternateScreen;

pub(crate) static ENTER_ALTERNATE_SCREEN_CSI_SEQUENCE: &'static str = csi!("?1049h");
pub(crate) static LEAVE_ALTERNATE_SCREEN_CSI_SEQUENCE: &'static str = csi!("?1049l");

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
