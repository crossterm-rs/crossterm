use std::{
    io::{self, Write},
    time::Duration,
};

use crate::colors::{ColorEntry, ColorScheme, ColorType};
use crate::event::{
    filter::{ColorQueryFilter, ColorSchemeFilter},
    internal::{self, InternalEvent},
    write_query,
};

/// Queries the terminal for the RGB values of the given color types.
///
/// Returns one `(u8, u8, u8)` per input element, in the same order.
///
/// This function must be called while raw mode is enabled.
pub fn query_terminal_colors(colors: &[ColorType]) -> io::Result<Vec<(u8, u8, u8)>> {
    // Drain stale responses.
    while internal::poll(Some(Duration::ZERO), &ColorQueryFilter)? {
        internal::read(&ColorQueryFilter)?;
    }

    let mut query: Vec<u8> = Vec::new();
    for color in colors {
        let n = color.osc_number();
        match color {
            ColorType::Palette(index) => {
                write!(query, "\x1B]{n};{index};?\x1B\\")?;
            }
            _ => {
                write!(query, "\x1B]{n};?\x1B\\")?;
            }
        }
    }

    // DA1 response arrives after all OSC replies.
    query.extend_from_slice(b"\x1B[c");

    write_query(&query)?;

    let mut results: Vec<(u8, u8, u8)> = Vec::with_capacity(colors.len());
    let timeout = Duration::from_secs(2);

    loop {
        if !internal::poll(Some(timeout), &ColorQueryFilter)? {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "The terminal colors could not be read within a normal duration",
            ));
        }
        match internal::read(&ColorQueryFilter)? {
            InternalEvent::PrimaryDeviceAttributes => {
                if results.len() != colors.len() {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        "The terminal did not respond with all requested colors",
                    ));
                }
                return Ok(results);
            }
            InternalEvent::ColorResponse(ColorEntry {
                color_type,
                r,
                g,
                b,
            }) => {
                if colors.get(results.len()) != Some(&color_type) {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        "The terminal responded incorrectly",
                    ));
                }
                results.push((r, g, b));
            }
            _ => {}
        }
    }
}

/// Queries the terminal for the current color scheme (dark or light mode).
///
/// This function must be called while raw mode is enabled.
pub fn query_color_scheme() -> io::Result<ColorScheme> {
    // Drain stale responses.
    while internal::poll(Some(Duration::ZERO), &ColorSchemeFilter)? {
        internal::read(&ColorSchemeFilter)?;
    }

    // DA1 response arrives after the scheme reply.
    write_query(b"\x1B[?996n\x1B[c")?;

    let timeout = Duration::from_secs(2);
    let mut scheme: Option<ColorScheme> = None;

    loop {
        if !internal::poll(Some(timeout), &ColorSchemeFilter)? {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "The terminal color scheme could not be read within a normal duration",
            ));
        }
        match internal::read(&ColorSchemeFilter)? {
            InternalEvent::PrimaryDeviceAttributes => {
                return scheme.ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::Other,
                        "The terminal did not respond with a color scheme",
                    )
                });
            }
            InternalEvent::ColorSchemeResponse(s) => {
                scheme = Some(s);
            }
            _ => {}
        }
    }
}
