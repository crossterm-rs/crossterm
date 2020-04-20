macro_rules! def_attr {
    ((), $name:ident => $attr:path) => {
        fn $name(self) -> StyledContent<D> {
            self.attribute($attr)
        }
    };
}

macro_rules! def_attr_generic {
    ($attr_ty:ty, $name:ident => $attr:path) => {
        fn $name(self) -> StyledContent<$attr_ty> {
            StyledContent::new(
                ContentStyle {
                    attributes: $attr.into(),
                    ..Default::default()
                },
                self,
            )
        }
    }
}

macro_rules! impl_styler_callback {
    ($callback:ident!($args:tt)) => {
        $callback!($args, reset => Attribute::Reset);
        $callback!($args, bold => Attribute::Bold);
        $callback!($args, underlined => Attribute::Underlined);
        $callback!($args, reverse => Attribute::Reverse);
        $callback!($args, dim => Attribute::Dim);
        $callback!($args, italic => Attribute::Italic);
        $callback!($args, negative => Attribute::Reverse);
        $callback!($args, slow_blink => Attribute::SlowBlink);
        $callback!($args, rapid_blink => Attribute::RapidBlink);
        $callback!($args, hidden => Attribute::Hidden);
        $callback!($args, crossed_out => Attribute::CrossedOut);
    }
}

macro_rules! impl_styler {
    ($impl_ty:ty) => {
        impl Styler<$impl_ty> for $impl_ty {
            impl_styler_callback!(def_attr_generic!($impl_ty));
        }
    };
}

macro_rules! def_color {
    ((), $side:ident: $name:ident => $color:path) => {
        fn $name(self) -> StyledContent<D> {
            StyledContent::new(
                ContentStyle {
                    $side: Some($color),
                    ..self.style
                },
                self.content,
            )
        }
    };
}

macro_rules! def_color_generic {
    ($color_ty:ty, $side:ident: $name:ident => $color:path) => {
        fn $name(self) -> StyledContent<$color_ty> {
            StyledContent::new(
                ContentStyle {
                    $side: Some($color),
                    ..Default::default()
                },
                self,
            )
        }
    };
}

macro_rules! impl_colorize_callback {
    ($callback:ident!($args:tt)) => {
        $callback!($args, foreground_color: black => Color::Black);
        $callback!($args, foreground_color: dark_grey => Color::DarkGrey);
        $callback!($args, foreground_color: red => Color::Red);
        $callback!($args, foreground_color: dark_red => Color::DarkRed);
        $callback!($args, foreground_color: green => Color::Green);
        $callback!($args, foreground_color: dark_green => Color::DarkGreen);
        $callback!($args, foreground_color: yellow => Color::Yellow);
        $callback!($args, foreground_color: dark_yellow => Color::DarkYellow);
        $callback!($args, foreground_color: blue => Color::Blue);
        $callback!($args, foreground_color: dark_blue => Color::DarkBlue);
        $callback!($args, foreground_color: magenta => Color::Magenta);
        $callback!($args, foreground_color: dark_magenta => Color::DarkMagenta);
        $callback!($args, foreground_color: cyan => Color::Cyan);
        $callback!($args, foreground_color: dark_cyan => Color::DarkCyan);
        $callback!($args, foreground_color: white => Color::White);
        $callback!($args, foreground_color: grey => Color::Grey);

        // background colors
        $callback!($args, background_color: on_black => Color::Black);
        $callback!($args, background_color: on_dark_grey => Color::DarkGrey);
        $callback!($args, background_color: on_red => Color::Red);
        $callback!($args, background_color: on_dark_red => Color::DarkRed);
        $callback!($args, background_color: on_green => Color::Green);
        $callback!($args, background_color: on_dark_green => Color::DarkGreen);
        $callback!($args, background_color: on_yellow => Color::Yellow);
        $callback!($args, background_color: on_dark_yellow => Color::DarkYellow);
        $callback!($args, background_color: on_blue => Color::Blue);
        $callback!($args, background_color: on_dark_blue => Color::DarkBlue);
        $callback!($args, background_color: on_magenta => Color::Magenta);
        $callback!($args, background_color: on_dark_magenta => Color::DarkMagenta);
        $callback!($args, background_color: on_cyan => Color::Cyan);
        $callback!($args, background_color: on_dark_cyan => Color::DarkCyan);
        $callback!($args, background_color: on_white => Color::White);
        $callback!($args, background_color: on_grey => Color::Grey);
    };
}

macro_rules! impl_colorize {
    ($impl_ty:ty) => {
        impl Colorize<$impl_ty> for $impl_ty {
            impl_colorize_callback!(def_color_generic!($impl_ty));
        }
    };
}
