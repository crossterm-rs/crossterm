//! UNIX related logic for terminal manipulation.

#[cfg(feature = "events")]
use crate::event::KeyboardEnhancementFlags;
use crate::terminal::{
    sys::file_descriptor::{tty_fd, FileDesc},
    WindowSize,
};
#[cfg(feature = "libc")]
use libc::{
    cfmakeraw, dup, ioctl, tcgetattr, tcsetattr, termios as Termios, winsize, STDOUT_FILENO,
    TCSANOW, TIOCGWINSZ,
};
use parking_lot::Mutex;
#[cfg(not(feature = "libc"))]
use {
    rustix::{
        fd::{AsFd, FromRawFd},
        termios::{Termios, Winsize},
    },
    std::{
        io::Error,
        os::fd::{BorrowedFd, IntoRawFd},
    },
};

use std::{
    fs::File,
    io::{self, Read, Write},
    process,
};
#[cfg(feature = "libc")]
use std::{
    io::Error,
    mem,
    os::fd::{AsRawFd, FromRawFd},
    os::unix::io::{IntoRawFd, RawFd},
};

// Some(Termios) -> we're in the raw mode and this is the previous mode
// None -> we're not in the raw mode
static TERMINAL_MODE_PRIOR_RAW_MODE: Mutex<Option<Termios>> = parking_lot::const_mutex(None);

pub(crate) fn is_raw_mode_enabled() -> bool {
    TERMINAL_MODE_PRIOR_RAW_MODE.lock().is_some()
}

#[cfg(feature = "libc")]
impl From<winsize> for WindowSize {
    fn from(size: winsize) -> WindowSize {
        WindowSize {
            columns: size.ws_col,
            rows: size.ws_row,
            width: size.ws_xpixel,
            height: size.ws_ypixel,
        }
    }
}
#[cfg(not(feature = "libc"))]
impl From<Winsize> for WindowSize {
    fn from(size: Winsize) -> WindowSize {
        WindowSize {
            columns: size.ws_col,
            rows: size.ws_row,
            width: size.ws_xpixel,
            height: size.ws_ypixel,
        }
    }
}

/// Parse the csi escape sequence response
///
/// It must follow this format : "\x1b[4;height;width" + "t"
fn parse_csi_response(buf: &[u8]) -> Option<(u16, u16)> {
    let nul_terminator_pos = buf.iter().position(|c| *c == 0).unwrap_or(buf.len());

    let response = std::str::from_utf8(&buf[..nul_terminator_pos]).ok()?;

    if let Some(response) = response
        .strip_prefix("\x1b[4;")
        .and_then(|response| response.strip_suffix("t"))
    {
        let mut iter = response.split(";");

        // TODO: rewrite using a let chain when using Rust edition 2024 or later
        let height = iter.next().and_then(|height| height.parse().ok());
        let width = iter.next().and_then(|height| height.parse().ok());
        match (height, width) {
            (Some(height), Some(width)) => Some((height, width)),
            _ => None,
        }
    } else {
        None
    }
}

#[cfg(feature = "libc")]
fn csi_window_size(fd: i32) -> io::Result<(u16, u16)> {
    let original_term_attr = if is_raw_mode_enabled() {
        None
    } else {
        Some(get_terminal_attr(fd)?)
    };

    // We need to duplicate the file descriptor so that the original fd is not closed
    // twice when the new `File` we create is dropped
    let fd = match unsafe { dup(fd) } {
        i32::MIN..0 => return Err(Error::new(io::ErrorKind::Other, "dup() failed")),
        fd => fd,
    };
    let mut file = unsafe { std::fs::File::from_raw_fd(fd) };

    // Enabling raw mode if not enabled already
    if let Some(original_term_attr) = &original_term_attr {
        let mut ios = *original_term_attr;
        raw_terminal_attr(&mut ios);
        set_terminal_attr(file.as_raw_fd(), &ios)?;
    }

    // After this point, we have to check if we need to disable raw mode
    // before exiting this function

    file.write("\x1b[14t".as_bytes()).map_err(|err| {
        if let Some(original_term_attr) = &original_term_attr {
            let _ = set_terminal_attr(file.as_raw_fd(), original_term_attr);
        };
        err
    })?;

    let mut response = [0u8; 32];
    file.read(&mut response).map_err(|err| {
        if let Some(original_term_attr) = &original_term_attr {
            let _ = set_terminal_attr(file.as_raw_fd(), original_term_attr);
        };
        err
    })?;

    let ws_pixel = parse_csi_response(&response).ok_or_else(|| {
        if let Some(original_term_attr) = &original_term_attr {
            let _ = set_terminal_attr(file.as_raw_fd(), original_term_attr);
        };
        Error::new(io::ErrorKind::Other, "parse_csi_response() failed")
    })?;

    // Disabling raw mode if *we* enabled it
    if let Some(original_term_attr) = original_term_attr {
        set_terminal_attr(file.as_raw_fd(), &original_term_attr)?;
    }

    Ok(ws_pixel)
}

/// Get the window size in pixels
///
/// This function is called when window_size() sets size.width or size.height to 0
/// It determines the window size using a CSI control sequence (https://www.xfree86.org/current/ctlseqs.html)
/// It returns a tuple (ws_xpixel, ws_ypixel)
#[cfg(not(feature = "libc"))]
fn csi_window_size(fd: BorrowedFd<'_>) -> io::Result<(u16, u16)> {
    let original_term_attr = if is_raw_mode_enabled() {
        None
    } else {
        Some(get_terminal_attr(fd)?)
    };

    // We need to duplicate the file descriptor so that the original fd is not closed
    // twice when the new `File` we create is dropped
    let fd = rustix::io::dup(fd)?;
    let mut file = unsafe { std::fs::File::from_raw_fd(fd.into_raw_fd()) };

    // Enabling raw mode if not enabled already
    if let Some(original_term_attr) = &original_term_attr {
        let mut ios = original_term_attr.clone();
        ios.make_raw();
        set_terminal_attr(file.as_fd(), &ios)?;
    }

    // After this point, we have to check if we need to disable raw mode
    // before exiting this function

    file.write("\x1b[14t".as_bytes()).map_err(|err| {
        if let Some(original_term_attr) = &original_term_attr {
            let _ = set_terminal_attr(file.as_fd(), original_term_attr);
        };
        err
    })?;

    let mut response = [0u8; 32];
    file.read(&mut response).map_err(|err| {
        if let Some(original_term_attr) = &original_term_attr {
            let _ = set_terminal_attr(file.as_fd(), original_term_attr);
        };
        err
    })?;

    let ws_pixel = parse_csi_response(&response).ok_or_else(|| {
        if let Some(original_term_attr) = &original_term_attr {
            let _ = set_terminal_attr(file.as_fd(), original_term_attr);
        };
        Error::new(io::ErrorKind::Other, "parse_csi_response() failed")
    })?;

    // Disabling raw mode if *we* enabled it
    if let Some(original_term_attr) = original_term_attr {
        set_terminal_attr(file.as_fd(), &original_term_attr)?;
    }

    Ok(ws_pixel)
}

#[allow(clippy::useless_conversion)]
#[cfg(feature = "libc")]
pub(crate) fn window_size() -> io::Result<WindowSize> {
    // http://rosettacode.org/wiki/Terminal_control/Dimensions#Library:_BSD_libc
    let mut size = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };

    let file = File::options()
        .read(true)
        .write(true)
        .open("/dev/tty")
        .map(|file| FileDesc::new(file.into_raw_fd(), true));

    let fd = if let Ok(file) = &file {
        file.raw_fd()
    } else {
        // Fallback to libc::STDOUT_FILENO if /dev/tty is missing
        STDOUT_FILENO
    };

    if wrap_with_result(unsafe { ioctl(fd, TIOCGWINSZ.into(), &mut size) }).is_ok() {
        if size.ws_xpixel == 0 || size.ws_ypixel == 0 {
            if let Ok((ws_xpixel, ws_ypixel)) = csi_window_size(fd) {
                size.ws_xpixel = ws_xpixel;
                size.ws_ypixel = ws_ypixel;
            }
        }
        return Ok(size.into());
    }

    Err(std::io::Error::last_os_error().into())
}

#[cfg(not(feature = "libc"))]
pub(crate) fn window_size() -> io::Result<WindowSize> {
    let file = File::options()
        .read(true)
        .write(true)
        .open("/dev/tty")
        .map(|file| FileDesc::Owned(file.into()));

    let fd = if let Ok(file) = &file {
        file.as_fd()
    } else {
        // Fallback to libc::STDOUT_FILENO if /dev/tty is missing
        rustix::stdio::stdout()
    };
    let mut size = rustix::termios::tcgetwinsize(fd)?;
    if size.ws_xpixel == 0 || size.ws_ypixel == 0 {
        if let Ok((ws_xpixel, ws_ypixel)) = csi_window_size(fd) {
            size.ws_xpixel = ws_xpixel;
            size.ws_ypixel = ws_ypixel;
        }
    }
    Ok(size.into())
}

#[allow(clippy::useless_conversion)]
pub(crate) fn size() -> io::Result<(u16, u16)> {
    if let Ok(window_size) = window_size() {
        return Ok((window_size.columns, window_size.rows));
    }

    tput_size().ok_or_else(|| std::io::Error::last_os_error().into())
}

#[cfg(feature = "libc")]
pub(crate) fn enable_raw_mode() -> io::Result<()> {
    let mut original_mode = TERMINAL_MODE_PRIOR_RAW_MODE.lock();
    if original_mode.is_some() {
        return Ok(());
    }

    let tty = tty_fd()?;
    let fd = tty.raw_fd();
    let mut ios = get_terminal_attr(fd)?;
    let original_mode_ios = ios;
    raw_terminal_attr(&mut ios);
    set_terminal_attr(fd, &ios)?;
    // Keep it last - set the original mode only if we were able to switch to the raw mode
    *original_mode = Some(original_mode_ios);
    Ok(())
}

#[cfg(not(feature = "libc"))]
pub(crate) fn enable_raw_mode() -> io::Result<()> {
    let mut original_mode = TERMINAL_MODE_PRIOR_RAW_MODE.lock();
    if original_mode.is_some() {
        return Ok(());
    }

    let tty = tty_fd()?;
    let mut ios = get_terminal_attr(&tty)?;
    let original_mode_ios = ios.clone();
    ios.make_raw();
    set_terminal_attr(&tty, &ios)?;
    // Keep it last - set the original mode only if we were able to switch to the raw mode
    *original_mode = Some(original_mode_ios);
    Ok(())
}

/// Reset the raw mode.
///
/// More precisely, reset the whole termios mode to what it was before the first call
/// to [enable_raw_mode]. If you don't mess with termios outside of crossterm, it's
/// effectively disabling the raw mode and doing nothing else.
#[cfg(feature = "libc")]
pub(crate) fn disable_raw_mode() -> io::Result<()> {
    let mut original_mode = TERMINAL_MODE_PRIOR_RAW_MODE.lock();
    if let Some(original_mode_ios) = original_mode.as_ref() {
        let tty = tty_fd()?;
        set_terminal_attr(tty.raw_fd(), original_mode_ios)?;
        // Keep it last - remove the original mode only if we were able to switch back
        *original_mode = None;
    }
    Ok(())
}

#[cfg(not(feature = "libc"))]
pub(crate) fn disable_raw_mode() -> io::Result<()> {
    let mut original_mode = TERMINAL_MODE_PRIOR_RAW_MODE.lock();
    if let Some(original_mode_ios) = original_mode.as_ref() {
        let tty = tty_fd()?;
        set_terminal_attr(&tty, original_mode_ios)?;
        // Keep it last - remove the original mode only if we were able to switch back
        *original_mode = None;
    }
    Ok(())
}

#[cfg(not(feature = "libc"))]
fn get_terminal_attr(fd: impl AsFd) -> io::Result<Termios> {
    let result = rustix::termios::tcgetattr(fd)?;
    Ok(result)
}

#[cfg(not(feature = "libc"))]
fn set_terminal_attr(fd: impl AsFd, termios: &Termios) -> io::Result<()> {
    rustix::termios::tcsetattr(fd, rustix::termios::OptionalActions::Now, termios)?;
    Ok(())
}

/// Queries the terminal's support for progressive keyboard enhancement.
///
/// On unix systems, this function will block and possibly time out while
/// [`crossterm::event::read`](crate::event::read) or [`crossterm::event::poll`](crate::event::poll) are being called.
#[cfg(feature = "events")]
pub fn supports_keyboard_enhancement() -> io::Result<bool> {
    query_keyboard_enhancement_flags().map(|flags| flags.is_some())
}

/// Queries the terminal's currently active keyboard enhancement flags.
///
/// On unix systems, this function will block and possibly time out while
/// [`crossterm::event::read`](crate::event::read) or [`crossterm::event::poll`](crate::event::poll) are being called.
#[cfg(feature = "events")]
pub fn query_keyboard_enhancement_flags() -> io::Result<Option<KeyboardEnhancementFlags>> {
    if is_raw_mode_enabled() {
        query_keyboard_enhancement_flags_raw()
    } else {
        query_keyboard_enhancement_flags_nonraw()
    }
}

#[cfg(feature = "events")]
fn query_keyboard_enhancement_flags_nonraw() -> io::Result<Option<KeyboardEnhancementFlags>> {
    enable_raw_mode()?;
    let flags = query_keyboard_enhancement_flags_raw();
    disable_raw_mode()?;
    flags
}

#[cfg(feature = "events")]
fn query_keyboard_enhancement_flags_raw() -> io::Result<Option<KeyboardEnhancementFlags>> {
    use crate::event::{
        filter::{KeyboardEnhancementFlagsFilter, PrimaryDeviceAttributesFilter},
        internal::{self, InternalEvent},
    };
    use std::io::Write;
    use std::time::Duration;

    // This is the recommended method for testing support for the keyboard enhancement protocol.
    // We send a query for the flags supported by the terminal and then the primary device attributes
    // query. If we receive the primary device attributes response but not the keyboard enhancement
    // flags, none of the flags are supported.
    //
    // See <https://sw.kovidgoyal.net/kitty/keyboard-protocol/#detection-of-support-for-this-protocol>

    // ESC [ ? u        Query progressive keyboard enhancement flags (kitty protocol).
    // ESC [ c          Query primary device attributes.
    const QUERY: &[u8] = b"\x1B[?u\x1B[c";

    let result = File::open("/dev/tty").and_then(|mut file| {
        file.write_all(QUERY)?;
        file.flush()
    });
    if result.is_err() {
        let mut stdout = io::stdout();
        stdout.write_all(QUERY)?;
        stdout.flush()?;
    }

    loop {
        match internal::poll(
            Some(Duration::from_millis(2000)),
            &KeyboardEnhancementFlagsFilter,
        ) {
            Ok(true) => {
                match internal::read(&KeyboardEnhancementFlagsFilter) {
                    Ok(InternalEvent::KeyboardEnhancementFlags(current_flags)) => {
                        // Flush the PrimaryDeviceAttributes out of the event queue.
                        internal::read(&PrimaryDeviceAttributesFilter).ok();
                        return Ok(Some(current_flags));
                    }
                    _ => return Ok(None),
                }
            }
            Ok(false) => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "The keyboard enhancement status could not be read within a normal duration",
                ));
            }
            Err(_) => {}
        }
    }
}

/// execute tput with the given argument and parse
/// the output as a u16.
///
/// The arg should be "cols" or "lines"
fn tput_value(arg: &str) -> Option<u16> {
    let output = process::Command::new("tput").arg(arg).output().ok()?;
    let value = output
        .stdout
        .into_iter()
        .filter_map(|b| char::from(b).to_digit(10))
        .fold(0, |v, n| v * 10 + n as u16);

    if value > 0 {
        Some(value)
    } else {
        None
    }
}

/// Returns the size of the screen as determined by tput.
///
/// This alternate way of computing the size is useful
/// when in a subshell.
fn tput_size() -> Option<(u16, u16)> {
    match (tput_value("cols"), tput_value("lines")) {
        (Some(w), Some(h)) => Some((w, h)),
        _ => None,
    }
}

#[cfg(feature = "libc")]
// Transform the given mode into an raw mode (non-canonical) mode.
fn raw_terminal_attr(termios: &mut Termios) {
    unsafe { cfmakeraw(termios) }
}

#[cfg(feature = "libc")]
fn get_terminal_attr(fd: RawFd) -> io::Result<Termios> {
    unsafe {
        let mut termios = mem::zeroed();
        wrap_with_result(tcgetattr(fd, &mut termios))?;
        Ok(termios)
    }
}

#[cfg(feature = "libc")]
fn set_terminal_attr(fd: RawFd, termios: &Termios) -> io::Result<()> {
    wrap_with_result(unsafe { tcsetattr(fd, TCSANOW, termios) })
}

#[cfg(feature = "libc")]
fn wrap_with_result(result: i32) -> io::Result<()> {
    if result == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}
