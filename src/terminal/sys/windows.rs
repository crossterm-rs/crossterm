//! WinAPI related logic for terminal manipulation.

use std::fmt::{self, Write};
use std::io::{self};

use crossterm_winapi::{Console, ConsoleMode, Coord, Handle, ScreenBuffer, Size};
use winapi::{
    shared::minwindef::DWORD,
    um::wincon::{SetConsoleTitleW, ENABLE_ECHO_INPUT, ENABLE_LINE_INPUT, ENABLE_PROCESSED_INPUT},
};

use crate::{
    cursor,
    terminal::{ClearType, WindowSize},
};

/// bits which can't be set in raw mode
const NOT_RAW_MODE_MASK: DWORD = ENABLE_LINE_INPUT | ENABLE_ECHO_INPUT | ENABLE_PROCESSED_INPUT;

// https://learn.microsoft.com/en-us/windows/console/setconsolemode
const ENABLE_VIRTUAL_TERMINAL_INPUT: DWORD = 0x0200;

pub(crate) fn is_raw_mode_enabled() -> std::io::Result<bool> {
    let console_mode = ConsoleMode::from(Handle::current_in_handle()?);

    let dw_mode = console_mode.mode()?;

    Ok(
        // check none of the "not raw" bits is set
        dw_mode & NOT_RAW_MODE_MASK == 0,
    )
}

/// Compute the console mode to apply when entering raw mode.
/// Extracted for unit-testability: the Win32 call site can't be mocked, but
/// the bit-manipulation logic can be verified on any platform.
///
/// `current` — the mode read from the console handle immediately before the call.
/// `original` — the mode that was in effect before crossterm first touched it,
///              or `None` if it hasn't been saved yet (VT adjustment is skipped).
#[cfg_attr(not(feature = "events"), allow(dead_code))]
fn compute_enable_raw_mode(current: DWORD, original: Option<DWORD>) -> DWORD {
    let mut mode = current & !NOT_RAW_MODE_MASK;
    // If this process originally enabled VT input (i.e., it was absent before we
    // touched the console), re-apply it.  disable_raw_mode() clears it to restore
    // the original state; we need it back every time we re-enter raw mode.
    if let Some(orig) = original {
        if orig & ENABLE_VIRTUAL_TERMINAL_INPUT == 0 {
            mode |= ENABLE_VIRTUAL_TERMINAL_INPUT;
        }
    }
    mode
}

/// Compute the console mode to apply when leaving raw mode.
/// See `compute_enable_raw_mode` for parameter semantics.
fn compute_disable_raw_mode(current: DWORD, original: Option<DWORD>) -> DWORD {
    let mut mode = current | NOT_RAW_MODE_MASK;
    // Clear VT input only if this process was the one that enabled it.
    if let Some(orig) = original {
        if orig & ENABLE_VIRTUAL_TERMINAL_INPUT == 0 {
            mode &= !ENABLE_VIRTUAL_TERMINAL_INPUT;
        }
    }
    mode
}

pub(crate) fn enable_raw_mode() -> std::io::Result<()> {
    let console_mode = ConsoleMode::from(Handle::current_in_handle()?);
    let dw_mode = console_mode.mode()?;

    #[cfg(feature = "events")]
    let original = crate::event::sys::windows::original_console_mode().ok();
    #[cfg(not(feature = "events"))]
    let original: Option<DWORD> = None;

    let new_mode = compute_enable_raw_mode(dw_mode, original);
    // ENABLE_VIRTUAL_TERMINAL_INPUT may be unsupported on legacy consoles.
    // If the combined set_mode call fails, retry without the VT bit so raw
    // mode is always entered; VT input is treated as best-effort.
    if console_mode.set_mode(new_mode).is_err() {
        console_mode.set_mode(new_mode & !ENABLE_VIRTUAL_TERMINAL_INPUT)?;
    }
    Ok(())
}

pub(crate) fn disable_raw_mode() -> std::io::Result<()> {
    let console_mode = ConsoleMode::from(Handle::current_in_handle()?);
    let dw_mode = console_mode.mode()?;

    #[cfg(feature = "events")]
    let original = crate::event::sys::windows::original_console_mode().ok();
    #[cfg(not(feature = "events"))]
    let original: Option<DWORD> = None;

    let new_mode = compute_disable_raw_mode(dw_mode, original);
    console_mode.set_mode(new_mode)?;

    Ok(())
}

pub(crate) fn size() -> io::Result<(u16, u16)> {
    let terminal_size = ScreenBuffer::current()?.info()?.terminal_size();
    // windows starts counting at 0, unix at 1, add one to replicated unix behaviour.
    Ok((
        (terminal_size.width + 1) as u16,
        (terminal_size.height + 1) as u16,
    ))
}

pub(crate) fn window_size() -> io::Result<WindowSize> {
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "Window pixel size not implemented for the Windows API.",
    ))
}

/// Queries the terminal's support for progressive keyboard enhancement.
///
/// This always returns `Ok(false)` on Windows.
#[cfg(feature = "events")]
pub fn supports_keyboard_enhancement() -> std::io::Result<bool> {
    Ok(false)
}

pub(crate) fn clear(clear_type: ClearType) -> std::io::Result<()> {
    let screen_buffer = ScreenBuffer::current()?;
    let csbi = screen_buffer.info()?;

    let pos = csbi.cursor_pos();
    let buffer_size = csbi.buffer_size();
    let current_attribute = csbi.attributes();

    match clear_type {
        ClearType::All => {
            clear_entire_screen(buffer_size, current_attribute)?;
        }
        ClearType::FromCursorDown => clear_after_cursor(pos, buffer_size, current_attribute)?,
        ClearType::FromCursorUp => clear_before_cursor(pos, buffer_size, current_attribute)?,
        ClearType::CurrentLine => clear_current_line(pos, buffer_size, current_attribute)?,
        ClearType::UntilNewLine => clear_until_line(pos, buffer_size, current_attribute)?,
        _ => {
            clear_entire_screen(buffer_size, current_attribute)?;
        } //TODO: make purge flush the entire screen buffer not just the visible window.
    };
    Ok(())
}

pub(crate) fn scroll_up(row_count: u16) -> std::io::Result<()> {
    let csbi = ScreenBuffer::current()?;
    let mut window = csbi.info()?.terminal_window();

    // check whether the window is too close to the screen buffer top
    let count = row_count as i16;
    if window.top >= count {
        window.top -= count; // move top down
        window.bottom -= count; // move bottom down

        Console::output()?.set_console_info(true, window)?;
    }
    Ok(())
}

pub(crate) fn scroll_down(row_count: u16) -> std::io::Result<()> {
    let screen_buffer = ScreenBuffer::current()?;
    let csbi = screen_buffer.info()?;
    let mut window = csbi.terminal_window();
    let buffer_size = csbi.buffer_size();

    // check whether the window is too close to the screen buffer top
    let count = row_count as i16;
    if window.bottom < buffer_size.height - count {
        window.top += count; // move top down
        window.bottom += count; // move bottom down

        Console::output()?.set_console_info(true, window)?;
    }
    Ok(())
}

pub(crate) fn set_size(width: u16, height: u16) -> std::io::Result<()> {
    if width <= 1 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "terminal width must be at least 1",
        ));
    }

    if height <= 1 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "terminal height must be at least 1",
        ));
    }

    // get the position of the current console window
    let screen_buffer = ScreenBuffer::current()?;
    let console = Console::from(screen_buffer.handle().clone());
    let csbi = screen_buffer.info()?;

    let current_size = csbi.buffer_size();
    let window = csbi.terminal_window();

    let mut new_size = Size::new(current_size.width, current_size.height);

    // If the buffer is smaller than this new window size, resize the
    // buffer to be large enough.  Include window position.
    let mut resize_buffer = false;

    let width = width as i16;
    if current_size.width < window.left + width {
        if window.left >= i16::MAX - width {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "terminal width too large",
            ));
        }

        new_size.width = window.left + width;
        resize_buffer = true;
    }
    let height = height as i16;
    if current_size.height < window.top + height {
        if window.top >= i16::MAX - height {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "terminal height too large",
            ));
        }

        new_size.height = window.top + height;
        resize_buffer = true;
    }

    if resize_buffer {
        screen_buffer.set_size(new_size.width - 1, new_size.height - 1)?;
    }

    let mut window = window;

    // preserve the position, but change the size.
    window.bottom = window.top + height - 1;
    window.right = window.left + width - 1;
    console.set_console_info(true, window)?;

    // if we resized the buffer, un-resize it.
    if resize_buffer {
        screen_buffer.set_size(current_size.width - 1, current_size.height - 1)?;
    }

    let bounds = console.largest_window_size()?;

    if width > bounds.x {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("terminal width {width} too large"),
        ));
    }
    if height > bounds.y {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("terminal height {height} too large"),
        ));
    }

    Ok(())
}

pub(crate) fn set_window_title(title: impl fmt::Display) -> std::io::Result<()> {
    struct Utf16Encoder(Vec<u16>);
    impl Write for Utf16Encoder {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.0.extend(s.encode_utf16());
            Ok(())
        }
    }

    let mut title_utf16 = Utf16Encoder(Vec::new());
    write!(title_utf16, "{title}").expect("formatting failed");
    title_utf16.0.push(0);
    let title = title_utf16.0;

    let result = unsafe { SetConsoleTitleW(title.as_ptr()) };
    if result != 0 {
        Ok(())
    } else {
        Err(io::Error::last_os_error())
    }
}

fn clear_after_cursor(
    location: Coord,
    buffer_size: Size,
    current_attribute: u16,
) -> std::io::Result<()> {
    let (mut x, mut y) = (location.x, location.y);

    // if cursor position is at the outer right position
    if x > buffer_size.width {
        y += 1;
        x = 0;
    }

    // location where to start clearing
    let start_location = Coord::new(x, y);

    // get sum cells before cursor
    let cells_to_write = buffer_size.width as u32 * buffer_size.height as u32;

    clear_winapi(start_location, cells_to_write, current_attribute)
}

fn clear_before_cursor(
    location: Coord,
    buffer_size: Size,
    current_attribute: u16,
) -> std::io::Result<()> {
    let (xpos, ypos) = (location.x, location.y);

    // one cell after cursor position
    let x = 0;
    // one at row of cursor position
    let y = 0;

    // location where to start clearing
    let start_location = Coord::new(x, y);

    // get sum cells before cursor
    let cells_to_write = (buffer_size.width as u32 * ypos as u32) + (xpos as u32 + 1);

    // clear everything before cursor position
    clear_winapi(start_location, cells_to_write, current_attribute)
}

fn clear_entire_screen(buffer_size: Size, current_attribute: u16) -> std::io::Result<()> {
    // get sum cells before cursor
    let cells_to_write = buffer_size.width as u32 * buffer_size.height as u32;

    // location where to start clearing
    let start_location = Coord::new(0, 0);

    // clear the entire screen
    clear_winapi(start_location, cells_to_write, current_attribute)?;

    // put the cursor back at cell 0,0
    cursor::sys::move_to(0, 0)?;
    Ok(())
}

fn clear_current_line(
    location: Coord,
    buffer_size: Size,
    current_attribute: u16,
) -> std::io::Result<()> {
    // location where to start clearing
    let start_location = Coord::new(0, location.y);

    // get sum cells before cursor
    let cells_to_write = buffer_size.width as u32;

    // clear the whole current line
    clear_winapi(start_location, cells_to_write, current_attribute)?;

    // put the cursor back at cell 1 on current row
    cursor::sys::move_to(0, location.y as u16)?;
    Ok(())
}

fn clear_until_line(
    location: Coord,
    buffer_size: Size,
    current_attribute: u16,
) -> std::io::Result<()> {
    let (x, y) = (location.x, location.y);

    // location where to start clearing
    let start_location = Coord::new(x, y);

    // get sum cells before cursor
    let cells_to_write = (buffer_size.width - x) as u32;

    // clear until the current line
    clear_winapi(start_location, cells_to_write, current_attribute)?;

    // put the cursor back at original cursor position before we did the clearing
    cursor::sys::move_to(x as u16, y as u16)?;
    Ok(())
}

fn clear_winapi(
    start_location: Coord,
    cells_to_write: u32,
    current_attribute: u16,
) -> std::io::Result<()> {
    let console = Console::from(Handle::current_out_handle()?);
    console.fill_whit_character(start_location, cells_to_write, ' ')?;
    console.fill_whit_attribute(start_location, cells_to_write, current_attribute)?;
    Ok(())
}

#[cfg(test)]
// Create a new screen buffer to avoid changing the terminal the test
// is running within.
pub fn temp_screen_buffer() -> std::io::Result<ScreenBuffer> {
    let alternate_screen = ScreenBuffer::create()?;
    alternate_screen.show().unwrap();
    Ok(alternate_screen)
}

#[cfg(test)]
mod tests {
    use std::{ffi::OsString, os::windows::ffi::OsStringExt};

    use crossterm_winapi::ScreenBuffer;
    use serial_test::serial;
    use winapi::um::wincon::GetConsoleTitleW;

    use super::{
        compute_disable_raw_mode, compute_enable_raw_mode, scroll_down, scroll_up, set_size,
        set_window_title, size, temp_screen_buffer, ENABLE_VIRTUAL_TERMINAL_INPUT,
        NOT_RAW_MODE_MASK,
    };

    #[test]
    #[serial]
    fn test_resize_winapi_20_21() {
        let _test_screen = temp_screen_buffer().unwrap();

        let (width, height) = size().unwrap();

        // The values 20 and 21 are arbitrary and different from each other
        // just to see they're not crossed over.
        set_size(20, 21).unwrap();
        assert_eq!((20, 21), size().unwrap());

        // reset to previous size
        set_size(width, height).unwrap();
        assert_eq!((width, height), size().unwrap());
    }

    // This is similar to test_resize_winapi_20_21() above. This verifies that
    // another test of similar functionality runs independently (that a testing
    // race condition has been addressed).
    #[test]
    #[serial]
    #[ignore]
    fn test_resize_winapi_30_31() {
        let _test_screen = temp_screen_buffer().unwrap();

        let (width, height) = size().unwrap();

        set_size(30, 31).unwrap();
        assert_eq!((30, 31), size().unwrap());

        // reset to previous size
        set_size(width, height).unwrap();
        assert_eq!((width, height), size().unwrap());
    }

    // Test is disabled, because it's failing on Travis CI
    #[test]
    #[ignore]
    fn test_scroll_down_winapi() {
        let current_window = ScreenBuffer::current()
            .unwrap()
            .info()
            .unwrap()
            .terminal_window();

        scroll_down(2).unwrap();

        let new_window = ScreenBuffer::current()
            .unwrap()
            .info()
            .unwrap()
            .terminal_window();

        assert_eq!(new_window.top, current_window.top + 2);
        assert_eq!(new_window.bottom, current_window.bottom + 2);
    }

    // Test is disabled, because it's failing on Travis CI
    #[test]
    #[ignore]
    fn test_scroll_up_winapi() {
        // move the terminal buffer down before moving it up
        test_scroll_down_winapi();

        let current_window = ScreenBuffer::current()
            .unwrap()
            .info()
            .unwrap()
            .terminal_window();

        scroll_up(2).unwrap();

        let new_window = ScreenBuffer::current()
            .unwrap()
            .info()
            .unwrap()
            .terminal_window();

        assert_eq!(new_window.top, current_window.top - 2);
        assert_eq!(new_window.bottom, current_window.bottom - 2);
    }

    #[test]
    #[serial]
    fn test_set_title_winapi() {
        let _test_screen = temp_screen_buffer().unwrap();

        let test_title = "this is a crossterm test title";
        set_window_title(test_title).unwrap();

        let mut raw = [0_u16; 128];
        let length = unsafe { GetConsoleTitleW(raw.as_mut_ptr(), raw.len() as u32) } as usize;
        assert_ne!(0, length);

        let console_title = OsString::from_wide(&raw[..length]).into_string().unwrap();
        assert_eq!(test_title, &console_title[..]);
    }

    // --- Pure bit-logic tests (platform-independent) ---

    const BASE_MODE: u32 = 0x0007; // some arbitrary non-VT bits

    #[test]
    fn test_compute_enable_raw_mode_clears_not_raw_bits() {
        let current = BASE_MODE | NOT_RAW_MODE_MASK;
        let result = compute_enable_raw_mode(current, None);
        assert_eq!(
            result & NOT_RAW_MODE_MASK,
            0,
            "NOT_RAW_MODE bits must be cleared"
        );
    }

    #[test]
    fn test_compute_enable_raw_mode_sets_vt_when_original_lacked_it() {
        // Original mode had no VT input — crossterm added it, so re-enable on entry.
        let current = BASE_MODE;
        let original = Some(BASE_MODE); // no ENABLE_VIRTUAL_TERMINAL_INPUT bit
        let result = compute_enable_raw_mode(current, original);
        assert_ne!(
            result & ENABLE_VIRTUAL_TERMINAL_INPUT,
            0,
            "VT input must be re-enabled"
        );
    }

    #[test]
    fn test_compute_enable_raw_mode_preserves_vt_when_original_had_it() {
        // VT input was already present before crossterm — don't touch it.
        let current = BASE_MODE | ENABLE_VIRTUAL_TERMINAL_INPUT;
        let original = Some(BASE_MODE | ENABLE_VIRTUAL_TERMINAL_INPUT);
        let result = compute_enable_raw_mode(current, original);
        // The function doesn't strip existing VT bits (it may or may not set it again, doesn't matter).
        assert_ne!(
            result & ENABLE_VIRTUAL_TERMINAL_INPUT,
            0,
            "pre-existing VT input must be preserved"
        );
    }

    #[test]
    fn test_compute_enable_raw_mode_no_panic_when_original_unknown() {
        // original_console_mode() can fail if called before any event reader init.
        let current = BASE_MODE | NOT_RAW_MODE_MASK;
        let result = compute_enable_raw_mode(current, None);
        assert_eq!(result & NOT_RAW_MODE_MASK, 0);
    }

    #[test]
    fn test_compute_disable_raw_mode_restores_not_raw_bits() {
        let current = BASE_MODE;
        let result = compute_disable_raw_mode(current, None);
        assert_eq!(
            result & NOT_RAW_MODE_MASK,
            NOT_RAW_MODE_MASK,
            "NOT_RAW_MODE bits must be restored"
        );
    }

    #[test]
    fn test_compute_disable_raw_mode_clears_vt_when_original_lacked_it() {
        let current = BASE_MODE | ENABLE_VIRTUAL_TERMINAL_INPUT;
        let original = Some(BASE_MODE); // crossterm enabled VT, so clear on exit
        let result = compute_disable_raw_mode(current, original);
        assert_eq!(
            result & ENABLE_VIRTUAL_TERMINAL_INPUT,
            0,
            "VT input must be cleared on exit"
        );
    }

    #[test]
    fn test_compute_disable_raw_mode_preserves_vt_when_original_had_it() {
        let current = BASE_MODE | ENABLE_VIRTUAL_TERMINAL_INPUT;
        let original = Some(BASE_MODE | ENABLE_VIRTUAL_TERMINAL_INPUT);
        let result = compute_disable_raw_mode(current, original);
        assert_ne!(
            result & ENABLE_VIRTUAL_TERMINAL_INPUT,
            0,
            "pre-existing VT input must not be cleared"
        );
    }

    #[test]
    fn test_compute_disable_raw_mode_no_panic_when_original_unknown() {
        let current = BASE_MODE | ENABLE_VIRTUAL_TERMINAL_INPUT;
        let result = compute_disable_raw_mode(current, None);
        // When original is unknown we don't touch VT (safe default: leave it as-is)
        assert_ne!(
            result & ENABLE_VIRTUAL_TERMINAL_INPUT,
            0,
            "VT input left intact when original unknown"
        );
    }

    #[test]
    fn test_repeated_enable_disable_cycle_preserves_vt() {
        // Simulate two raw mode cycles when original had no VT input.
        let original = Some(BASE_MODE);
        let starting = BASE_MODE;

        // First enable_raw_mode
        let after_enable1 = compute_enable_raw_mode(starting, original);
        assert_ne!(after_enable1 & ENABLE_VIRTUAL_TERMINAL_INPUT, 0);
        // First disable_raw_mode
        let after_disable1 = compute_disable_raw_mode(after_enable1, original);
        assert_eq!(after_disable1 & ENABLE_VIRTUAL_TERMINAL_INPUT, 0);
        // Second enable_raw_mode (the regression this patch fixes)
        let after_enable2 = compute_enable_raw_mode(after_disable1, original);
        assert_ne!(
            after_enable2 & ENABLE_VIRTUAL_TERMINAL_INPUT,
            0,
            "VT input must be re-enabled on second cycle"
        );
        // Second disable_raw_mode
        let after_disable2 = compute_disable_raw_mode(after_enable2, original);
        assert_eq!(after_disable2 & ENABLE_VIRTUAL_TERMINAL_INPUT, 0);
    }
}
