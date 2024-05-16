//! Emulates rustix's interface using libc.

pub(crate) mod io {
    use super::cvt;
    use std::io::Result;
    use std::os::unix::io::{AsFd, AsRawFd};

    pub(crate) fn read(f: impl AsFd, buf: &mut [u8]) -> Result<usize> {
        unsafe {
            cvt(libc::read(
                f.as_fd().as_raw_fd(),
                buf.as_mut_ptr().cast(),
                buf.len() as _,
            ) as i32)
            .map(|x| x as usize)
        }
    }
}

pub(crate) mod stdio {
    use std::os::unix::io::BorrowedFd;

    pub(crate) fn stdin() -> BorrowedFd<'static> {
        unsafe { BorrowedFd::borrow_raw(libc::STDIN_FILENO) }
    }

    pub(crate) fn stdout() -> BorrowedFd<'static> {
        unsafe { BorrowedFd::borrow_raw(libc::STDOUT_FILENO) }
    }
}

pub(crate) mod termios {
    use super::cvt;
    use std::io::Result;
    use std::mem::MaybeUninit;
    use std::os::unix::io::{AsFd, AsRawFd, BorrowedFd};

    pub(crate) type Termios = libc::termios;
    pub(crate) type Winsize = libc::winsize;

    #[repr(u32)]
    pub(crate) enum OptionalActions {
        Now = 0,
        // All others are unused by crossterm.
    }

    pub(crate) fn isatty(fd: BorrowedFd<'_>) -> bool {
        unsafe { libc::isatty(fd.as_raw_fd()) != 0 }
    }

    pub(crate) fn tcgetwinsize(fd: impl AsFd) -> Result<Winsize> {
        unsafe {
            let mut buf = MaybeUninit::<Winsize>::uninit();
            cvt(libc::ioctl(
                fd.as_fd().as_raw_fd(),
                libc::TIOCGWINSZ,
                buf.as_mut_ptr(),
            ))?;
            Ok(buf.assume_init())
        }
    }

    pub(crate) fn tcgetattr(fd: impl AsFd) -> Result<Termios> {
        unsafe {
            let mut buf = MaybeUninit::<Termios>::uninit();
            cvt(libc::tcgetattr(fd.as_fd().as_raw_fd(), buf.as_mut_ptr()))?;
            Ok(buf.assume_init())
        }
    }

    pub(crate) fn tcsetattr(
        fd: impl AsFd,
        optional: OptionalActions,
        termios: &Termios
    ) -> Result<()> {
        unsafe {
            cvt(libc::tcsetattr(
                fd.as_fd().as_raw_fd(),
                optional as _,
                termios
            ))?;

            Ok(())
        }
    }
}

fn cvt(res: i32) -> std::io::Result<i32> {
    if res < 0 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(res)
    }
}
