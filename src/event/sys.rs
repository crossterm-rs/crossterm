#[cfg(all(unix, feature = "event-stream"))]
pub(crate) use unix::Waker;
#[cfg(all(windows, feature = "event-stream"))]
pub(crate) use windows::Waker;

#[cfg(unix)]
pub(crate) mod unix;
#[cfg(windows)]
pub(crate) mod windows;
