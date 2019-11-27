use crate::csi;

pub(crate) const ENTER_ALTERNATE_SCREEN_CSI_SEQUENCE: &str = csi!("?1049h");
pub(crate) const LEAVE_ALTERNATE_SCREEN_CSI_SEQUENCE: &str = csi!("?1049l");
