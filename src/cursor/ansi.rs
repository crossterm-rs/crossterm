//! This module provides cursor related ANSI escape codes.

use crate::csi;

pub(crate) fn goto_csi_sequence(x: u16, y: u16) -> String {
    format!(csi!("{};{}H"), y + 1, x + 1)
}

pub(crate) fn move_up_csi_sequence(count: u16) -> String {
    format!(csi!("{}A"), count)
}

pub(crate) fn move_right_csi_sequence(count: u16) -> String {
    format!(csi!("{}C"), count)
}

pub(crate) fn move_down_csi_sequence(count: u16) -> String {
    format!(csi!("{}B"), count)
}

pub(crate) fn move_left_csi_sequence(count: u16) -> String {
    format!(csi!("{}D"), count)
}

pub(crate) static SAVE_POSITION_CSI_SEQUENCE: &'static str = csi!("s");
pub(crate) static RESTORE_POSITION_CSI_SEQUENCE: &'static str = csi!("u");
pub(crate) static HIDE_CSI_SEQUENCE: &'static str = csi!("?25l");
pub(crate) static SHOW_CSI_SEQUENCE: &'static str = csi!("?25h");
pub(crate) static BLINKING_ON_CSI_SEQUENCE: &'static str = csi!("?12h");
pub(crate) static BLINKING_OFF_CSI_SEQUENCE: &'static str = csi!("?12l");
