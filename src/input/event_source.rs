use std::time::Duration;

use crate::input::events::InternalEvent;

pub mod fake;
#[cfg(unix)]
pub mod tty;
#[cfg(windows)]
pub mod winapi;

/// An interface for trying to read an `InternalEvent` within an optional `Duration`.
pub trait EventSource: Sync + Send {
    /// Tries to read an `InternalEvent` within the given duration.
    ///
    /// This function takes in an optional duration.
    /// * `None`: will block indefinitely until an event is read.
    /// * `Some(duration)`: will block for the given duration.
    ///
    /// Returns:
    /// `Ok(Some(event))`: in case an event is ready.
    /// `Ok(None)`: in case an event is not ready.
    fn try_read(&mut self, timeout: Option<Duration>) -> crate::Result<Option<InternalEvent>>;
}
