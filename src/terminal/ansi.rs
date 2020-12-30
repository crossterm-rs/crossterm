//! This module provides terminal related ANSI escape codes.

use std::fmt;

use crate::csi;

pub(crate) const CLEAR_ALL_CSI_SEQUENCE: &str = csi!("2J");
pub(crate) const CLEAR_FROM_CURSOR_DOWN_CSI_SEQUENCE: &str = csi!("J");
pub(crate) const CLEAR_FROM_CURSOR_UP_CSI_SEQUENCE: &str = csi!("1J");
pub(crate) const CLEAR_FROM_CURRENT_LINE_CSI_SEQUENCE: &str = csi!("2K");
pub(crate) const CLEAR_UNTIL_NEW_LINE_CSI_SEQUENCE: &str = csi!("K");
pub(crate) const ENTER_ALTERNATE_SCREEN_CSI_SEQUENCE: &str = csi!("?1049h");
pub(crate) const LEAVE_ALTERNATE_SCREEN_CSI_SEQUENCE: &str = csi!("?1049l");
pub(crate) const DISABLE_LINE_WRAP_CSI_SEQUENCE: &str = csi!("?7l");
pub(crate) const ENABLE_LINE_WRAP_CSI_SEQUENCE: &str = csi!("?7h");

pub(crate) fn scroll_up_csi_sequence(f: &mut impl fmt::Write, count: u16) -> fmt::Result {
    write!(f, csi!("{}S"), count)
}

pub(crate) fn scroll_down_csi_sequence(f: &mut impl fmt::Write, count: u16) -> fmt::Result {
    write!(f, csi!("{}T"), count)
}

pub(crate) fn set_size_csi_sequence(
    f: &mut impl fmt::Write,
    width: u16,
    height: u16,
) -> fmt::Result {
    write!(f, csi!("8;{};{}t"), height, width)
}

pub(crate) fn set_title_ansi_sequence(
    f: &mut impl fmt::Write,
    title: impl fmt::Display,
) -> fmt::Result {
    write!(f, "\x1B]0;{}\x07", title)
}
