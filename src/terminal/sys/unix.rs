//! UNIX related logic for terminal manipulation.

#[cfg(feature = "events")]
use crate::event::KeyboardEnhancementFlags;
use crate::terminal::{WindowSize, sys::file_descriptor::tty_fd};
use parking_lot::Mutex;
use rustix::{
    fd::{AsFd, BorrowedFd},
    termios::{Termios, Winsize},
};

use std::{fs::File, io};

// Some(Termios) -> we're in the raw mode and this is the previous mode
// None -> we're not in the raw mode
static TERMINAL_MODE_PRIOR_RAW_MODE: Mutex<Option<Termios>> = parking_lot::const_mutex(None);

pub(crate) fn is_raw_mode_enabled() -> bool {
    TERMINAL_MODE_PRIOR_RAW_MODE.lock().is_some()
}

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

pub(crate) fn window_size() -> io::Result<WindowSize> {
    if let Ok(file) = File::open("/dev/tty") {
        if let Ok(size) = rustix::termios::tcgetwinsize(file.as_fd()) {
            return Ok(size.into());
        }
    }

    first_window_size([
        rustix::stdio::stdout(),
        rustix::stdio::stderr(),
        rustix::stdio::stdin(),
    ])
}

fn first_window_size<'fd>(
    descriptors: impl IntoIterator<Item = BorrowedFd<'fd>>,
) -> io::Result<WindowSize> {
    let mut last_error = None;
    for descriptor in descriptors {
        match rustix::termios::tcgetwinsize(descriptor) {
            Ok(size) => return Ok(size.into()),
            Err(error) => last_error = Some(io::Error::from(error)),
        }
    }

    Err(last_error.unwrap_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            "no terminal file descriptors to query",
        )
    }))
}

pub(crate) fn size() -> io::Result<(u16, u16)> {
    let window_size = window_size()?;
    Ok((window_size.columns, window_size.rows))
}

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

fn get_terminal_attr(fd: impl AsFd) -> io::Result<Termios> {
    let result = rustix::termios::tcgetattr(fd)?;
    Ok(result)
}

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

    let result = File::options()
        .write(true)
        .open("/dev/tty")
        .and_then(|mut file| {
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
                return Err(io::Error::other(
                    "The keyboard enhancement status could not be read within a normal duration",
                ));
            }
            Err(_) => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{ffi::OsStr, fs::File, os::unix::ffi::OsStrExt, path::Path};

    use rustix::{
        fd::AsFd,
        pty::{OpenptFlags, grantpt, openpt, ptsname, unlockpt},
        termios::{Winsize, tcsetwinsize},
    };

    use super::first_window_size;

    #[test]
    fn window_size_uses_later_terminal_descriptor_when_earlier_one_is_not_a_terminal() {
        let master = openpt(OpenptFlags::RDWR).unwrap();
        grantpt(&master).unwrap();
        unlockpt(&master).unwrap();
        let slave_name = ptsname(&master, Vec::new()).unwrap();
        let slave = File::open(Path::new(OsStr::from_bytes(slave_name.to_bytes()))).unwrap();
        tcsetwinsize(
            &slave,
            Winsize {
                ws_col: 81,
                ws_row: 23,
                ws_xpixel: 0,
                ws_ypixel: 0,
            },
        )
        .unwrap();
        let non_terminal = File::open("/dev/null").unwrap();

        let size = first_window_size([non_terminal.as_fd(), slave.as_fd()]).unwrap();

        assert_eq!((size.columns, size.rows), (81, 23));
    }
}
