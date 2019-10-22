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
                object_style: ObjectStyle {
                    $side: Some($color),
                    ..self.object_style
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
                object_style: ObjectStyle {
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
                object_style: Default::default(),
                content: self,
            }
        }
    }
}
