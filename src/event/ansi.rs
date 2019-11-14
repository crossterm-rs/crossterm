//! This module provides input related ANSI escape codes.

use crate::csi;

pub(crate) const ENABLE_MOUSE_MODE_CSI_SEQUENCE: &str = concat!(
    csi!("?1000h"),
    csi!("?1002h"),
    csi!("?1015h"),
    csi!("?1006h")
);

pub(crate) const DISABLE_MOUSE_MODE_CSI_SEQUENCE: &str = concat!(
    csi!("?1006l"),
    csi!("?1015l"),
    csi!("?1002l"),
    csi!("?1000l")
);
