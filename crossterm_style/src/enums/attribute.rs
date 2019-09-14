use std::fmt::Display;

use crossterm_utils::csi;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Enum with the different attributes to style your test.
///
/// There are few things to note:
/// - Not all attributes are supported, some of them are only supported on Windows some only on Unix,
/// and some are only very rarely supported.
/// - I got those attributes, descriptions, supportability from here: https://en.wikipedia.org/wiki/ANSI_escape_code#SGR_(Select_Graphic_Rendition)_parameters
/// - Take note of the fact that when running your program cross-platform that some attributes might not work because of their support.
/// - When an attribute is not supported nothing will happen with the terminal state.
///
/// # Example
/// You can use an attribute in a write statement to apply the attribute to the terminal output.
///
/// ```ignore
/// println!(
///     "{} Underlined {} No Underline",
///     Attribute::Underlined,
///     Attribute::NoUnderline
/// );
/// ```
///
/// You can also call attribute functions on a `&'static str`:
/// ```ignore
/// use Colorizer;
///
/// println!("{}", style("Bold text").bold());
/// println!("{}", style("Underlined text").underlined());
/// println!("{}", style("Negative text").negative());
/// ```
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum Attribute {
    /// All attributes off
    /// [info]: This will reset all current set attributes.
    /// [Supportability]: Windows, UNIX.
    Reset = 0,
    /// Increased Intensity
    /// [info]: This will increase the text sensitivity also known as bold.
    /// [Supportability]: Windows, UNIX.
    Bold = 1,
    /// Decreased Intensity
    /// [info]: This will decrease the text sensitivity also known as bold.
    /// [Supportability]: Windows, UNIX.
    Dim = 2,
    /// Italic Text
    /// [info]: This will make the text italic.
    /// [Supportability]: Not widely supported, sometimes treated as inverse.
    Italic = 3,
    /// This will draw a line under the text.
    /// [info]: An line under a word, especially in order to show its importance.
    /// [Supportability]: Windows, UNIX
    Underlined = 4,
    /// Slow Blinking Text
    /// [info]: Blink Less than 150 per minute.
    /// [Supportability]: UNIX
    SlowBlink = 5,
    /// Slow Blinking Text
    /// [info]: MS-DOS ANSI.SYS; 150+ per minute;
    /// [Supportability]: Not widely supported
    RapidBlink = 6,
    /// Swap foreground and background colors
    /// [info]: swap foreground and background colors
    /// [Supportability]: Windows, UNIX
    Reverse = 7,
    /// Hide text
    /// [info]:
    /// - This will make the text hidden.
    /// - Also known as 'Conceal'
    /// [Supportability]: Windows, UNIX
    Hidden = 8,
    /// Cross-out text
    /// [info]: Characters legible, but marked for deletion.
    /// [Supportability]: UNIX
    CrossedOut = 9,
    /// The Fraktur is a typeface belonging to the group of Gothic typefaces.
    /// [info]: https://nl.wikipedia.org/wiki/Fraktur
    /// [Supportability]: Rarely supported
    Fraktur = 20,
    /// This will turn off the bold attribute.
    /// [info]:
    /// - Double-underline per ECMA-48.
    /// - WikiPedia: https://en.wikipedia.org/wiki/Talk:ANSI_escape_code#SGR_21%E2%80%94%60Bold_off%60_not_widely_supported
    /// - Opposite of `Bold`(1)
    /// [Supportability]: not widely supported
    NoBold = 21,
    /// Normal color or intensity
    /// Neither bold nor faint
    NormalIntensity = 22,
    /// This will turn off the italic attribute.
    /// [info]:
    /// - Not italic, not Fraktur
    /// - Opposite of `Italic`(3)
    /// [Supportability]: Windows, UNIX
    NoItalic = 23,
    /// This will turn off the underline attribute.
    /// [info]:
    /// - Not singly or doubly underlined will be turned off.
    /// - Opposite of `Underlined.`(4)
    /// [Supportability]: Windows, UNIX
    NoUnderline = 24,
    /// This will turn off the blinking attribute
    /// [info]: Opposite of `Slow and Rapid blink.`(5,6)
    /// [Supportability]: Unknown
    NoBlink = 25,
    /// This will turn off the reverse attribute.
    /// [info]: Opposite of `Reverse`(7)
    /// [Supportability]: Windows, unknown
    NoInverse = 27,
    /// This will make the text visible.
    /// [info]: Opposite of `Hidden`(8)
    /// [Supportability]: Unknown
    NoHidden = 28,
    /// This will turn off the crossed out attribute.
    /// [info]: Opposite of `CrossedOut`(9)
    /// [Supportability]: Not widely supported
    NotCrossedOut = 29,
    /// Framed text.
    /// [Supportability]: Not widely supported
    Framed = 51,
    /// This will turn on the encircled attribute.
    Encircled = 52,
    /// This will draw a line at the top of the text.
    /// [info]: Implementation defined (according to standard)
    /// [Supportability]: Unknown
    OverLined = 53,
    /// This will turn off the framed or encircled attribute.
    NotFramedOrEncircled = 54,
    /// This will turn off the overLined attribute.
    /// [info]: Opposite of `OverLined`(7)
    /// [Supportability]: Windows, unknown
    NotOverLined = 55,

    #[doc(hidden)]
    __Nonexhaustive,
}

impl Display for Attribute {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", format!(csi!("{}m"), *self as i16))?;
        Ok(())
    }
}
