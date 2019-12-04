//! This module provides terminal related ANSI escape codes.

use crate::csi;

pub(crate) const CLEAR_ALL_CSI_SEQUENCE: &str = csi!("2J");
pub(crate) const CLEAR_FROM_CURSOR_DOWN_CSI_SEQUENCE: &str = csi!("J");
pub(crate) const CLEAR_FROM_CURSOR_UP_CSI_SEQUENCE: &str = csi!("1J");
pub(crate) const CLEAR_FROM_CURRENT_LINE_CSI_SEQUENCE: &str = csi!("2K");
pub(crate) const CLEAR_UNTIL_NEW_LINE_CSI_SEQUENCE: &str = csi!("K");
pub(crate) const ENTER_ALTERNATE_SCREEN_CSI_SEQUENCE: &str = csi!("?1049h");
pub(crate) const LEAVE_ALTERNATE_SCREEN_CSI_SEQUENCE: &str = csi!("?1049l");

pub(crate) fn scroll_up_csi_sequence(count: u16) -> String {
    format!(csi!("{}S"), count)
}

pub(crate) fn scroll_down_csi_sequence(count: u16) -> String {
    format!(csi!("{}T"), count)
}

pub(crate) fn set_size_csi_sequence(width: u16, height: u16) -> String {
    format!(csi!("8;{};{}t"), height, width)
}
