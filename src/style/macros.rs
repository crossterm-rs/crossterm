macro_rules! def_attr {
    ($name:ident => $attr:path) => {
        fn $name(self) -> StyledObject<D> {
            self.attr($attr)
        }
    };
}

macro_rules! def_color {
    ($side:ident: $name:ident => $color:path) => {
        fn $name(self) -> StyledObject<D> {
            StyledObject {
                content_style: ContentStyle {
                    $side: Some($color),
                    ..self.content_style
                },
                ..self
            }
        }
    };
}

macro_rules! def_str_color {
    ($side:ident: $name:ident => $color:path) => {
        fn $name(self) -> StyledObject< &'static str> {
            StyledObject {
                content_style: ContentStyle {
                    $side: Some($color),
                    ..Default::default()
                },
                content: self
            }
        }
    };
}

macro_rules! def_str_attr {
    ($name:ident => $color:path) => {
        fn $name(self) -> StyledObject<&'static str> {
            StyledObject {
                content_style: Default::default(),
                content: self,
            }
        }
    }
}
