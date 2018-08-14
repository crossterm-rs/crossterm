#[cfg(not(windows))]
use common::commands::unix_command;

#[cfg(windows)]
use common::commands::win_commands;

use common::commands::IAlternateScreenCommand;

use super::{AlternateScreen,RawScreen};
use super::super::super::modules::write::Stdout;

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
    pub stdout: Arc<Stdout>,
}

impl Screen
{
    /// Create new instance of the Screen also specify if the current screen should be in raw mode or normal mode. Check out `RawScreen` type for more info.
    /// If you are not sure what raw mode is then pass false or use the `Screen::default()` to create an instance.
    pub fn new(raw_mode: bool) -> Screen
    {
        if raw_mode
        {
            RawScreen::into_raw_mode();;
            return Screen { stdout: Arc::new(Stdout::new(true)), buffer: Vec::new() };
        }

        return Screen::default();
    }

    /// This method could be used for enabling raw mode for the terminal.
    ///
    /// What exactly is raw state:
    /// - No line buffering.
    ///    Normally the terminals uses line buffering. This means that the input will be send to the terminal line by line.
    ///    With raw mode the input will be send one byte at a time.
    /// - Input
    ///   All input has to be written manually by the programmer.
    /// - Characters
    ///   The characters are not processed by the terminal driver, but are sent straight through.
    ///   Special character have no meaning, like backspace will not be interpret as backspace but instead will be directly send to the terminal.
    /// - Escape characters
    ///   Note that in raw modes `\n` will move to the new line but the cursor will be at the same position as before on the new line therefor use `\n\r` to start at the new line at the first cell.
    ///
    /// With these modes you can easier design the terminal screen.
    pub fn enable_raw_modes(&self) -> Result<()> {
        RawScreen::into_raw_mode()?;
        return Ok(())
    }

    /// Switch to alternate screen. This function will return an `AlternateScreen` instance if everything went well this type will give you control over the `AlternateScreen`.
    ///
    /// # What is Alternate screen?
    /// *Nix style applications often utilize an alternate screen buffer, so that they can modify the entire contents of the buffer, without affecting the application that started them.
    /// The alternate buffer is exactly the dimensions of the window, without any scrollback region.
    /// For an example of this behavior, consider when vim is launched from bash.
    /// Vim uses the entirety of the screen to edit the file, then returning to bash leaves the original buffer unchanged.
    pub fn enable_alternate_modes(&self, raw_mode: bool) -> Result<AlternateScreen> {
        let mut stdout = Stdout::new(raw_mode);

        if raw_mode
        {
            RawScreen::into_raw_mode();
        }

        let alternate_screen = AlternateScreen::to_alternate_screen(stdout)?;
        return Ok(alternate_screen);
    }
}

impl From<Stdout> for Screen
{
    /// Create an screen with the given `Stdout`
    fn from(stdout: Stdout) -> Self {
        return Screen { stdout: Arc::new(stdout), buffer: Vec::new() };
    }
}

impl From<Arc<Stdout>> for Screen
{
    /// Create an screen with the given 'Arc<Stdout>'
    fn from(stdout: Arc<Stdout>) -> Self {
        return Screen { stdout: stdout, buffer: Vec::new() };
    }
}

impl Default for Screen
{
    /// Create an new screen which will not be in raw mode or alternate mode.
    fn default() -> Self {
        return Screen { stdout: Arc::new(Stdout::new(false)), buffer: Vec::new() };
    }
}

impl Drop for Screen
{
    /// If the current screen is in raw mode whe need to disable it when the instance goes out of scope.
    fn drop(&mut self) {
        if self.stdout.is_in_raw_mode
        {
            RawScreen::disable_raw_modes();
        }
    }
}

impl Write for Screen
{
    /// Write buffer to an internal buffer. When you want to write the buffer to screen use `flush()`.
    ///
    /// This function is useful if you want to build up some output and when you are ready you could flush the output to the screen.
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.buffer.write(buf);
        Ok(buf.len())
    }

    /// Flush the internal buffer to the screen.
    fn flush(&mut self) -> Result<()> {
        self.stdout.write_buf(&self.buffer);
        self.stdout.flush()
    }
}