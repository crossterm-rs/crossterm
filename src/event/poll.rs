use std::time::Duration;

use super::filter::Filter;
use crate::Result;

/// An interface for polling event readiness and reading events.
pub(crate) trait EventPoll {
    type Output;

    fn poll(&mut self, timeout: Option<Duration>) -> Result<bool>;

    fn read(&mut self, mask: impl Filter) -> Result<Self::Output>;
}
