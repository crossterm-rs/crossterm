use std::io;

#[cfg(feature = "libc")]
use libc::size_t;
#[cfg(feature = "rustix")]
use rustix::fd::{AsFd, AsRawFd, BorrowedFd, OwnedFd, RawFd};
#[cfg(feature = "libc")]
use std::{
    fs,
    os::unix::{
        io::{IntoRawFd, RawFd},
        prelude::AsRawFd,
    },
};

/// A file descriptor wrapper.
///
/// It allows to retrieve raw file descriptor, write to the file descriptor and
/// mainly it closes the file descriptor once dropped.
#[derive(Debug)]
#[cfg(feature = "libc")]
pub struct FileDesc {
    fd: RawFd,
    close_on_drop: bool,
}

#[cfg(feature = "rustix")]
pub enum FileDesc {
    Owned(OwnedFd),
    Static(BorrowedFd<'static>),
}

#[cfg(feature = "libc")]
impl FileDesc {
    /// Constructs a new `FileDesc` with the given `RawFd`.
    ///
    /// # Arguments
    ///
    /// * `fd` - raw file descriptor
    /// * `close_on_drop` - specify if the raw file descriptor should be closed once the `FileDesc` is dropped
    pub fn new(fd: RawFd, close_on_drop: bool) -> FileDesc {
        FileDesc { fd, close_on_drop }
    }

    pub fn read(&self, buffer: &mut [u8]) -> io::Result<usize> {
        let result = unsafe {
            libc::read(
                self.fd,
                buffer.as_mut_ptr() as *mut libc::c_void,
                buffer.len() as size_t,
            )
        };

        if result < 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(result as usize)
        }
    }

    /// Returns the underlying file descriptor.
    pub fn raw_fd(&self) -> RawFd {
        self.fd
    }
}

#[cfg(feature = "rustix")]
impl FileDesc {
    pub fn read(&self, buffer: &mut [u8]) -> io::Result<usize> {
        let fd = match self {
            FileDesc::Owned(fd) => fd.as_fd(),
            FileDesc::Static(fd) => fd.as_fd(),
        };
        let result = rustix::io::read(fd, buffer)?;
        Ok(result)
    }

    pub fn raw_fd(&self) -> RawFd {
        match self {
            FileDesc::Owned(fd) => fd.as_raw_fd(),
            FileDesc::Static(fd) => fd.as_raw_fd(),
        }
    }
}

#[cfg(feature = "libc")]
impl Drop for FileDesc {
    fn drop(&mut self) {
        if self.close_on_drop {
            // Note that errors are ignored when closing a file descriptor. The
            // reason for this is that if an error occurs we don't actually know if
            // the file descriptor was closed or not, and if we retried (for
            // something like EINTR), we might close another valid file descriptor
            // opened after we closed ours.
            let _ = unsafe { libc::close(self.fd) };
        }
    }
}

impl AsRawFd for FileDesc {
    fn as_raw_fd(&self) -> RawFd {
        self.raw_fd()
    }
}

#[cfg(feature = "rustix")]
impl AsFd for FileDesc {
    fn as_fd(&self) -> BorrowedFd<'_> {
        match self {
            FileDesc::Owned(fd) => fd.as_fd(),
            FileDesc::Static(fd) => fd.as_fd(),
        }
    }
}

#[cfg(feature = "libc")]
/// Creates a file descriptor pointing to the standard input or `/dev/tty`.
pub fn tty_fd() -> io::Result<FileDesc> {
    let (fd, close_on_drop) = if unsafe { libc::isatty(libc::STDIN_FILENO) == 1 } {
        (libc::STDIN_FILENO, false)
    } else {
        (
            fs::OpenOptions::new()
                .read(true)
                .write(true)
                .open("/dev/tty")?
                .into_raw_fd(),
            true,
        )
    };

    Ok(FileDesc::new(fd, close_on_drop))
}

#[cfg(feature = "rustix")]
/// Creates a file descriptor pointing to the standard input or `/dev/tty`.
pub fn tty_fd() -> io::Result<FileDesc> {
    use std::fs::File;

    let stdin = rustix::stdio::stdin();
    let fd = if rustix::termios::isatty(stdin) {
        FileDesc::Static(stdin)
    } else {
        let dev_tty = File::options().read(true).write(true).open("/dev/tty")?;
        FileDesc::Owned(dev_tty.into())
    };
    Ok(fd)
}
