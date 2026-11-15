use std::io::{self, Write};

use crate::{ExecutableCommand, terminal::supports_keyboard_enhancement};

use super::{KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags};

/// Attempts to enable optional keyboard enhancement on the current terminal.
///
/// Returns an inactive guard when the support query reports `false` or the query/activation
/// returns [`io::ErrorKind::Unsupported`]. Other errors are returned to the caller.
/// The current Windows backend always reports no support.
///
/// `writer` must address the same terminal as Crossterm's capability query. The query uses
/// Crossterm's terminal input/output, not this writer. Run this function before starting an
/// event-reading loop or `EventStream`: on Unix, concurrent event reads can block the
/// query or cause it to time out. A timeout is an error, not an unsupported result.
///
/// Activation writes and flushes a push command. Success means the command was sent, not that
/// the terminal acknowledged every requested flag. A failed write or flush may have partially
/// changed terminal state; no automatic pop is attempted when activation fails.
///
/// # Example
///
/// ```no_run
/// use std::io;
/// use crossterm::event::{try_enable_keyboard_enhancement, KeyboardEnhancementFlags};
///
/// # fn main() -> io::Result<()> {
/// let mut enhancement = try_enable_keyboard_enhancement(
///     io::stdout(),
///     KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES,
/// )?;
/// if enhancement.is_enabled() {
///     // Enhanced input was requested successfully.
/// } else {
///     // Continue using ordinary keyboard input.
/// }
/// // Run the application, enabling raw mode separately when needed.
/// enhancement.disable()?;
/// # Ok(())
/// # }
/// ```
pub fn try_enable_keyboard_enhancement<W: Write>(
    writer: W,
    flags: KeyboardEnhancementFlags,
) -> io::Result<KeyboardEnhancement<W>> {
    enable_with_query(writer, flags, supports_keyboard_enhancement)
}

fn enable_with_query<W: Write>(
    mut writer: W,
    flags: KeyboardEnhancementFlags,
    query: impl FnOnce() -> io::Result<bool>,
) -> io::Result<KeyboardEnhancement<W>> {
    let state = EnhancementState::enable(query, || {
        writer
            .execute(PushKeyboardEnhancementFlags(flags))
            .map(|_| ())
    })?;
    Ok(KeyboardEnhancement { writer, state })
}

/// Owns the writer and cleanup responsibility for optional keyboard enhancement.
///
/// Created by [`try_enable_keyboard_enhancement`]. The writer may itself be a mutable reference.
/// Use [`Self::writer_mut`] to continue writing while the guard exists.
///
/// An active guard attempts one pop on drop, ignoring errors. Call [`Self::disable`] to observe
/// cleanup errors instead. Nested guards and manual pushes must be cleaned up in reverse order
/// on the same terminal; this guard does not coordinate other writers or threads.
/// Raw mode is managed separately. Drop cannot restore state after process abort or termination.
#[derive(Debug)]
#[must_use = "dropping the guard immediately restores keyboard enhancement"]
pub struct KeyboardEnhancement<W: Write> {
    writer: W,
    state: EnhancementState,
}

impl<W: Write> KeyboardEnhancement<W> {
    /// Whether activation succeeded and cleanup has not yet been attempted.
    ///
    /// This tracks this guard's responsibility, not the terminal's current state.
    pub fn is_enabled(&self) -> bool {
        self.state.enabled
    }

    /// Borrows the writer used for activation and cleanup.
    pub fn writer_mut(&mut self) -> &mut W {
        &mut self.writer
    }

    /// Pops and flushes once if activation succeeded; otherwise does nothing.
    ///
    /// Cleanup is not retried, even after an error: a failed write or flush may already have
    /// delivered the pop, and retrying could remove another caller's stack entry. After an
    /// error, terminal state is uncertain, but [`Self::is_enabled`] returns `false`.
    pub fn disable(&mut self) -> io::Result<()> {
        let writer = &mut self.writer;
        self.state
            .disable(|| writer.execute(PopKeyboardEnhancementFlags).map(|_| ()))
    }
}

impl<W: Write> Drop for KeyboardEnhancement<W> {
    fn drop(&mut self) {
        let _ = self.disable();
    }
}

#[derive(Debug, Default)]
struct EnhancementState {
    enabled: bool,
}

impl EnhancementState {
    fn enable(
        query: impl FnOnce() -> io::Result<bool>,
        push: impl FnOnce() -> io::Result<()>,
    ) -> io::Result<Self> {
        match query() {
            Ok(true) => {}
            Ok(false) => return Ok(Self::default()),
            Err(error) if error.kind() == io::ErrorKind::Unsupported => return Ok(Self::default()),
            Err(error) => return Err(error),
        }
        match push() {
            Ok(()) => Ok(Self { enabled: true }),
            Err(error) if error.kind() == io::ErrorKind::Unsupported => Ok(Self::default()),
            Err(error) => Err(error),
        }
    }

    fn disable(&mut self, pop: impl FnOnce() -> io::Result<()>) -> io::Result<()> {
        if std::mem::take(&mut self.enabled) {
            pop()?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unsupported_query_skips_push_and_pop() {
        for result in [Ok(false), Err(io::ErrorKind::Unsupported.into())] {
            let mut state =
                EnhancementState::enable(|| result, || panic!("unexpected push")).unwrap();
            assert!(!state.enabled);
            state.disable(|| panic!("unexpected pop")).unwrap();
        }
    }

    #[test]
    fn unsupported_push_skips_pop() {
        let mut state =
            EnhancementState::enable(|| Ok(true), || Err(io::ErrorKind::Unsupported.into()))
                .unwrap();
        assert!(!state.enabled);
        state.disable(|| panic!("unexpected pop")).unwrap();
    }

    #[test]
    fn unexpected_errors_propagate() {
        for kind in [io::ErrorKind::TimedOut, io::ErrorKind::BrokenPipe] {
            let error = EnhancementState::enable(|| Err(kind.into()), || panic!("unexpected push"))
                .unwrap_err();
            assert_eq!(error.kind(), kind);
            let error = EnhancementState::enable(|| Ok(true), || Err(kind.into())).unwrap_err();
            assert_eq!(error.kind(), kind);
        }
    }

    #[test]
    fn cleanup_is_attempted_once_even_on_error() {
        for result in [Ok(()), Err(io::ErrorKind::BrokenPipe.into())] {
            let mut state = EnhancementState::enable(|| Ok(true), || Ok(())).unwrap();
            assert!(state.enabled);
            let expected = result.as_ref().err().map(io::Error::kind);
            assert_eq!(state.disable(|| result).err().map(|e| e.kind()), expected);
            assert!(!state.enabled);
            state.disable(|| panic!("duplicate pop")).unwrap();
        }
    }

    #[test]
    fn inactive_guard_leaves_writer_available() {
        let mut bytes = Vec::new();
        {
            let mut guard = enable_with_query(
                &mut bytes,
                KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES,
                || Ok(false),
            )
            .unwrap();
            guard.writer_mut().write_all(b"ordinary output").unwrap();
            guard.disable().unwrap();
        }
        assert_eq!(bytes, b"ordinary output");
    }

    #[cfg(unix)]
    #[derive(Debug, Default)]
    struct FailingWriter {
        bytes: Vec<u8>,
        fail_write: bool,
        fail_flush: bool,
        writes: usize,
        flushes: usize,
    }

    #[cfg(unix)]
    impl Write for FailingWriter {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            self.writes += 1;
            if self.fail_write {
                return Err(io::ErrorKind::BrokenPipe.into());
            }
            self.bytes.extend_from_slice(bytes);
            Ok(bytes.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            self.flushes += 1;
            if self.fail_flush {
                return Err(io::ErrorKind::BrokenPipe.into());
            }
            Ok(())
        }
    }

    #[cfg(unix)]
    #[test]
    fn activation_write_and_flush_errors_propagate_without_pop() {
        for fail_write in [true, false] {
            let mut writer = FailingWriter {
                fail_write,
                fail_flush: !fail_write,
                ..Default::default()
            };
            let error = enable_with_query(
                &mut writer,
                KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES,
                || Ok(true),
            )
            .unwrap_err();
            assert_eq!(error.kind(), io::ErrorKind::BrokenPipe);
            assert!(!writer.bytes.windows(5).any(|bytes| bytes == b"\x1b[<1u"));
            assert_eq!(writer.flushes, usize::from(!fail_write));
        }
    }

    #[cfg(unix)]
    #[test]
    fn cleanup_write_and_flush_errors_are_not_retried_on_drop() {
        for fail_write in [true, false] {
            let mut writer = FailingWriter::default();
            let counts;
            {
                let mut guard = enable_with_query(
                    &mut writer,
                    KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES,
                    || Ok(true),
                )
                .unwrap();
                assert_eq!(guard.writer_mut().flushes, 1);
                guard.writer_mut().fail_write = fail_write;
                guard.writer_mut().fail_flush = !fail_write;
                assert_eq!(
                    guard.disable().unwrap_err().kind(),
                    io::ErrorKind::BrokenPipe
                );
                assert!(!guard.is_enabled());
                let writer = guard.writer_mut();
                counts = (writer.writes, writer.flushes);
                guard.disable().unwrap();
            }
            assert_eq!((writer.writes, writer.flushes), counts);
        }
    }

    #[cfg(windows)]
    #[test]
    fn windows_falls_back_without_touching_writer() {
        struct NoWrites;
        impl Write for NoWrites {
            fn write(&mut self, _: &[u8]) -> io::Result<usize> {
                panic!("unexpected write")
            }
            fn flush(&mut self) -> io::Result<()> {
                panic!("unexpected flush")
            }
        }
        let mut guard = try_enable_keyboard_enhancement(
            NoWrites,
            KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES,
        )
        .unwrap();
        assert!(!guard.is_enabled());
        guard.disable().unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn commands_and_nested_cleanup_use_the_same_writer() {
        fn activate<W: Write>(
            writer: W,
            flags: KeyboardEnhancementFlags,
        ) -> KeyboardEnhancement<W> {
            enable_with_query(writer, flags, || Ok(true)).unwrap()
        }
        let mut bytes = Vec::new();
        {
            let mut outer = activate(
                &mut bytes,
                KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES,
            );
            {
                let mut inner = activate(
                    outer.writer_mut(),
                    KeyboardEnhancementFlags::REPORT_EVENT_TYPES,
                );
                inner.writer_mut().write_all(b"hello").unwrap();
                inner.disable().unwrap();
                inner.disable().unwrap();
            }
            // The outer guard cleans up on drop.
        }
        assert_eq!(bytes, b"\x1b[>1u\x1b[>2uhello\x1b[<1u\x1b[<1u");
    }
}
