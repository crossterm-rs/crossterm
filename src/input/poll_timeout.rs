use std::time::{Duration, Instant};

/// Keeps track of the elapsed time since the moment the polling started.
pub struct PollTimeOut {
    total_timeout: Option<Duration>,
    left_timeout: Option<Duration>,
    start: Instant,
}

impl PollTimeOut {
    /// Constructs a new `PollTimeout` with the given optional `Duration`.
    pub fn new(timeout: Option<Duration>) -> PollTimeOut {
        PollTimeOut {
            total_timeout: timeout.clone(),
            left_timeout: timeout.clone(),
            start: Instant::now(),
        }
    }

    /// Returns whether the poll duration has elapsed.
    /// This will always return `false` in case that the duration is `None`.
    pub fn elapsed(&mut self) -> bool {
        if let Some(timeout) = self.total_timeout {
            let elapsed = self.start.elapsed();

            if elapsed >= timeout {
                return false;
            }

            self.left_timeout = Some(timeout - elapsed);

            return true;
        }
        return false;
    }

    /// Returns the remaining duration.
    pub fn left_over(&self) -> Option<Duration> {
        self.left_timeout
    }
}
