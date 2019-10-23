use crate::InputEvent;

pub mod fake;
#[cfg(unix)]
pub mod tty;
#[cfg(windows)]
pub mod winapi;

pub trait EventSource: Sync + Send {
    fn read_event(&mut self) -> crate::Result<Option<InputEvent>>;
}
