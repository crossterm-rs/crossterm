///! Notice that this feature is not used. But will be implemented later.

use super::kernel;

/// Enables ansi for windows terminals.
pub fn enable_ansi_support() {
    let enable_ansi_code: u32 = 7;
    kernel::set_console_mode(enable_ansi_code);
}