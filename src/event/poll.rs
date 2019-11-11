use std::time::Duration;

use crate::Result;

use super::filter::Filter;

/// An interface for polling event readiness and reading events.
pub(crate) trait EventPoll {
    type Output;

    fn poll(&mut self, timeout: Option<Duration>) -> Result<bool>;

    fn read(&mut self, mask: impl Filter) -> Result<Self::Output>;
}
