//! # Colors
//!
//! Terminal capability queries for color values and the color scheme
//! (light vs. dark mode).

/// Terminal color type, used in queries and responses.
///
/// `Palette(n)` uses OSC 4. The remaining variants use OSC 10..=19.
#[derive(Debug, PartialOrd, PartialEq, Hash, Clone, Copy, Eq)]
pub enum ColorType {
    Palette(u8),
    Foreground,
    Background,
    Cursor,
    PointerForeground,
    PointerBackground,
    TektronixForeground,
    TektronixBackground,
    HighlightBackground,
    TektronixCursor,
    HighlightForeground,
}

impl ColorType {
    /// Maps an OSC number (10..=19) to the corresponding `ColorType` variant.
    pub(crate) fn from_osc_number(n: u8) -> Option<Self> {
        match n {
            10 => Some(Self::Foreground),
            11 => Some(Self::Background),
            12 => Some(Self::Cursor),
            13 => Some(Self::PointerForeground),
            14 => Some(Self::PointerBackground),
            15 => Some(Self::TektronixForeground),
            16 => Some(Self::TektronixBackground),
            17 => Some(Self::HighlightBackground),
            18 => Some(Self::TektronixCursor),
            19 => Some(Self::HighlightForeground),
            _ => None,
        }
    }

    /// Returns the OSC number for this color type.
    pub(crate) fn osc_number(&self) -> u8 {
        match self {
            Self::Palette(_) => 4,
            Self::Foreground => 10,
            Self::Background => 11,
            Self::Cursor => 12,
            Self::PointerForeground => 13,
            Self::PointerBackground => 14,
            Self::TektronixForeground => 15,
            Self::TektronixBackground => 16,
            Self::HighlightBackground => 17,
            Self::TektronixCursor => 18,
            Self::HighlightForeground => 19,
        }
    }
}

/// The terminal's color scheme preference (dark or light).
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialOrd, Ord, PartialEq, Hash, Clone, Copy, Eq)]
pub enum ColorScheme {
    Dark,
    Light,
}

/// A parsed color response from the terminal.
#[derive(Debug, PartialOrd, PartialEq, Hash, Clone, Eq)]
pub(crate) struct ColorEntry {
    pub color_type: ColorType,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

/// Query for the RGB value of a single terminal color.
///
/// Use OSC 4 for palette entries and OSC 10..=19 for dynamic colors such as
/// foreground, background, and cursor. Returns `None` if the terminal does not
/// reply.
///
/// Use with [`QueryBatch`](crate::query::QueryBatch):
///
/// ```no_run
/// # #[cfg(unix)] {
/// use crossterm::colors::{ColorType, QueryColor};
/// use crossterm::query::QueryBatch;
///
/// let mut batch = QueryBatch::new();
/// let fg = batch.add(QueryColor(ColorType::Foreground));
/// let bg = batch.add(QueryColor(ColorType::Background));
/// let results = batch.execute()?;
/// println!("fg: {:?}, bg: {:?}", results.get(&fg)?, results.get(&bg)?);
/// # }
/// # Ok::<(), std::io::Error>(())
/// ```
#[cfg(all(unix, feature = "events"))]
#[derive(Clone)]
pub struct QueryColor(pub ColorType);

#[cfg(all(unix, feature = "events"))]
#[allow(private_interfaces)]
impl crate::query::TerminalQuery for QueryColor {
    type Response = Option<(u8, u8, u8)>;

    fn query_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        let n = self.0.osc_number();
        match self.0 {
            ColorType::Palette(index) => {
                use std::io::Write;
                let _ = write!(buf, "\x1B]{n};{index};?\x1B\\");
            }
            _ => {
                use std::io::Write;
                let _ = write!(buf, "\x1B]{n};?\x1B\\");
            }
        }
        buf
    }

    fn matches(&self, event: &crate::event::internal::InternalEvent) -> bool {
        matches!(
            event,
            crate::event::internal::InternalEvent::ColorResponse(entry)
                if entry.color_type == self.0
        )
    }

    fn extract(
        &self,
        event: Option<crate::event::internal::InternalEvent>,
    ) -> std::io::Result<Option<(u8, u8, u8)>> {
        match event {
            Some(crate::event::internal::InternalEvent::ColorResponse(entry)) => {
                Ok(Some((entry.r, entry.g, entry.b)))
            }
            None => Ok(None),
            _ => unreachable!(),
        }
    }
}

/// Query the terminal's current color scheme (dark or light mode).
///
/// Sends DEC private mode report `CSI ? 996 n`; the terminal replies with
/// `CSI ? 997 ; 1 n` (dark) or `CSI ? 997 ; 2 n` (light). Returns `None` if
/// the terminal does not reply.
///
/// Use with [`QueryBatch`](crate::query::QueryBatch):
///
/// ```no_run
/// # #[cfg(unix)] {
/// use crossterm::colors::QueryColorScheme;
/// use crossterm::query::QueryBatch;
///
/// let mut batch = QueryBatch::new();
/// let scheme = batch.add(QueryColorScheme);
/// let results = batch.execute()?;
/// println!("scheme: {:?}", results.get(&scheme)?);
/// # }
/// # Ok::<(), std::io::Error>(())
/// ```
#[cfg(all(unix, feature = "events"))]
#[derive(Clone)]
pub struct QueryColorScheme;

#[cfg(all(unix, feature = "events"))]
#[allow(private_interfaces)]
impl crate::query::TerminalQuery for QueryColorScheme {
    type Response = Option<ColorScheme>;

    fn query_bytes(&self) -> Vec<u8> {
        b"\x1B[?996n".to_vec()
    }

    fn matches(&self, event: &crate::event::internal::InternalEvent) -> bool {
        matches!(
            event,
            crate::event::internal::InternalEvent::ColorSchemeResponse(_)
        )
    }

    fn extract(
        &self,
        event: Option<crate::event::internal::InternalEvent>,
    ) -> std::io::Result<Option<ColorScheme>> {
        match event {
            Some(crate::event::internal::InternalEvent::ColorSchemeResponse(s)) => Ok(Some(s)),
            None => Ok(None),
            _ => unreachable!(),
        }
    }
}
