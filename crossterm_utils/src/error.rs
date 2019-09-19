//! Module containing error handling logic.

use std::{
    fmt::{self, Display, Formatter},
    io,
};

/// The `crossterm` result type.
pub type Result<T> = std::result::Result<T, ErrorKind>;

/// Wrapper for all errors that can occur in `crossterm`.
#[derive(Debug)]
pub enum ErrorKind {
    IoError(io::Error),
    FmtError(fmt::Error),
    Utf8Error(std::string::FromUtf8Error),
    ParseIntError(std::num::ParseIntError),
    ResizingTerminalFailure(String),

    #[doc(hidden)]
    __Nonexhaustive,
}

impl std::error::Error for ErrorKind {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ErrorKind::IoError(e) => Some(e),
            ErrorKind::FmtError(e) => Some(e),
            ErrorKind::Utf8Error(e) => Some(e),
            ErrorKind::ParseIntError(e) => Some(e),
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

macro_rules! impl_from {
    ($from:path, $to:expr) => {
        impl From<$from> for ErrorKind {
            fn from(e: $from) -> Self {
                $to(e)
            }
        }
    };
}

impl_from!(io::Error, ErrorKind::IoError);
impl_from!(fmt::Error, ErrorKind::FmtError);
impl_from!(std::string::FromUtf8Error, ErrorKind::Utf8Error);
impl_from!(std::num::ParseIntError, ErrorKind::ParseIntError);
