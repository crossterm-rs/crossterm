use std::io;

use crate::colors::{ColorScheme, ColorType};

/// Queries the terminal for the RGB values of the given color types.
///
/// Not supported on Windows; always returns an [`io::ErrorKind::Unsupported`] error.
pub fn query_terminal_colors(_colors: &[ColorType]) -> io::Result<Vec<(u8, u8, u8)>> {
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "Querying terminal colors is not implemented for the Windows API.",
    ))
}

/// Queries the terminal for the current color scheme (dark or light mode).
///
/// Not supported on Windows; always returns an [`io::ErrorKind::Unsupported`] error.
pub fn query_color_scheme() -> io::Result<ColorScheme> {
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "Querying the color scheme is not implemented for the Windows API.",
    ))
}
