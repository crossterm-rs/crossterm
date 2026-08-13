use crate::style::{Color, Colored};

/// Represents, optionally, a foreground and/or a background color.
///
/// It can be applied using the `SetColors` command.
///
/// It can also be created from a [Colored](enum.Colored.html) value or a tuple of
/// `(Color, Color)` in the order `(foreground, background)`.
///
/// The [then](#method.then) method can be used to combine `Colors` values.
///
/// For example:
/// ```no_run
/// use crossterm::style::{Color, Colors, Colored};
///
/// // An example color, loaded from a config, file in ANSI format.
/// let config_color = "38;2;23;147;209";
///
/// // Default to green text on a black background.
/// let default_colors = Colors::new(Color::Green, Color::Black);
/// // Load a colored value from a config and override the default colors
/// let colors = match Colored::parse_ansi(config_color) {
///     Some(colored) => default_colors.then(&colored.into()),
///     None => default_colors,
/// };
/// ```
///
/// See [Color](enum.Color.html).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Colors {
    pub foreground: Option<Color>,
    pub background: Option<Color>,
}

impl Colors {
    /// Returns a new `Color` which, when applied, has the same effect as applying `self` and *then*
    /// `other`.
    pub fn then(&self, other: &Colors) -> Colors {
        Colors {
            foreground: other.foreground.or(self.foreground),
            background: other.background.or(self.background),
        }
    }
}

impl Colors {
    pub fn new(foreground: Color, background: Color) -> Colors {
        Colors {
            foreground: Some(foreground),
            background: Some(background),
        }
    }
}

impl From<Colored> for Colors {
    fn from(colored: Colored) -> Colors {
        match colored {
            Colored::ForegroundColor(color) => Colors {
                foreground: Some(color),
                background: None,
            },
            Colored::BackgroundColor(color) => Colors {
                foreground: None,
                background: Some(color),
            },
            // `Colors` carries no underline color, so this one cannot be kept.
            // Dropping it loses information, but putting it in `background`
            // would apply a color the caller never asked for.
            Colored::UnderlineColor(_) => Colors {
                foreground: None,
                background: None,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::style::{Color, Colored, Colors};

    #[test]
    fn test_colors_from_colored() {
        assert_eq!(
            Colors::from(Colored::ForegroundColor(Color::Blue)),
            Colors {
                foreground: Some(Color::Blue),
                background: None,
            }
        );

        assert_eq!(
            Colors::from(Colored::BackgroundColor(Color::Blue)),
            Colors {
                foreground: None,
                background: Some(Color::Blue),
            }
        );

        // there is no field to keep an underline color in, but it must not
        // turn into a background color
        assert_eq!(
            Colors::from(Colored::UnderlineColor(Color::Blue)),
            Colors {
                foreground: None,
                background: None,
            }
        );
    }

    #[test]
    fn test_colors_from_parsed_underline_color() {
        // SGR 58 sets an underline color, 59 resets it
        let underline = Colored::parse_ansi("58;5;1").unwrap();
        assert_eq!(underline, Colored::UnderlineColor(Color::DarkRed));
        assert_eq!(
            Colors::from(underline),
            Colors {
                foreground: None,
                background: None,
            }
        );

        let reset = Colored::parse_ansi("59").unwrap();
        assert_eq!(reset, Colored::UnderlineColor(Color::Reset));
        assert_eq!(
            Colors::from(reset),
            Colors {
                foreground: None,
                background: None,
            }
        );

        // the pattern from the `Colors` doc example: reading a color out of a
        // config must not repaint the background when that color is an
        // underline color
        let defaults = Colors::new(Color::Green, Color::Black);
        assert_eq!(defaults.then(&underline.into()), defaults);
    }

    #[test]
    fn test_colors_then() {
        use Color::*;

        assert_eq!(
            Colors {
                foreground: None,
                background: None,
            }
            .then(&Colors {
                foreground: None,
                background: None,
            }),
            Colors {
                foreground: None,
                background: None,
            }
        );

        assert_eq!(
            Colors {
                foreground: None,
                background: None,
            }
            .then(&Colors {
                foreground: Some(Black),
                background: None,
            }),
            Colors {
                foreground: Some(Black),
                background: None,
            }
        );

        assert_eq!(
            Colors {
                foreground: None,
                background: None,
            }
            .then(&Colors {
                foreground: None,
                background: Some(Grey),
            }),
            Colors {
                foreground: None,
                background: Some(Grey),
            }
        );

        assert_eq!(
            Colors {
                foreground: None,
                background: None,
            }
            .then(&Colors::new(White, Grey)),
            Colors::new(White, Grey),
        );

        assert_eq!(
            Colors {
                foreground: None,
                background: Some(Blue),
            }
            .then(&Colors::new(White, Grey)),
            Colors::new(White, Grey),
        );

        assert_eq!(
            Colors {
                foreground: Some(Blue),
                background: None,
            }
            .then(&Colors::new(White, Grey)),
            Colors::new(White, Grey),
        );

        assert_eq!(
            Colors::new(Blue, Green).then(&Colors::new(White, Grey)),
            Colors::new(White, Grey),
        );

        assert_eq!(
            Colors {
                foreground: Some(Blue),
                background: Some(Green),
            }
            .then(&Colors {
                foreground: None,
                background: Some(Grey),
            }),
            Colors {
                foreground: Some(Blue),
                background: Some(Grey),
            }
        );

        assert_eq!(
            Colors {
                foreground: Some(Blue),
                background: Some(Green),
            }
            .then(&Colors {
                foreground: Some(White),
                background: None,
            }),
            Colors {
                foreground: Some(White),
                background: Some(Green),
            }
        );

        assert_eq!(
            Colors {
                foreground: Some(Blue),
                background: Some(Green),
            }
            .then(&Colors {
                foreground: None,
                background: None,
            }),
            Colors {
                foreground: Some(Blue),
                background: Some(Green),
            }
        );

        assert_eq!(
            Colors {
                foreground: None,
                background: Some(Green),
            }
            .then(&Colors {
                foreground: None,
                background: None,
            }),
            Colors {
                foreground: None,
                background: Some(Green),
            }
        );

        assert_eq!(
            Colors {
                foreground: Some(Blue),
                background: None,
            }
            .then(&Colors {
                foreground: None,
                background: None,
            }),
            Colors {
                foreground: Some(Blue),
                background: None,
            }
        );
    }
}
