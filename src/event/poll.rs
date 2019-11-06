use std::time::Duration;

use crate::Result;

/// An interface for polling event readiness and reading events.
pub(crate) trait EventPoll {
    type Output;

    fn poll(&mut self, timeout: Option<Duration>) -> Result<bool>;

    fn read(&mut self) -> Result<Self::Output>;
}
