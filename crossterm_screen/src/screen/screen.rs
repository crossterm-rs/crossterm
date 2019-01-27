use super::{AlternateScreen, RawScreen};
use crossterm_utils::TerminalOutput;

use std::io::Result;
use std::io::Write;
use std::sync::Arc;

/// This type represents a screen which could be in normal, raw and alternate modes.
///
/// Let's talk about the different modes a bit:
///
/// - Alternate modes:
///
///   *Nix style applications often utilize an alternate screen buffer, so that they can modify the entire contents of the buffer, without affecting the application that started them.
/// The alternate buffer is exactly the dimensions of the window, without any scrollback region.
/// For an example of this behavior, consider when vim is launched from bash.
/// Vim uses the entirety of the screen to edit the file, then returning to bash leaves the original buffer unchanged.
///
/// - RawModes
///     - No line buffering.
///         Normally the terminals use line buffering. This means that the input will be sent to the terminal line by line.
///         With raw mode the input will send one byte at a time.
///     - Input
///          All input has to be written manually by the programmer.
///     - Characters
///         The characters are not processed by the terminal driver but are sent straight through.
///         Special character have no meaning, like backspace will not be interpreted as backspace but instead will be directly sent to the terminal.
///     - Escape characters
///         Note that in raw modes `\n` `\r` will move to the new line but the cursor will be at the same position as before on the new line therefor use `\n\r` to start at the new line at the first cell.
///
/// You have to make sure that you pass the correct `Screen` to the modules `cursor, terminal, color, input, style`.
/// If you switch to alternate screen modes you will get some `Screen` handle back. This `Screen` handle represents the alternate screen.
/// Once you want to do coloring or such you need to pass the `Screen` handle the library so that it could be used for coloring on the right screen.
///
/// # Example
/// ```rust
/// // create default screen (not raw).
/// let screen = Screen::default();
///
/// // create raw screen.
/// let mut screen = Screen::new(true);
///
/// // create a `Screen` with raw modes disabled.
/// let screen = Screen::new(false);
///
/// // create 'raw alternate screen' from normal screen.
/// if let Ok(alternate_screen) = screen.enable_alternate_modes(true)
/// {
///    // 'alternate screen' is an instance which you should use when you want your actions like: coloring and cursor movement happening at the alternate screen.
///    // For that you can use `Crossterm::from_screen(alternate.screen)` so that all modules like: cursor, input, terminal will be executed on alternate screen.
///     let crossterm = Crossterm::from_screen(&alternate_screen.screen);
///     crossterm.cursor();
///     crossterm.terminal();
///
///     // If you want access modules directly without the `Crossterm` type. You should do the following:
///     let cursor = crossterm::cursor::from_screen(&alternate_screen.screen);
///     let terminal = crossterm::terminal::from_screen(&alternate_screen.screen);
///     let input = crossterm::input::from_screen(&alternate_screen.screen);
/// }
/// ```
/// # Remarks
/// Note that using `Screen` is preferred over manually using `AlternateScreen` or `RawScreen`.
pub struct Screen {
    buffer: Vec<u8>,
    pub stdout: Arc<TerminalOutput>,
    drop: bool,
}

impl Screen {
    /// Create a new instance of the Screen also specify if the current screen should be in raw mode or normal mode.
    /// If you are not sure what raw mode is then passed false or use the `Screen::default()` to create an instance.
    pub fn new(raw_mode: bool) -> Screen {
        if raw_mode {
            let screen = Screen {
                stdout: Arc::new(TerminalOutput::new(true)),
                buffer: Vec::new(),
                drop: true,
            };
            RawScreen::into_raw_mode().unwrap();
            return screen;
        }

        Screen::default()
    }

    /// Switch to alternate screen. This function will return an `AlternateScreen` instance. If everything went well this type will give you control over the `AlternateScreen`.
    ///
    /// The bool 'raw_mode' specifies whether the alternate screen should be raw mode or not.
    ///
    /// # What is Alternate screen?
    /// *Nix style applications often utilize an alternate screen buffer, so that they can modify the entire contents of the buffer, without affecting the application that started them.
    /// The alternate buffer is exactly the dimensions of the window, without any scrollback region.
    /// For an example of this behavior, consider when vim is launched from bash.
    /// Vim uses the entirety of the screen to edit the file, then returning to bash leaves the original buffer unchanged.
    pub fn enable_alternate_modes(&self, raw_mode: bool) -> Result<AlternateScreen> {
        let stdout = TerminalOutput::new(raw_mode);

        let alternate_screen = AlternateScreen::to_alternate_screen(stdout, raw_mode)?;
        Ok(alternate_screen)
    }

    /// Write buffer to an internal buffer. When you want to write the buffer to screen use `flush_buf()`.
    ///
    /// This function is useful if you want to build up some output and when you are ready you could flush the output to the screen.
    ///
    /// # Example
    /// ```
    /// // write some text to the internal buffer of this type. Note that this will not be printed until you call `flush_buf`
    /// let screen = Screen::default();
    /// screen.write_buf(b"Some text");
    /// screen.write_buf(b"Some more text");
    /// screen.write_buf(b"Some more text");
    /// ```
    pub fn write_buf(&mut self, buf: &[u8]) -> Result<usize> {
        self.buffer.write(buf)
    }

    /// Flush the internal buffer to the screen.
    pub fn flush_buf(&mut self) -> Result<()> {
        self.stdout.write_buf(&self.buffer)?;
        self.stdout.flush()?;
        self.buffer.clear();
        Ok(())
    }

    /// This will disable the drop which will cause raw modes not to be undone on the drop of `Screen`.
    pub fn disable_drop(&mut self) {
        self.drop = false;
    }
}

impl From<TerminalOutput> for Screen {
    /// Create a screen with the given `Stdout`
    fn from(stdout: TerminalOutput) -> Self {
        Screen {
            stdout: Arc::new(stdout),
            buffer: Vec::new(),
            drop: true,
        }
    }
}

impl From<Arc<TerminalOutput>> for Screen {
    /// Create a screen with the given 'Arc<Stdout>'
    fn from(stdout: Arc<TerminalOutput>) -> Self {
        Screen {
            stdout,
            buffer: Vec::new(),
            drop: true,
        }
    }
}

impl Default for Screen {
    /// Create a new screen which will not be in raw mode or alternate mode.
    fn default() -> Self {
        Screen {
            stdout: Arc::new(TerminalOutput::new(false)),
            buffer: Vec::new(),
            drop: true,
        }
    }
}

impl Drop for Screen {
    /// If the current screen is in raw mode we need to disable it when the instance goes out of scope.
    fn drop(&mut self) {
        if self.stdout.is_in_raw_mode && self.drop {
            RawScreen::disable_raw_modes().unwrap();
        }
    }
}

impl Write for Screen {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.stdout.write_buf(buf)
    }

    fn flush(&mut self) -> Result<()> {
        self.stdout.flush()
    }
}
