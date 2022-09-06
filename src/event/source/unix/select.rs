use std::{fmt::Debug, mem::MaybeUninit, os::unix::prelude::AsRawFd, time::Duration};

use crate::Result;

#[repr(transparent)]
#[derive(Clone)]
struct FdSet(libc::fd_set);

impl Default for FdSet {
    fn default() -> Self {
        let mut fd_set = MaybeUninit::<libc::fd_set>::uninit();
        FdSet(unsafe {
            libc::FD_ZERO(fd_set.as_mut_ptr());
            // SAFETY: we trust FD_ZERO initializes the values of fd_set to 0
            fd_set.assume_init()
        })
    }
}

impl Debug for FdSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut set = f.debug_set();
        for i in 0..libc::FD_SETSIZE {
            if self.contains(i as i32) {
                set.entry(&i);
            }
        }
        set.finish()
    }
}

impl FdSet {
    #[inline]
    fn set(&mut self, fd: i32) {
        assert!(fd >= 0 && (fd as usize) < libc::FD_SETSIZE);
        // SAFETY: pointer is valid and fd is in bound
        unsafe { libc::FD_SET(fd, self.as_mut_ptr()) }
    }

    #[inline]
    fn contains(&self, fd: i32) -> bool {
        assert!(fd >= 0 && (fd as usize) < libc::FD_SETSIZE);
        // SAFETY: pointer is valid and fd is in bound
        unsafe { libc::FD_ISSET(fd, self.as_ptr()) }
    }

    #[inline]
    fn as_mut_ptr(&mut self) -> *mut libc::fd_set {
        &mut self.0 as *mut _
    }

    #[inline]
    fn as_ptr(&self) -> *const libc::fd_set {
        &self.0 as *const _
    }
}

#[derive(Copy, Clone, Debug)]
pub struct FdResult {
    pub fd: i32,
    pub read: bool,
    pub write: bool,
    pub error: bool,
}

/// Wraps the `select` syscall. Instantiate with `::default()`, then
/// add FDs to select on using `.add()`, and finally call `.select()`.
/// Currently only read and exception events are supportd.
#[derive(Clone, Default, Debug)]
pub(crate) struct Selector {
    read: FdSet,
    write: FdSet,
    error: FdSet,
    max_fd: i32,
}

impl Selector {
    /// Performs the select() syscal. If timeout is None, will block indefinitely.
    /// Updates the internal FD sets of the selector instance, which can later
    /// be accessed by the `get(fd)` method
    pub fn select(&mut self, timeout: Option<Duration>) -> Result<usize> {
        let Selector {
            read, write, error, ..
        } = self;

        let read = read as *mut _ as *mut libc::fd_set;
        let write = write as *mut _ as *mut libc::fd_set;
        let error = error as *mut _ as *mut libc::fd_set;

        let mut timeval = timeout.map(|t| libc::timeval {
            tv_sec: t.as_secs() as libc::time_t,
            tv_usec: t.subsec_micros() as libc::suseconds_t,
        });
        let timeval_ptr = timeval
            .as_mut()
            .map(|timeval| timeval as *mut _)
            .unwrap_or(std::ptr::null_mut());

        // SAFETY:
        // * read/write/error pointers are exclusive and valid
        // * timeval_ptr points to timeval (a local variable) or is null (which is allowed)
        let result = unsafe { libc::select(self.max_fd + 1, read, write, error, timeval_ptr) };

        if result >= 0 {
            Ok(result as usize)
        } else {
            Err(std::io::Error::last_os_error())
        }
    }

    #[inline]
    /// adds a file descriptor to the read and error FD sets of the select operation.
    pub fn add<F: AsRawFd>(&mut self, fd: &F) -> &mut Self {
        let fd = fd.as_raw_fd();
        // Only add to read and error sets. Not supporting write fds for now
        self.read.set(fd);
        self.error.set(fd);
        self.max_fd = self.max_fd.max(fd);
        self
    }

    /// If the given file descriptor exists in any of the FD sets
    /// in the selector, returns an FdResult specifying which sets
    /// it was a part of.
    pub fn get<F: AsRawFd>(&self, fd: &F) -> Option<FdResult> {
        let fd = fd.as_raw_fd();
        let read = self.read.contains(fd);
        let write = self.write.contains(fd);
        let error = self.error.contains(fd);
        if read | write | error {
            Some(FdResult {
                fd,
                read,
                write,
                error,
            })
        } else {
            None
        }
    }
}
