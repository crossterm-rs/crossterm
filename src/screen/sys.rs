#[cfg(unix)]
pub(crate) mod unix;

#[cfg(windows)]
pub(crate) mod winapi;
