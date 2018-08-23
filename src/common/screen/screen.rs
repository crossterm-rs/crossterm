use super::{AlternateScreen,RawScreen};
use TerminalOutput;

use std::io::Write;
use std::io::Result;
use std::sync::Arc;

/// This type represents an screen.
/// This screen has an stdout which is used by the program to write to or to execute commands with.
///
/// You have to make sure that you pass the correct `Screen` to the modules `cursor, terminal, color, input, style`.
/// Most of the time you just have one screen so you could get an instance of that screen with: `Screen::default()`.
///
/// Also this screen has an buffer where you can write to. When you want to write the buffer to the screen you could flush the screen.
///
/// #Example
///
/// ```rust
/// // create default screen.
/// let screen = Screen::default();
/// // create raw screen.
/// let mut screen = Screen::new(true);
///
/// // write some text to the internal buffer of this type.
/// screen.write(b"Some text");
/// screen.write(b"Some more text");
/// screen.write(b"Some more text");
///
/// // write the above text by flushing the internal buffer of this type.
/// screen.flush();
///
/// let screen = Screen::new(false);
///
/// // create raw alternate screen from normal screen.
/// if let Ok(alternate_screen) = screen.enable_alternate_modes(true)
/// {
///     let crossterm = Crossterm::new(&alternate_screen.screen);
///
///     // make sure to pass in the screen of the AlternateScreen.
///     crossterm.cursor();
/// }
/// ```
///
pub struct Screen
{
    buffer: Vec<u8>,
    pub stdout: Arc<TerminalOutput>,
    drop: bool,
}

impl Screen
{
    /// Create new instance of the Screen also specify if the current screen should be in raw mode or normal mode. Check out `RawScreen` type for more info.
    /// If you are not sure what raw mode is then pass false or use the `Screen::default()` to create an instance.
    pub fn new(raw_mode: bool) -> Screen
    {
        if raw_mode
        {
            let screen = Screen { stdout: Arc::new(TerminalOutput::new(true)), buffer: Vec::new() };
            RawScreen::into_raw_mode();
            return screen;
        }

        return Screen::default();
    }

    /// Switch to alternate screen. This function will return an `AlternateScreen` instance if everything went well this type will give you control over the `AlternateScreen`.
    ///
    /// # What is Alternate screen?
    /// *Nix style applications often utilize an alternate screen buffer, so that they can modify the entire contents of the buffer, without affecting the application that started them.
    /// The alternate buffer is exactly the dimensions of the window, without any scrollback region.
    /// For an example of this behavior, consider when vim is launched from bash.
    /// Vim uses the entirety of the screen to edit the file, then returning to bash leaves the original buffer unchanged.
    pub fn enable_alternate_modes(&self, raw_mode: bool) -> Result<AlternateScreen> {
        let stdout = TerminalOutput::new(raw_mode);

        let alternate_screen = AlternateScreen::to_alternate_screen(stdout, raw_mode)?;
        return Ok(alternate_screen);
    }

    /// Write buffer to an internal buffer. When you want to write the buffer to screen use `flush()`.
   ///
   /// This function is useful if you want to build up some output and when you are ready you could flush the output to the screen.
    pub fn write_buf(&mut self, buf: &[u8]) -> Result<usize> {
        self.buffer.write(buf);
        Ok(buf.len())
    }

    /// Flush the internal buffer to the screen.
    pub fn flush_buf(&mut self) -> Result<()> {
        self.stdout.write_buf(&self.buffer);
        self.stdout.flush()
    }

    // this will disable the drop which will cause raw modes not to be undone on drop of `Screen`.
    pub fn disable_drop(&self)
    {
        self.drop = false;
    }
}

impl From<TerminalOutput> for Screen
{
    /// Create an screen with the given `Stdout`
    fn from(stdout: TerminalOutput) -> Self {
        return Screen { stdout: Arc::new(stdout), buffer: Vec::new(), drop: true};
    }
}

impl From<Arc<TerminalOutput>> for Screen
{
    /// Create an screen with the given 'Arc<Stdout>'
    fn from(stdout: Arc<TerminalOutput>) -> Self {
        return Screen { stdout: stdout, buffer: Vec::new() drop: true};
    }
}

impl Default for Screen
{
    /// Create an new screen which will not be in raw mode or alternate mode.
    fn default() -> Self {
        return Screen { stdout: Arc::new(TerminalOutput::new(false)), buffer: Vec::new(), drop: true};
    }
}

impl Drop for Screen
{
    /// If the current screen is in raw mode whe need to disable it when the instance goes out of scope.
    fn drop(&mut self) {
        if self.stdout.is_in_raw_mode && self.drop
        {
            RawScreen::disable_raw_modes();
        }
    }
}

impl Write for Screen
{
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.stdout.write_buf(buf)
    }

    fn flush(&mut self) -> Result<()> {
        self.stdout.flush()
    }
}