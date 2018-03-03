//! Some actions need to preformed platform independently.
//!
use Context;
use shared::traits::Construct;
#[cfg(windows)]
use kernel::windows_kernel::terminal::terminal_size;
#[cfg(unix)]
use kernel::unix_kernel::terminal::terminal_size;

#[cfg(windows)]
use kernel::windows_kernel::cursor::pos;
#[cfg(unix)]
use kernel::unix_kernel::terminal::pos;


pub fn get_terminal_size() -> (u16, u16)
{
    terminal_size()
}

pub fn get_cursor_position() -> (u16,u16)
{
   pos()
}

#[cfg(windows)]
/// Get the module specific implementation based on the current platform
pub fn get_module<T>(winapi_impl: T, unix_impl: T, context: &mut Context) -> Option<T>
{
    let mut term: Option<T> = None;
    let mut does_support = true;

    if cfg!(target_os = "windows") {
        #[cfg(windows)]
        use kernel::windows_kernel::ansi_support::try_enable_ansi_support;

        // Try to enable ansi on windows if not than use WINAPI.
        #[cfg(windows)]
        does_support = try_enable_ansi_support(context);

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