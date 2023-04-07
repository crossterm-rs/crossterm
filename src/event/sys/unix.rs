#[cfg(feature = "event-stream")]
#[cfg(feature = "events")]
pub(crate) mod waker;

pub(crate) mod file_descriptor;
#[cfg(feature = "events")]
pub(crate) mod parse;
