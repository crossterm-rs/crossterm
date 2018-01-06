use Construct;
use super::base_terminal::{ClearType, ITerminal};
/// This struct will be used for ansi terminals and unix systems.
pub struct NoTerminal;

impl Construct for NoTerminal {
    fn new() -> Box<NoTerminal> {
        Box::from(NoTerminal {})
    }
}

impl ITerminal for NoTerminal {
    fn clear(&self, clear_type: ClearType) {}

    fn terminal_size(&self) -> Option<(u16, u16)> {
        None
    }

    fn scroll_up(&self, count: i16) {}

    fn scroll_down(&self, count: u16) {}
}
