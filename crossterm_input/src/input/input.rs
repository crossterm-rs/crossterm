//! A module that contains all the actions related to reading input from the terminal.
//! Like reading a line, reading a character and reading asynchronously.

use super::*;
use std::io::{Error, ErrorKind};
use std::iter::Iterator;
use std::str;

use crossterm_utils::TerminalOutput;

/// Allows you to preform actions with the < option >.
///
/// # Features:
///
/// - features
///
/// Check `/examples/` in the library for more specific examples.
///
/// # Remarks
///
/// When you want to use '< name >' on 'alternate screen' use the 'crossterm_screen' crate.

/// Allows you to read user input.
///
/// # Features:
///
/// - Read character
/// - Read line
/// - Read async
/// - Read async until
/// - Wait for key event (terminal pause)
///
/// Check `/examples/` in the library for more specific examples.
///
/// # Remarks
///
/// When you want to use 'input' on 'alternate screen' use the 'crossterm_screen' crate.
pub struct TerminalInput<'stdout> {
    terminal_input: Box<ITerminalInput + Sync + Send>,
    stdout: Option<&'stdout Arc<TerminalOutput>>,
}

impl<'stdout> TerminalInput<'stdout> {
    /// Create a new instance of `TerminalInput` whereon input related actions could be preformed.
    pub fn new() -> TerminalInput<'stdout> {
        #[cfg(target_os = "windows")]
        let input = Box::from(WindowsInput::new());

        #[cfg(not(target_os = "windows"))]
        let input = Box::from(UnixInput::new());

        TerminalInput {
            terminal_input: input,
            stdout: None,
        }
    }

    /// Create a new instance of `TerminalInput` whereon input related actions could be preformed.
    ///
    /// # Remarks
    ///
    /// Use this function when you want your terminal to operate with a specific output.
    /// This could be useful when you have a screen which is in 'alternate mode',
    /// and you want your actions from the `TerminalInput`, created by this function, to operate on the 'alternate screen'.
    ///
    /// You should checkout the 'crossterm_screen' crate for more information about this.
    /// # Example
    /// ```rust
    /// let screen = Screen::default();
    //
    /// if let Ok(alternate) = screen.enable_alternate_modes(false) {
    ///    let terminal = TerminalInput::from_output(&alternate.screen.stdout);
    /// }
    /// ```
    pub fn from_output(stdout: &'stdout Arc<TerminalOutput>) -> TerminalInput<'stdout> {
        #[cfg(target_os = "windows")]
        let input = Box::from(WindowsInput::new());

        #[cfg(not(target_os = "windows"))]
        let input = Box::from(UnixInput::new());

        TerminalInput {
            terminal_input: input,
            stdout: Some(stdout),
        }
    }

    /// Read one line from the user input.
    ///
    /// # Remark
    /// This function is not work when raw screen is turned on.
    /// When you do want to read a line in raw mode please, checkout `read_async` or `read_async_until`.
    /// Not sure what 'raw mode' is, checkout the 'crossterm_screen' crate.
    ///
    /// # Example
    /// ```rust
    /// let input = input();
    ///  match input.read_line() {
    ///     Ok(s) => println!("string typed: {}", s),
    ///     Err(e) => println!("error: {}", e),
    ///  }
    /// ```
    pub fn read_line(&self) -> io::Result<String> {
        if let Some(stdout) = self.stdout {
            if stdout.is_in_raw_mode {
                return Err(Error::new(ErrorKind::Other, "Crossterm does not support readline in raw mode this should be done instead whit `read_async` or `read_async_until`"));
            }
        }

        let mut rv = String::new();
        io::stdin().read_line(&mut rv)?;
        let len = rv.trim_right_matches(&['\r', '\n'][..]).len();
        rv.truncate(len);
        Ok(rv)
    }

    /// Read one character from the user input
    ///
    /// ```rust
    /// let input = input();
    ///
    ///  match input.read_char() {
    ///     Ok(c) => println!("character pressed: {}", c),
    ///     Err(e) => println!("error: {}", e),
    ///   }
    /// ```
    pub fn read_char(&self) -> io::Result<char> {
        self.terminal_input.read_char(&self.stdout)
    }

    /// Read the input asynchronously from the user.
    ///
    /// # Remarks
    /// - This call will not block the current thread.
    ///   A thread will be fired to read input, on unix systems from TTY and on windows systems with '_getwch' and '_getwche'.
    /// - Requires 'raw screen to be enabled'.
    ///   Not sure what this is, please checkout the 'crossterm_screen' crate.
    ///
    /// ```rust
    /// // we need to enable raw mode otherwise the characters will be outputted by default before we are able to read them.
    /// let screen = Screen::new(true);
    /// let input = crossterm::input::from_screen(&screen);
    ///
    /// let mut stdin = input.read_async().bytes();
    ///
    /// for i in 0..100 {
    ///
    ///     // Get the next character typed. This is None if nothing is pressed. And Some(Ok(u8 value of character))
    ///     let a = stdin.next();
    ///
    ///     println!("pressed key: {:?}", a);
    ///
    ///     if let Some(Ok(b'x')) = a {
    ///         println!("The key: `x` was pressed and program is terminated.");
    ///         break;
    ///     }
    ///     // simulate some timeout so that we can see the character on the screen.
    ///     thread::sleep(time::Duration::from_millis(50));
    /// }
    ///
    /// ```
    pub fn read_async(&self) -> AsyncReader {
        self.terminal_input.read_async(&self.stdout)
    }

    /// Read the input asynchronously until a certain character is hit.
    ///
    /// This is the same as `read_async()` but stops reading when a certain character is hit.
    ///
    /// # Remarks
    /// - This call will not block the current thread.
    ///   A thread will be fired to read input, on unix systems from TTY and on windows systems with '_getwch' and '_getwche'.
    /// - Requires 'raw screen to be enabled'.
    ///   Not sure what this is, please checkout the 'crossterm_screen' crate.
    /// - Thread is automatically destroyed when the 'delimiter' is hit.
    ///
    /// ```rust
    /// // we need to enable raw mode otherwise the characters will be outputted by default before we are able to read them.
    /// let screen = Screen::new(true);
    ///
    /// // create an instance of `Crossterm` which will preform the actions on the raw screen.
    /// let crossterm = Crossterm::from_screen(&screen);
    /// let input = crossterm.input();
    /// let terminal = crossterm.terminal();
    /// let mut cursor = crossterm.cursor();
    ///
    /// let mut stdin = input.read_until_async(b'\r').bytes();
    ///
    /// for i in 0..100 {
    ///     terminal.clear(ClearType::All);
    ///     cursor.goto(1, 1);
    ///     let a = stdin.next();
    ///
    ///     println!("pressed key: {:?}", a);
    ///
    ///     if let Some(Ok(b'\r')) = a {
    ///         println!("The enter key is hit and program is not listening to input anymore.");
    ///         break;
    ///     }
    ///
    ///     if let Some(Ok(b'x')) = a {
    ///          println!("The key: x was pressed and program is terminated.");
    ///         break;
    ///     }
    ///
    ///     thread::sleep(time::Duration::from_millis(100));
    /// }
    /// ```
    pub fn read_until_async(&self, delimiter: u8) -> AsyncReader {
        self.terminal_input
            .read_until_async(delimiter, &self.stdout)
    }

    /// Enable mouse events to be captured.
    ///
    /// ```rust
    /// let input = input();
    /// input.enable_mouse();
    /// ```
    pub fn enable_mouse_mode(&self) -> io::Result<()> {
        self.terminal_input.enable_mouse_mode(&self.stdout)
    }

    /// Disable mouse events to be captured.
    ///
    /// ```rust
    /// let input = input();
    /// input.disable_mouse();
    /// ```
    pub fn disable_mouse_mode(&self) -> io::Result<()> {
        self.terminal_input.disable_mouse_mode(&self.stdout)
    }
}

/// Get a `TerminalInput` instance whereon input related actions can be performed.
pub fn input<'stdout>() -> TerminalInput<'stdout> {
    TerminalInput::new()
}

/// Parse an Event from `item` and possibly subsequent bytes through `iter`.
pub fn parse_event<I>(item: u8, iter: &mut I) -> Result<InputEvent>
where
    I: Iterator<Item = u8>,
{
    let error = Error::new(ErrorKind::Other, "Could not parse an event");
    let input_event = match item {
        b'\x1B' => {
            // This is an escape character, leading a control sequence.
            match iter.next() {
                Some(b'O') => {
                    match iter.next() {
                        // F1-F4
                        Some(val @ b'P'...b'S') => {
                            InputEvent::Keyboard(KeyEvent::F(1 + val - b'P'))
                        }
                        _ => return Err(error),
                    }
                }
                Some(b'[') => {
                    // This is a CSI sequence.
                    parse_csi(iter)
                }
                Some(c) => {
                    let ch = parse_utf8_char(c, iter);
                    InputEvent::Keyboard(KeyEvent::Alt(ch?))
                }
                None => InputEvent::Keyboard(KeyEvent::Esc),
            }
        }
        b'\n' | b'\r' => InputEvent::Keyboard(KeyEvent::Char('\n')),
        b'\t' => InputEvent::Keyboard(KeyEvent::Char('\t')),
        b'\x7F' => InputEvent::Keyboard(KeyEvent::Backspace),
        c @ b'\x01'...b'\x1A' => {
            InputEvent::Keyboard(KeyEvent::Ctrl((c as u8 - 0x1 + b'a') as char))
        }
        c @ b'\x1C'...b'\x1F' => {
            InputEvent::Keyboard(KeyEvent::Ctrl((c as u8 - 0x1C + b'4') as char))
        }
        b'\0' => InputEvent::Keyboard(KeyEvent::Null),
        c => {
            let ch = parse_utf8_char(c, iter);
            InputEvent::Keyboard(KeyEvent::Char(ch?))
        }
    };

    Ok(input_event)
}

/// Parses a CSI sequence, just after reading ^[
/// Returns Event::Unknown if an unrecognized sequence is found.
fn parse_csi<I>(iter: &mut I) -> InputEvent
where
    I: Iterator<Item = u8>,
{
    match iter.next() {
        Some(b'[') => match iter.next() {
            // NOTE (@imdaveho): cannot find when this occurs;
            // having another '[' after ESC[ not a likely scenario
            Some(val @ b'A'...b'E') => InputEvent::Keyboard(KeyEvent::F(1 + val - b'A')),
            _ => InputEvent::Unknown,
        },
        Some(b'D') => InputEvent::Keyboard(KeyEvent::Left),
        Some(b'C') => InputEvent::Keyboard(KeyEvent::Right),
        Some(b'A') => InputEvent::Keyboard(KeyEvent::Up),
        Some(b'B') => InputEvent::Keyboard(KeyEvent::Down),
        Some(b'H') => InputEvent::Keyboard(KeyEvent::Home),
        Some(b'F') => InputEvent::Keyboard(KeyEvent::End),
        Some(b'M') => {
            // X10 emulation mouse encoding: ESC [ CB Cx Cy (6 characters only).
            // NOTE (@imdaveho): cannot find documentation on this

            let mut next = || iter.next().unwrap();

            let cb = next() as i8 - 32;
            // (1, 1) are the coords for upper left.
            let cx = next().saturating_sub(32) as u16;
            let cy = next().saturating_sub(32) as u16;

            match cb & 0b11 {
                0 => {
                    if cb & 0x40 != 0 {
                        InputEvent::Mouse(MouseEvent::Press(MouseButton::WheelUp, cx, cy))
                    } else {
                        InputEvent::Mouse(MouseEvent::Press(MouseButton::Left, cx, cy))
                    }
                }
                1 => {
                    if cb & 0x40 != 0 {
                        InputEvent::Mouse(MouseEvent::Press(MouseButton::WheelDown, cx, cy))
                    } else {
                        InputEvent::Mouse(MouseEvent::Press(MouseButton::Middle, cx, cy))
                    }
                }
                2 => InputEvent::Mouse(MouseEvent::Press(MouseButton::Right, cx, cy)),
                3 => InputEvent::Mouse(MouseEvent::Release(cx, cy)),
                _ => InputEvent::Unknown,
            }
        }
        Some(b'<') => {
            // xterm mouse handling:
            // ESC [ < Cb ; Cx ; Cy (;) (M or m)
            let mut buf = Vec::new();
            let mut c = iter.next().unwrap();
            while match c {
                b'm' | b'M' => false,
                _ => true,
            } {
                buf.push(c);
                c = iter.next().unwrap();
            }
            let str_buf = String::from_utf8(buf).unwrap();
            let nums = &mut str_buf.split(';');

            let cb = nums.next().unwrap().parse::<u16>().unwrap();
            let cx = nums.next().unwrap().parse::<u16>().unwrap();
            let cy = nums.next().unwrap().parse::<u16>().unwrap();

            match cb {
                0...2 | 64...65 => {
                    let button = match cb {
                        0 => MouseButton::Left,
                        1 => MouseButton::Middle,
                        2 => MouseButton::Right,
                        64 => MouseButton::WheelUp,
                        65 => MouseButton::WheelDown,
                        _ => unreachable!(),
                    };
                    match c {
                        b'M' => InputEvent::Mouse(MouseEvent::Press(button, cx, cy)),
                        b'm' => InputEvent::Mouse(MouseEvent::Release(cx, cy)),
                        _ => InputEvent::Unknown,
                    }
                }
                32 => InputEvent::Mouse(MouseEvent::Hold(cx, cy)),
                3 => InputEvent::Mouse(MouseEvent::Release(cx, cy)),
                _ => InputEvent::Unknown,
            }
        }
        Some(c @ b'0'...b'9') => {
            // Numbered escape code.
            let mut buf = Vec::new();
            buf.push(c);
            let mut c = iter.next().unwrap();
            // The final byte of a CSI sequence can be in the range 64-126, so
            // let's keep reading anything else.
            while c < 64 || c > 126 {
                buf.push(c);
                c = iter.next().unwrap();
            }
            match c {
                // rxvt mouse encoding:
                // rxvt mouse encoding:
                // ESC [ Cb ; Cx ; Cy ; M
                b'M' => {
                    let str_buf = String::from_utf8(buf).unwrap();

                    let nums: Vec<u16> = str_buf.split(';').map(|n| n.parse().unwrap()).collect();

                    let cb = nums[0];
                    let cx = nums[1];
                    let cy = nums[2];

                    match cb {
                        32 => InputEvent::Mouse(MouseEvent::Press(MouseButton::Left, cx, cy)),
                        33 => InputEvent::Mouse(MouseEvent::Press(MouseButton::Middle, cx, cy)),
                        34 => InputEvent::Mouse(MouseEvent::Press(MouseButton::Right, cx, cy)),
                        35 => InputEvent::Mouse(MouseEvent::Release(cx, cy)),
                        64 => InputEvent::Mouse(MouseEvent::Hold(cx, cy)),
                        96 | 97 => {
                            InputEvent::Mouse(MouseEvent::Press(MouseButton::WheelUp, cx, cy))
                        }
                        _ => InputEvent::Unknown,
                    }
                }
                // Special key code.
                b'~' => {
                    let str_buf = String::from_utf8(buf).unwrap();

                    // This CSI sequence can be a list of semicolon-separated
                    // numbers.
                    let nums: Vec<u8> = str_buf.split(';').map(|n| n.parse().unwrap()).collect();

                    if nums.is_empty() {
                        return InputEvent::Unknown;
                    }

                    // TODO: handle multiple values for key modififiers (ex: values
                    // [3, 2] means Shift+Delete)
                    if nums.len() > 1 {
                        return InputEvent::Unknown;
                    }

                    match nums[0] {
                        1 | 7 => InputEvent::Keyboard(KeyEvent::Home),
                        2 => InputEvent::Keyboard(KeyEvent::Insert),
                        3 => InputEvent::Keyboard(KeyEvent::Delete),
                        4 | 8 => InputEvent::Keyboard(KeyEvent::End),
                        5 => InputEvent::Keyboard(KeyEvent::PageUp),
                        6 => InputEvent::Keyboard(KeyEvent::PageDown),
                        v @ 11...15 => InputEvent::Keyboard(KeyEvent::F(v - 10)),
                        v @ 17...21 => InputEvent::Keyboard(KeyEvent::F(v - 11)),
                        v @ 23...24 => InputEvent::Keyboard(KeyEvent::F(v - 12)),
                        _ => InputEvent::Unknown,
                    }
                }
                _ => InputEvent::Unknown,
            }
        }
        _ => InputEvent::Unknown,
    }
}

/// Parse `c` as either a single byte ASCII char or a variable size UTF-8 char.
fn parse_utf8_char<I>(c: u8, iter: &mut I) -> Result<char>
where
    I: Iterator<Item = u8>,
{
    let error = Err(Error::new(
        ErrorKind::Other,
        "Input character is not valid UTF-8",
    ));

    if c.is_ascii() {
        Ok(c as char)
    } else {
        let mut bytes = Vec::new();
        bytes.push(c);

        while let Some(next) = iter.next() {
            bytes.push(next);
            if let Ok(st) = str::from_utf8(&bytes) {
                return Ok(st.chars().next().unwrap()); // todo: can this be st.chars().first()
            }
            if bytes.len() >= 4 {
                return error;
            }
        }

        return error;
    }
}
