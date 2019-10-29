use std::time::Duration;

use crate::input::events::InternalEvent;

pub mod fake;
#[cfg(unix)]
pub mod tty;
#[cfg(windows)]
pub mod winapi;

// to be implemented
struct InputMask;

pub trait EventSource: Sync + Send {
    /// Block read for input.
    fn read(&mut self) -> crate::Result<Option<InternalEvent>>;

    /// Poll for event readiness.
    fn poll(&mut self, timeout: Option<Duration>) -> crate::Result<bool>;
}
