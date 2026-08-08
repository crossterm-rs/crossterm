//! `NO_COLOR` is read once per process, so this lives in its own test binary.

use crossterm::style::{
    Attribute, Color, Colors, SetAttribute, SetBackgroundColor, SetColors, SetForegroundColor,
};
use crossterm::Command;

fn ansi_of(command: &impl Command) -> String {
    let mut ansi = String::new();
    command.write_ansi(&mut ansi).unwrap();
    ansi
}

#[test]
fn disabled_colors_emit_nothing_and_keep_attributes() {
    unsafe { std::env::set_var("NO_COLOR", "1") };

    // A bare `CSI m` is the same as `CSI 0 m`, which resets every attribute,
    // so a disabled color must not emit the wrapper at all.
    assert_eq!(ansi_of(&SetForegroundColor(Color::Cyan)), "");
    assert_eq!(ansi_of(&SetBackgroundColor(Color::Reset)), "");
    assert_eq!(
        ansi_of(&SetColors(Colors::new(Color::Cyan, Color::Reset))),
        ""
    );

    // Attributes are not colors and must survive a neighbouring disabled color.
    let reverse = ansi_of(&SetAttribute(Attribute::Reverse));
    assert!(!reverse.is_empty());
    assert_eq!(
        format!("{}{}", reverse, ansi_of(&SetForegroundColor(Color::Cyan))),
        reverse
    );
}
