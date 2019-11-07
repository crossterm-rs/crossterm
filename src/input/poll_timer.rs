use std::time::{Duration, Instant};

/// Keeps track of the elapsed time since the moment the polling started.
pub struct PollTimer {
    total_timeout: Option<Duration>,
    left_timeout: Option<Duration>,
    start: Instant,
}

impl PollTimer {
    /// Constructs a new `PollTimeout` with the given optional `Duration`.
    pub fn new(timeout: Option<Duration>) -> PollTimer {
        PollTimer {
            total_timeout: timeout.clone(),
            left_timeout: timeout.clone(),
            start: Instant::now(),
        }
    }

    /// Returns whether the the poll timer has elapsed the duration.
    /// This wil also calculate the left over poll duration.
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
