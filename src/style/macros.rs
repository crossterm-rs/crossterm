macro_rules! def_attr {
    ($name:ident => $attr:path) => {
        fn $name(self) -> StyledContent<D> {
            self.attribute($attr)
        }
    };
}

macro_rules! def_color {
    ($side:ident: $name:ident => $color:path) => {
        fn $name(self) -> StyledContent<D> {
            StyledContent::new(
                ContentStyle {
                    $side: Some($color),
                    ..self.style
                },
                self.content
            )
        }
    };
}

macro_rules! def_str_color {
    ($side:ident: $name:ident => $color:path) => {
        fn $name(self) -> StyledContent< &'static str> {
            StyledContent::new(
                ContentStyle {
                    $side: Some($color),
                    ..Default::default()
                },
                self
            )
        }
    };
}

macro_rules! def_str_attr {
    ($name:ident => $attr:path) => {
        fn $name(self) -> StyledContent<&'static str> {
            StyledContent::new(
                ContentStyle {
                    attributes: vec![ $attr ],
                    ..Default::default()
                },
                self
            )
        }
    }
}
