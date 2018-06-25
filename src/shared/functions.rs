//! Some actions need to preformed platform independently since they can not be solved `ANSI escape codes`.

use Context;
use std::rc::Rc;

#[cfg(windows)]
use kernel::windows_kernel::terminal::terminal_size;

#[cfg(unix)]
use kernel::unix_kernel::terminal::terminal_size;

#[cfg(windows)]
use kernel::windows_kernel::cursor::pos;
#[cfg(unix)]
use kernel::unix_kernel::terminal::pos;

/// Get the terminal size based on the current platform.
pub fn get_terminal_size() -> (u16, u16)
{
    terminal_size()
}

/// Get the cursor position based on the current platform.
pub fn get_cursor_position(context: Rc<Context>) -> (u16, u16)
{
    #[cfg(unix)]
    return pos(context.clone());

    #[cfg(windows)]
    return pos();
}

#[cfg(windows)]
/// Get an module specific implementation based on the current platform.
pub fn get_module<T>(winapi_impl: T, unix_impl: T) -> Option<T>
{
    let mut term: Option<T> = None;
    let mut does_support = true;

    if cfg!(target_os = "windows") {
        #[cfg(windows)]
        use kernel::windows_kernel::ansi_support::try_enable_ansi_support;

        // Try to enable ansi on windows if not than use WINAPI.
        does_support = try_enable_ansi_support();

        if !does_support
        {
            term = Some(winapi_impl);
        }
    }

    if does_support
    {
        term = Some(unix_impl);
    }

    term
}