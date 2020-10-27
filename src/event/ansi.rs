//! This module provides input related ANSI escape codes.

use crate::csi;

pub(crate) const ENABLE_MOUSE_MODE_CSI_SEQUENCE: &str = concat!(
    // Normal tracking: Send mouse X & Y on button press and release
    csi!("?1000h"),
    // Button-event tracking: Report button motion events (dragging)
    csi!("?1002h"),
    // Any-event tracking: Report all motion events
    csi!("?1003h"),
    // RXVT mouse mode: Allows mouse coordinates of >223
    csi!("?1015h"),
    // SGR mouse mode: Allows mouse coordinates of >223, preferred over RXVT mode
    csi!("?1006h"),
);

pub(crate) const DISABLE_MOUSE_MODE_CSI_SEQUENCE: &str = concat!(
    // The above, in reverse order.
    csi!("?1006l"),
    csi!("?1015l"),
    csi!("?1003l"),
    csi!("?1002l"),
    csi!("?1000l"),
);
