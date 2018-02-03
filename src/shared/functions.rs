#[cfg(unix)]
use kernel::linux_kernel::terminal::terminal_size;
#[cfg(windows)]
use kernel::windows_kernel::terminal::terminal_size;

pub fn resize_terminal() -> (u16,u16)
{
    terminal_size()
}