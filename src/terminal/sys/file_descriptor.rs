use std::{fs, io};

use rustix::fd::{AsFd, AsRawFd, BorrowedFd, OwnedFd, RawFd};

/// A file descriptor wrapper.
///
/// It allows to retrieve raw file descriptor, write to the file descriptor and
/// mainly it closes the file descriptor once dropped.
#[derive(Debug)]
pub enum FileDesc {
    Owned(OwnedFd),
    Static(BorrowedFd<'static>),
}

impl FileDesc {
    pub fn read(&self, buffer: &mut [u8]) -> io::Result<usize> {
        let fd = match self {
            Self::Owned(fd) => fd.as_fd(),
            Self::Static(fd) => fd.as_fd(),
        };

        let result = rustix::io::read(fd, buffer)?;
        Ok(result)
    }

    /// Returns the underlying file descriptor.
    pub fn raw_fd(&self) -> RawFd {
        match self {
            Self::Owned(fd) => fd.as_raw_fd(),
            Self::Static(fd) => fd.as_raw_fd(),
        }
    }
}

impl AsFd for FileDesc {
    fn as_fd(&self) -> BorrowedFd<'_> {
        match self {
            Self::Owned(fd) => fd.as_fd(),
            Self::Static(fd) => fd.as_fd(),
        }
    }
}

impl AsRawFd for FileDesc {
    fn as_raw_fd(&self) -> RawFd {
        self.raw_fd()
    }
}

/// Creates a file descriptor pointing to the standard input or `/dev/tty`.
pub fn tty_fd() -> io::Result<FileDesc> {
    if rustix::termios::isatty(rustix::stdio::stdin()) {
        Ok(FileDesc::Static(rustix::stdio::stdin()))
    } else {
        Ok(FileDesc::Owned(
            fs::OpenOptions::new()
                .read(true)
                .write(true)
                .open("/dev/tty")?
                .into(),
        ))
    }
}
