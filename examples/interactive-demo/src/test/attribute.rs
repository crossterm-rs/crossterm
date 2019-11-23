#![allow(clippy::cognitive_complexity)]

use crate::{MoveCursorToNextLine, Result};
use crossterm::{queue, style};
use std::io::Write;

const ATTRIBUTES: [(style::Attribute, style::Attribute); 6] = [
    (style::Attribute::Bold, style::Attribute::NoBold),
    (style::Attribute::Italic, style::Attribute::NoItalic),
    (style::Attribute::Underlined, style::Attribute::NoUnderline),
    (style::Attribute::Reverse, style::Attribute::NoReverse),
    (
        style::Attribute::CrossedOut,
        style::Attribute::NotCrossedOut,
    ),
    (style::Attribute::SlowBlink, style::Attribute::NoBlink),
];

fn test_set_display_attributes<W>(w: &mut W) -> Result<()>
where
    W: Write,
{
    queue!(
        w,
        style::Print("Display attributes"),
        MoveCursorToNextLine(2)
    )?;

    for (on, off) in &ATTRIBUTES {
        queue!(
            w,
            style::SetAttribute(*on),
            style::Print(format!("{:>width$} ", format!("{:?}", on), width = 35)),
            style::SetAttribute(*off),
            style::Print(format!("{:>width$}", format!("{:?}", off), width = 35)),
            style::ResetColor,
            MoveCursorToNextLine(1)
        )?;
    }

    w.flush()?;

    Ok(())
}

pub fn run<W>(w: &mut W) -> Result<()>
where
    W: Write,
{
    run_tests!(w, test_set_display_attributes,);
    Ok(())
}
