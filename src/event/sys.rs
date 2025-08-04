#[cfg(all(unix, feature = "event-stream"))]
#[cfg(not(feature = "no-tty"))]
pub(crate) use unix::waker::Waker;
#[cfg(all(windows, feature = "event-stream"))]
#[cfg(not(feature = "no-tty"))]
pub(crate) use windows::waker::Waker;

#[cfg(unix)]
pub(crate) mod unix;
#[cfg(windows)]
#[cfg(not(feature = "no-tty"))]
pub(crate) mod windows;
