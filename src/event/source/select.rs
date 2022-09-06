use std::{fmt::Debug, mem::MaybeUninit, time::Duration};

use crate::Result;

#[repr(transparent)]
#[derive(Clone)]
pub struct FdSet(libc::fd_set);

impl Default for FdSet {
    fn default() -> Self {
        let mut fd_set = MaybeUninit::<libc::fd_set>::uninit();
        FdSet(unsafe {
            libc::FD_ZERO(fd_set.as_mut_ptr());
            fd_set.assume_init()
        })
    }
}

impl Debug for FdSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut set = f.debug_set();
        for i in 0..libc::FD_SETSIZE {
            if self.is_set(i as i32) {
                set.entry(&i);
            }
        }
        set.finish()
    }
}

impl FdSet {
    #[inline]
    pub fn set(&mut self, fd: i32) {
        unsafe { libc::FD_SET(fd, self.as_mut_ptr()) }
    }

    #[inline]
    pub fn clear(&mut self, fd: i32) {
        unsafe { libc::FD_CLR(fd, self.as_mut_ptr()) }
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut libc::fd_set {
        &mut self.0 as *mut _
    }

    #[inline]
    pub fn as_ptr(&self) -> *const libc::fd_set {
        &self.0 as *const _
    }
    #[inline]
    pub fn is_set(&self, fd: i32) -> bool {
        unsafe { libc::FD_ISSET(fd, self.as_ptr()) }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct FdSelector {
    pub fd: i32,
    pub read: bool,
    pub write: bool,
    pub error: bool,
}

impl FdSelector {
    pub fn new(fd: i32, read: bool, write: bool, error: bool) -> Self {
        FdSelector {
            fd,
            read,
            write,
            error,
        }
    }

    pub fn read(fd: i32) -> Self {
        FdSelector {
            fd,
            read: true,
            write: false,
            error: false,
        }
    }

    pub fn write(fd: i32) -> Self {
        FdSelector {
            fd,
            read: false,
            write: true,
            error: false,
        }
    }

    pub fn error(fd: i32) -> Self {
        FdSelector {
            fd,
            read: false,
            write: false,
            error: true,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub(crate) struct Selector {
    read: FdSet,
    write: FdSet,
    error: FdSet,
    max_fd: i32,
}

impl Selector {
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
        let result = unsafe { libc::select(self.max_fd + 1, read, write, error, timeval_ptr) };

        if result >= 0 {
            Ok(result as usize)
        } else {
            Err(std::io::Error::last_os_error())
        }
    }

    #[inline]
    pub fn set(&mut self, selector: FdSelector) -> &mut Self {
        if selector.read {
            self.read.set(selector.fd)
        }
        if selector.write {
            self.write.set(selector.fd)
        }
        if selector.error {
            self.error.set(selector.fd)
        }
        self.max_fd = self.max_fd.max(selector.fd);
        self
    }
    #[inline]
    pub fn clear(&mut self, selector: FdSelector) -> &mut Self {
        if selector.read {
            self.read.clear(selector.fd)
        }
        if selector.write {
            self.write.clear(selector.fd)
        }
        if selector.error {
            self.error.clear(selector.fd)
        }
        self
    }

    pub fn get(&self, fd: i32) -> Option<FdSelector> {
        let read = self.read.is_set(fd);
        let write = self.write.is_set(fd);
        let error = self.error.is_set(fd);
        if read | write | error {
            Some(FdSelector {
                fd,
                read,
                write,
                error,
            })
        } else {
            None
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = FdSelector> + '_ {
        (0..=self.max_fd).filter_map(|i| self.get(i))
    }
}
