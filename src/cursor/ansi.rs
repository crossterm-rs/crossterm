//! This module provides cursor related ANSI escape codes.

use crate::csi;

pub(crate) fn move_to_csi_sequence(x: u16, y: u16) -> String {
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

pub(crate) fn move_to_column_csi_sequence(count: u16) -> String {
    format!(csi!("{}G"), count)
}

pub(crate) fn move_to_previous_line_csi_sequence(count: u16) -> String {
    format!(csi!("{}F"), count)
}

pub(crate) fn move_to_next_line_csi_sequence(count: u16) -> String {
    format!(csi!("{}E"), count)
}

pub(crate) const SAVE_POSITION_CSI_SEQUENCE: &str = "\x1B7";
pub(crate) const RESTORE_POSITION_CSI_SEQUENCE: &str = "\x1B8";
pub(crate) const HIDE_CSI_SEQUENCE: &str = csi!("?25l");
pub(crate) const SHOW_CSI_SEQUENCE: &str = csi!("?25h");
pub(crate) const ENABLE_BLINKING_CSI_SEQUENCE: &str = csi!("?12h");
pub(crate) const DISABLE_BLINKING_CSI_SEQUENCE: &str = csi!("?12l");
