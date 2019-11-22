use std::time::Duration;

use super::InternalEvent;

#[cfg(unix)]
pub mod unix;
#[cfg(windows)]
pub mod windows;
#[cfg(unix)]
pub use crate::event::sys::unix::{cancellation, CancelRx, CancelTx};
#[cfg(windows)]
pub use crate::event::sys::windows::{cancellation, CancelRx, CancelTx};

/// An interface for trying to read an `InternalEvent` within an optional `Duration`.
pub(crate) trait EventSource: Sync + Send {
    /// Tries to read an `InternalEvent` within the given duration.
    ///
    /// This function takes in an optional duration.
    /// * `None`: blocks indefinitely until an event is able to be read.
    /// * `Some(duration)`: blocks for the given duration.
    ///
    /// Returns:
    /// `Ok(Some(event))`: in case an event is ready.
    /// `Ok(None)`: in case an event is not ready.
    fn try_read(
        &mut self,
        timeout: Option<Duration>,
        cancel: Option<&CancelRx>,
    ) -> crate::Result<Option<InternalEvent>>;
}
