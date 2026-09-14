//! This is a WINDOWS specific implementation for input related action.

use std::convert::TryFrom;
use std::io;
use std::sync::atomic::{AtomicU64, Ordering};

use crossterm_winapi::{ConsoleMode, Handle};

use crate::terminal::sys::windows_mode::BracketedPasteState;

pub(crate) mod parse;
pub(crate) mod poll;
#[cfg(feature = "event-stream")]
pub(crate) mod waker;

const ENABLE_MOUSE_MODE: u32 = 0x0010 | 0x0080 | 0x0008;

// See https://learn.microsoft.com/en-us/windows/console/setconsolemode
pub(crate) const ENABLE_VIRTUAL_TERMINAL_INPUT: u32 = 0x0200;

/// This is a either `u64::MAX` if it's uninitialized or a valid `u32` that stores the original
/// console mode if it's initialized.
static ORIGINAL_CONSOLE_MODE: AtomicU64 = AtomicU64::new(u64::MAX);

impl BracketedPasteState {
    fn load() -> Self {
        Self::from_raw(BRACKETED_PASTE_STATE.load(Ordering::Relaxed))
    }
}

static BRACKETED_PASTE_STATE: std::sync::atomic::AtomicU8 =
    std::sync::atomic::AtomicU8::new(BracketedPasteState::Inactive.as_raw());

/// Saves the original console mode on first call (uses compare_exchange, so only the
/// first caller wins). Callers that modify the mode (raw-mode and mouse capture)
/// must call this **before** modifying the mode to ensure the stored value is the
/// true original.
pub(crate) fn init_original_console_mode(original_mode: u32) {
    let _ = ORIGINAL_CONSOLE_MODE.compare_exchange(
        u64::MAX,
        u64::from(original_mode),
        Ordering::Relaxed,
        Ordering::Relaxed,
    );
}

/// Returns the original console mode, if it has been captured.
pub(crate) fn original_console_mode() -> std::io::Result<u32> {
    u32::try_from(ORIGINAL_CONSOLE_MODE.load(Ordering::Relaxed))
        .map_err(|_| io::Error::other("Initial console modes not set"))
}

/// Enable VT input for bracketed paste and save the VT bit observed immediately
/// before this enable in the paste-specific lifecycle state. Raw mode and
/// mouse capture use a separate first-touch snapshot and do not call this
/// function. A repeated enable keeps the first saved state.
pub(crate) fn enable_bracketed_paste() -> std::io::Result<()> {
    if !matches!(BracketedPasteState::load(), BracketedPasteState::Inactive) {
        return Ok(());
    }

    let input_handle = match Handle::current_in_handle() {
        Ok(handle) => handle,
        Err(error) if crate::terminal::sys::windows_mode::is_console_unavailable_error(&error) => {
            BRACKETED_PASTE_STATE.store(BracketedPasteState::NoConsole.as_raw(), Ordering::Relaxed);
            return Ok(());
        }
        Err(error) => return Err(error),
    };
    let mode = ConsoleMode::from(input_handle);
    let current = match mode.mode() {
        Ok(current) => current,
        Err(error) if crate::terminal::sys::windows_mode::is_console_unavailable_error(&error) => {
            BRACKETED_PASTE_STATE.store(BracketedPasteState::NoConsole.as_raw(), Ordering::Relaxed);
            return Ok(());
        }
        Err(error) => return Err(error),
    };
    let state = if current & ENABLE_VIRTUAL_TERMINAL_INPUT == 0 {
        BracketedPasteState::ConsolePreVtOff
    } else {
        BracketedPasteState::ConsolePreVtOn
    };
    if current & ENABLE_VIRTUAL_TERMINAL_INPUT == 0 {
        mode.set_mode(current | ENABLE_VIRTUAL_TERMINAL_INPUT)?;
    }
    BRACKETED_PASTE_STATE.store(state.as_raw(), Ordering::Relaxed);
    Ok(())
}

/// Restore only the VT bit saved by the matching bracketed-paste enable. The
/// lifecycle state is consumed only after restoration succeeds, so a failed
/// restore can be retried. A disable without an active paste state is safe and
/// leaves the console mode untouched.
pub(crate) fn disable_bracketed_paste() -> std::io::Result<()> {
    let state = BracketedPasteState::load();
    if matches!(
        state,
        BracketedPasteState::Inactive | BracketedPasteState::NoConsole
    ) {
        if state == BracketedPasteState::NoConsole {
            BRACKETED_PASTE_STATE.store(state.after_restore(true).as_raw(), Ordering::Relaxed);
        }
        return Ok(());
    }
    let original_vt_enabled = state == BracketedPasteState::ConsolePreVtOn;

    let mode = ConsoleMode::from(Handle::current_in_handle()?);
    let current = mode.mode()?;
    let original = if original_vt_enabled {
        ENABLE_VIRTUAL_TERMINAL_INPUT
    } else {
        0
    };
    let restored =
        crate::terminal::sys::windows_mode::compute_restore_vt_input_mode(current, original);
    if restored != current {
        mode.set_mode(restored)?;
    }
    BRACKETED_PASTE_STATE.store(state.after_restore(true).as_raw(), Ordering::Relaxed);
    Ok(())
}

pub(crate) fn enable_mouse_capture() -> std::io::Result<()> {
    let mode = ConsoleMode::from(Handle::current_in_handle()?);
    let current = mode.mode()?;
    init_original_console_mode(current);
    // OR the flags to preserve existing mode bits (e.g. VT input)
    mode.set_mode(current | ENABLE_MOUSE_MODE)?;

    Ok(())
}

pub(crate) fn disable_mouse_capture() -> std::io::Result<()> {
    let mode = ConsoleMode::from(Handle::current_in_handle()?);
    // Keep the existing error behavior when the snapshot is missing. Both
    // initializers save it before modifying the mode, so a correctly paired
    // Enable/Disable guarantees that it exists; falling back would mask an
    // invalid lifecycle.
    let original = original_console_mode()?;
    let current = mode.mode()?;
    mode.set_mode(
        crate::terminal::sys::windows_mode::compute_disable_mouse_capture_mode(current, original),
    )?;
    Ok(())
}
