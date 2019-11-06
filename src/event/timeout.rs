use std::time::{Duration, Instant};

/// Keeps track of the elapsed time since the moment the polling started.
pub struct PollTimeout {
    timeout: Option<Duration>,
    start: Instant,
}

impl PollTimeout {
    /// Constructs a new `PollTimeout` with the given optional `Duration`.
    pub fn new(timeout: Option<Duration>) -> PollTimeout {
        PollTimeout {
            timeout,
            start: Instant::now(),
        }
    }

    /// Returns whether the timeout has elapsed.
    ///
    /// It always returns `false` if the initial timeout was set to `None`.
    pub fn elapsed(&mut self) -> bool {
        self.timeout
            .map(|timeout| self.start.elapsed() >= timeout)
            .unwrap_or(false)
    }

    /// Returns the timeout leftover (initial timeout duration - elapsed duration).
    pub fn leftover(&self) -> Option<Duration> {
        self.timeout.map(|timeout| {
            let elapsed = self.start.elapsed();

            if elapsed >= timeout {
                Duration::from_secs(0)
            } else {
                timeout - elapsed
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    use crate::event::timeout::PollTimeout;

    #[test]
    pub fn test_timer_without_duration_should_have_no_leftover() {
        let timer = PollTimeout::new(None);
        assert_eq!(timer.leftover(), None)
    }

    #[test]
    pub fn test_timer_with_duration_should_have_leftover() {
        let timeout = Duration::from_millis(200);

        let timer = PollTimeout::new(Some(timeout.clone()));

        sleep_thread_millis(50);

        let left_over = timer.leftover().unwrap();

        // should be - ~1ms ~150ms, however we can't be sure with the CI because `thread::sleep` might be off by some millis.
        assert!(
            left_over < timeout - Duration::from_millis(50)
                && left_over > Duration::from_millis(140)
        );
    }

    #[test]
    pub fn test_timer_timeout_should_elapse() {
        let mut timer = PollTimeout::new(Some(Duration::from_millis(2)));

        sleep_thread_millis(5);

        assert_eq!(timer.elapsed(), true);
        assert_eq!(timer.leftover(), Some(Duration::from_millis(0)));
    }

    fn sleep_thread_millis(duration: u64) {
        thread::sleep(Duration::from_millis(duration));
    }
}
