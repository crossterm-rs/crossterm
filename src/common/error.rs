//! Module containing error handling logic.

use std::{
    fmt::{self, Display, Formatter},
    io,
};

/// The `crossterm` result type.
pub type Result<T> = std::result::Result<T, ErrorKind>;

/// Wrapper for all errors who could occur in `crossterm`.
#[derive(Debug)]
pub enum ErrorKind {
    IoError(io::Error),
    FmtError(fmt::Error),
    ResizingTerminalFailure(String),

    #[doc(hidden)]
    __Nonexhaustive,
}

impl std::error::Error for ErrorKind {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            ErrorKind::IoError(ref e) => Some(e),
            _ => None,
        }
    }
}

impl Display for ErrorKind {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            ErrorKind::IoError(_) => write!(fmt, "IO-error occurred"),
            ErrorKind::ResizingTerminalFailure(_) => write!(fmt, "Cannot resize the terminal"),
            _ => write!(fmt, "Some error has occurred"),
        }
    }
}

impl From<io::Error> for ErrorKind {
    fn from(e: io::Error) -> ErrorKind {
        ErrorKind::IoError(e)
    }
}

impl From<fmt::Error> for ErrorKind {
    fn from(e: fmt::Error) -> ErrorKind {
        ErrorKind::FmtError(e)
    }
}
