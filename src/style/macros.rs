//! Defines the macros for repetitive styling implementations

// There's a single core set of macros structure here that's essentially repeated twice; once for
// implementing `Styler` and once for `Colorize`. We'll go through `Styler` as the example, knowing
// that `Colorize` works in precisely the same manner.
//
// There are four macros in each group. For `Styler`, they are:
//  * def_attr_base,
//  * def_attr_generic,
//  * impl_styler_callback,
//  * impl_styler
//
// Fundamentally, any implementation works in a similar fashion; many methods with near-identical
// bodies are grouped together. There are additionally two types of implementors: so-called "base"
// implementors (`char`, `String`, etc.) and a single "generic" implementor - 'StyledContent<D>'.
//
// We can visualize the macro expansion with a sort of pipeline:
//
//                                    /--------> def_attr_base
//   [impl_styler ->] impl_styler_callback
//                                    \--------> def_attr_generic
//
// The code-gen starts at 'impl_styler' for "base" implementors, and at 'impl_styler_callback' for
// `StyledContent<D>`. From there, 'impl_styler_callback' either repeatedly calls 'def_attr_base'
// or 'def_attr_generic' - this is determined by the 'callback' argument.
//
// 'def_attr_base' is used to provide the method bodies for base types, and 'def_attr_generic'
// provides the method bodies for 'StyledContent<D>'.

////////////////////////////////////////////////////////////////////////////////
// `Styler` macros                                                            //
////////////////////////////////////////////////////////////////////////////////

// Produces a single method for a "base" Styler implementation
//
// The first argument is the type for which Styler is being implemented. Because this is the same
// for all "base" types, we can collase them into a single macro.
macro_rules! def_attr_base {
    ($impl_ty:ty, $name:ident => $attr:path) => {
        fn $name(self) -> StyledContent<$impl_ty> {
            StyledContent::new(
                ContentStyle {
                    attributes: $attr.into(),
                    ..Default::default()
                },
                self,
            )
        }
    };
}

// Produces a single method within an implementation of Styler for 'StyledContent<D>'
//
// We give it an empty argument at the start so that it has the same "signature" as
// 'def_attr_base', which takes a type as its first argument
macro_rules! def_attr_generic {
    ((), $name:ident => $attr:path) => {
        fn $name(self) -> StyledContent<D> {
            self.attribute($attr)
        }
    };
}

// Produces the set of methods inside the implementation, but not the outer block itself
//
// 'callback' should be either one of 'def_attr_base' or 'def_attr_generic'. Each expansion of
// 'callback' produces a single method with the name given by the second argument.
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

// Produces the full implementation of Styler for "base" types
//
// This macro is mostly here for convenience; it's nice to not require writing out the
// `impl Styler<..> for ..` for each base type.
macro_rules! impl_styler {
    ($impl_ty:ty) => {
        impl Styler<$impl_ty> for $impl_ty {
            impl_styler_callback!(def_attr_base!($impl_ty));
        }
    };
}

////////////////////////////////////////////////////////////////////////////////
// `Colorize` macros                                                          //
//                                                                            //
// These are effectively the same as the `Styler` macros described above, so  //
// not much detail is repeated here. Where above we have 'def_attr_*', there  //
// is 'def_color_*' here, and 'impl_colorize' takes the place of              //
// 'impl_styler'.                                                             //
////////////////////////////////////////////////////////////////////////////////

macro_rules! def_color_base {
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

macro_rules! def_color_generic {
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

macro_rules! impl_colorize_callback {
    ($callback:ident!($args:tt)) => {
        // foreground colors
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
            impl_colorize_callback!(def_color_base!($impl_ty));
        }
    };
}
