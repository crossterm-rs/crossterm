use std;
use std::fmt;
use std::io::Write;

use terminal_style as style;

/// This is used to make StyledObject able to be displayed.
/// This macro will set the styled stored in Styled Object 
#[macro_export] 
macro_rules! impl_fmt
{
    ($name:ident) => {
        impl<D: fmt::$name> fmt::$name for style::StyledObject<D> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result 
            {
                let mut colored_terminal = style::colored_terminal();                                
                let mut reset = true;

                if let Some(bg) = self.object_style.bg_color
                {
                    colored_terminal.set_bg(bg);
                    reset = true;
                }
                if let Some(fg) = self.object_style.fg_color
                {
                   colored_terminal.set_fg(fg);
                   reset = true;
                }
                
                write!(f, "{}", &self.content);
                std::io::stdout().flush().expect("Flush stdout failed");

                if reset
                {
                    colored_terminal.reset();
                }

                Ok(())
            }
        }
    }
}

/// This macro will take an ANSI input and combines it with some default ANSI characters and returns the result
#[macro_export] 
macro_rules! csi {
    ($( $l:expr ),*) => { concat!("\x1B[", $( $l ),*) };
}

/// This inplements Display for StyledObject
/// Notice that more implementations can be maked.
/// # Example
/// ```rust
/// example impl_fmt!(Debug);
/// ```
impl_fmt!(Display);