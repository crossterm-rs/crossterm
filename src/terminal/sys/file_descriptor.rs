use std::{fs::File, io};

use crate::tty::IsTty;
#[cfg(feature = "libc")]
use libc::size_t;
#[cfg(not(feature = "libc"))]
use rustix::fd::{AsFd, AsRawFd, BorrowedFd, OwnedFd, RawFd};
#[cfg(feature = "libc")]
use std::{
    marker::PhantomData,
    os::unix::{io::RawFd, prelude::AsRawFd},
};

/// A file descriptor wrapper.
///
/// It allows to retrieve raw file descriptor, write to the file descriptor and
/// mainly it closes the file descriptor once dropped.
#[derive(Debug)]
#[cfg(feature = "libc")]
pub struct FileDesc<'a> {
    fd: RawFd,
    close_on_drop: bool,
    phantom: PhantomData<&'a ()>,
}

#[cfg(not(feature = "libc"))]
pub enum FileDesc<'a> {
    Owned(OwnedFd),
    Borrowed(BorrowedFd<'a>),
}

#[cfg(feature = "libc")]
impl FileDesc<'_> {
    /// Constructs a new `FileDesc` with the given `RawFd`.
    ///
    /// # Arguments
    ///
    /// * `fd` - raw file descriptor
    /// * `close_on_drop` - specify if the raw file descriptor should be closed once the `FileDesc` is dropped
    pub fn new(fd: RawFd, close_on_drop: bool) -> FileDesc<'static> {
        FileDesc {
            fd,
            close_on_drop,
            phantom: PhantomData,
        }
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

    pub fn write(&self, buffer: &[u8]) -> io::Result<usize> {
        let result = unsafe {
            libc::write(
                self.fd,
                buffer.as_ptr() as *const libc::c_void,
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

#[cfg(not(feature = "libc"))]
impl FileDesc<'_> {
    pub fn read(&self, buffer: &mut [u8]) -> io::Result<usize> {
        let fd = match self {
            FileDesc::Owned(fd) => fd.as_fd(),
            FileDesc::Borrowed(fd) => fd.as_fd(),
        };
        let result = rustix::io::read(fd, buffer)?;
        Ok(result)
    }

    pub fn write(&self, buffer: &[u8]) -> io::Result<usize> {
        let fd = match self {
            FileDesc::Owned(fd) => fd.as_fd(),
            FileDesc::Borrowed(fd) => fd.as_fd(),
        };
        let result = rustix::io::write(fd, buffer)?;
        Ok(result)
    }

    pub fn raw_fd(&self) -> RawFd {
        match self {
            FileDesc::Owned(fd) => fd.as_raw_fd(),
            FileDesc::Borrowed(fd) => fd.as_raw_fd(),
        }
    }
}

#[cfg(feature = "libc")]
impl Drop for FileDesc<'_> {
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

impl AsRawFd for FileDesc<'_> {
    fn as_raw_fd(&self) -> RawFd {
        self.raw_fd()
    }
}

#[cfg(not(feature = "libc"))]
impl AsFd for FileDesc<'_> {
    fn as_fd(&self) -> BorrowedFd<'_> {
        match self {
            FileDesc::Owned(fd) => fd.as_fd(),
            FileDesc::Borrowed(fd) => fd.as_fd(),
        }
    }
}

#[cfg(feature = "libc")]
/// Creates a file descriptor pointing to the standard input or `/dev/tty`.
pub fn tty_fd_in() -> io::Result<FileDesc<'static>> {
    use std::os::fd::IntoRawFd;

    let (fd, close_on_drop) = if libc::STDIN_FILENO.is_tty() {
        (libc::STDIN_FILENO, false)
    } else {
        (open_dev_tty()?.into_raw_fd(), true)
    };
    Ok(FileDesc::new(fd, close_on_drop))
}

#[cfg(feature = "libc")]
/// Creates a file descriptor pointing to the standard output or `/dev/tty`.
pub fn tty_fd_out() -> io::Result<FileDesc<'static>> {
    use std::os::fd::IntoRawFd;

    let (fd, close_on_drop) = if libc::STDOUT_FILENO.is_tty() {
        (libc::STDOUT_FILENO, false)
    } else {
        (open_dev_tty()?.into_raw_fd(), true)
    };
    Ok(FileDesc::new(fd, close_on_drop))
}

#[cfg(not(feature = "libc"))]
/// Creates a file descriptor pointing to the standard input or `/dev/tty`.
pub fn tty_fd_in() -> io::Result<FileDesc<'static>> {
    let stdin = rustix::stdio::stdin();
    let fd = if stdin.is_tty() {
        FileDesc::Borrowed(stdin)
    } else {
        open_dev_tty().map(|file| (FileDesc::Owned(file.into())))?
    };

    Ok(fd)
}

#[cfg(not(feature = "libc"))]
/// Creates a file descriptor pointing to the standard output or `/dev/tty`.
pub fn tty_fd_out() -> io::Result<FileDesc<'static>> {
    let stdout = rustix::stdio::stdout();
    let fd = if stdout.is_tty() {
        FileDesc::Borrowed(stdout)
    } else {
        open_dev_tty().map(|file| (FileDesc::Owned(file.into())))?
    };

    Ok(fd)
}

fn open_dev_tty() -> io::Result<File> {
    File::options().read(true).write(true).open("/dev/tty")
}
