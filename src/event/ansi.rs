//! This module provides input related ANSI escape codes.

use crate::csi;

pub(crate) fn enable_mouse_mode_csi_sequence() -> String {
    format!(
        "{}h{}h{}h{}h",
        csi!("?1000"),
        csi!("?1002"),
        csi!("?1015"),
        csi!("?1006")
    )
}

pub(crate) fn disable_mouse_mode_csi_sequence() -> String {
    format!(
        "{}l{}l{}l{}l",
        csi!("?1006"),
        csi!("?1015"),
        csi!("?1002"),
        csi!("?1000")
    )
}
