//! All platform specific (unsafe) code will be handled in this module.

#[cfg(unix)]
pub mod unix_kernel;
#[cfg(windows)]
pub mod windows_kernel;
