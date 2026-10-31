//! [OSC 7501] program-status reports (protocol revision 0.4).
//!
//! Reports replace records completely: repeat metadata such as `app` on every update.
//! Status is independent of the screen buffer and survives alternate-screen transitions.
//! Applications own status lifetimes and decide when to query support. Nothing is emitted automatically.
//! This module requires the `osc7501` feature and does not require the `events` feature.
//!
//! # Terminal support
//!
//! Successful execution means the report was written and flushed, not that the terminal supports
//! the protocol. The terminal and any intervening multiplexer must support OSC 7501.
//! There is no WinAPI equivalent; execution returns [`std::io::ErrorKind::Unsupported`] when
//! Crossterm uses the legacy Windows API instead of ANSI output.
//!
//! The protocol's detection query is `ESC ] 7501 ; ? ESC \`.
//! A supporting terminal replies with OSC 7501 and body `?`; accept either `ESC \` or BEL as
//! the terminator, rather than matching the query's bytes exactly. Future revisions may append
//! pairs after `?`; applications must ignore suffix extensions they do not understand. Treat no
//! reply within an application-chosen timeout as unsupported. Missing or stale terminfo entries
//! do not establish lack of support.
//!
//! With `events` enabled, `supports_program_status` performs this query on Unix. OSC 7501
//! acknowledgements are consumed internally during that query. Without `events`, this module
//! remains output-only.
//! Reply recognition is limited to an active query; enabling the feature does not change
//! normal Alt+`]` handling. During detection, a possible reply has a 4096-byte candidate limit
//! so fragmented replies are not mistaken for keystrokes. Non-replies, oversized candidates,
//! and unfinished candidates at query completion go through the normal input parser instead
//! of being discarded. Late replies also go through normal input handling. Input identical to
//! a complete acknowledgement is inherently indistinguishable from a terminal reply while
//! the query is active.
//!
//! Report [`ProgramState::Idle`](crate::program_status::ProgramState::Idle) on cancellation, and
//! [`ProgramState::Done`](crate::program_status::ProgramState::Done) or
//! [`ProgramState::Error`](crate::program_status::ProgramState::Error) before exiting after work.
//! Terminals discard working and blocked
//! records on process exit or the next shell prompt, but retain done and error records until
//! replaced or cleared. This command does not manage those transitions for the application.
//!
//! [OSC 7501]: https://www.superlogical.com/rex/docs/build/program-status

use std::fmt;

use base64::prelude::{BASE64_STANDARD, Engine};

use crate::{Command, impl_display};

/// Explicitly queries whether the terminal supports OSC 7501.
///
/// Requires the `events` feature. On Unix, enable raw mode first and call this before starting
/// an input-reading thread or `EventStream`. This function holds Crossterm's
/// input-reader lock while querying; unrelated input remains available in its original order.
/// It does not change raw mode or cache results. Failure to acquire the input lock or receive
/// an acknowledgement within `timeout` returns `Ok(false)`; I/O failures return `Err`.
/// The timeout bounds waiting for input and the reader lock, not blocking output writes.
/// No DA1 query is sent, so missing or reordered DA1 responses cannot stall detection.
/// Optional state/kind lists are ignored: `true` confirms protocol support, not support for
/// every state or kind. Query once during startup; the protocol has no request identifiers
/// with which to distinguish a delayed reply to an earlier request.
///
/// On Windows this returns `Ok(false)` without querying: Crossterm's Windows input backend
/// cannot parse OSC replies. This does not prove the terminal itself lacks support.
///
/// ```no_run
/// use std::time::Duration;
/// use crossterm::{program_status::supports_program_status, terminal};
///
/// terminal::enable_raw_mode()?;
/// let result = supports_program_status(Duration::from_millis(500));
/// terminal::disable_raw_mode()?;
/// let supported = result?;
/// # Ok::<(), std::io::Error>(())
/// ```
#[cfg(feature = "events")]
pub fn supports_program_status(timeout: std::time::Duration) -> std::io::Result<bool> {
    #[cfg(windows)]
    {
        let _ = timeout;
        Ok(false)
    }
    #[cfg(unix)]
    {
        use crate::event::{internal, timeout::PollTimeout};
        use std::io::{self, Write};

        if !crate::terminal::is_raw_mode_enabled()? {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "enable raw mode before querying OSC 7501 support",
            ));
        }
        let deadline = PollTimeout::new(Some(timeout));
        let Some(mut reader) = internal::try_lock_event_reader_for(timeout) else {
            return Ok(false);
        };
        // Drop acknowledgements already queued by earlier queries, without moving ordinary input.
        while reader.try_read(&ProgramStatusFilter).is_some() {}
        if deadline.elapsed() {
            return Ok(false);
        }
        reader.set_program_status_query(true)?;
        let result = (|| {
            let mut tty = std::fs::OpenOptions::new().write(true).open("/dev/tty");
            let mut stdout = io::stdout();
            let output: &mut dyn Write = match &mut tty {
                Ok(file) => file,
                Err(_) => &mut stdout,
            };
            output.write_all(b"\x1b]7501;?\x1b\\")?;
            output.flush()?;
            if reader.poll(deadline.leftover(), &ProgramStatusFilter)? {
                Ok(reader.try_read(&ProgramStatusFilter).is_some())
            } else {
                Ok(false)
            }
        })();
        // Restore Escape-key handling on every I/O result, including write failures.
        reader.set_program_status_query(false)?;
        result
    }
}

#[cfg(all(unix, feature = "events"))]
struct ProgramStatusFilter;

#[cfg(all(unix, feature = "events"))]
impl crate::event::filter::Filter for ProgramStatusFilter {
    fn eval(&self, event: &crate::event::internal::InternalEvent) -> bool {
        matches!(
            event,
            crate::event::internal::InternalEvent::ProgramStatusSupport
        )
    }
}

/// The state reported by a program.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProgramState {
    /// Waiting for the user's next instruction.
    Idle,
    /// Running, optionally with progress.
    Working,
    /// Finished; the record survives process exit and shell prompts.
    Done,
    /// Waiting for user action, optionally with a reason and progress.
    Blocked,
    /// Failed and stopped; the record survives process exit and shell prompts.
    Error,
    /// Remove the addressed record and its descendants; without an id, remove all records.
    Clear,
}

impl ProgramState {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::Working => "working",
            Self::Done => "done",
            Self::Blocked => "blocked",
            Self::Error => "error",
            Self::Clear => "clear",
        }
    }
}

/// The action needed to unblock a program.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockedKind {
    /// Approval to perform an action.
    Permission,
    /// An answer to a question.
    Question,
    /// Authentication or credentials.
    Auth,
}

impl BlockedKind {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Permission => "permission",
            Self::Question => "question",
            Self::Auth => "auth",
        }
    }
}

/// An invalid program-status field. Invalid values are rejected, never truncated.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProgramStatusError(&'static str);

impl fmt::Display for ProgramStatusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid program-status field: {}", self.0)
    }
}

impl std::error::Error for ProgramStatusError {}

/// A command that emits a validated, complete OSC 7501 program-status report.
///
/// Omitted fields are removed from the terminal's record when this report is applied. An absent
/// id addresses the root record. Child records inherit `app` from the nearest ancestor with one.
/// Commands must be executed or queued for execution; constructing a command emits nothing.
/// Validation happens during construction, before any bytes are written.
///
/// ```no_run
/// use crossterm::ExecutableCommand;
/// use crossterm::program_status::{BlockedKind, ProgramState, SetProgramStatus};
///
/// let status = SetProgramStatus::new(ProgramState::Blocked)
///     .with_app("deploy")?
///     .with_id("eu-west/production")?
///     .with_kind(BlockedKind::Permission)?
///     .with_message("Approve deployment?")?;
/// std::io::stdout().execute(status)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetProgramStatus<'a> {
    state: ProgramState,
    id: Option<&'a str>,
    app: Option<&'a str>,
    title: Option<&'a str>,
    message: Option<&'a str>,
    kind: Option<BlockedKind>,
    progress: Option<u8>,
}

impl<'a> SetProgramStatus<'a> {
    /// Creates a report for the root record with no optional metadata.
    pub const fn new(state: ProgramState) -> Self {
        Self {
            state,
            id: None,
            app: None,
            title: None,
            message: None,
            kind: None,
            progress: None,
        }
    }

    /// Addresses a child record. Ids have at most eight slash-separated segments, each 1–32
    /// ASCII letters, digits, `_`, `.`, `+`, or `-`, with at most 128 bytes in total.
    pub fn with_id(mut self, id: &'a str) -> Result<Self, ProgramStatusError> {
        if id.len() > 128 || id.split('/').count() > 8 || !id.split('/').all(valid_segment) {
            return Err(ProgramStatusError("id"));
        }
        self.id = Some(id);
        Ok(self)
    }

    /// Sets the machine-readable app name: 1–32 ASCII letters, digits, `_`, `.`, `+`, or `-`.
    pub fn with_app(mut self, app: &'a str) -> Result<Self, ProgramStatusError> {
        if !valid_segment(app) {
            return Err(ProgramStatusError("app"));
        }
        self.app = Some(app);
        Ok(self)
    }

    /// Sets a label of at most 192 UTF-8 bytes, without C0, DEL, or C1 control characters.
    pub fn with_title(mut self, title: &'a str) -> Result<Self, ProgramStatusError> {
        validate_text(title, 192, "title")?;
        self.title = Some(title);
        Ok(self)
    }

    /// Sets a message of at most 2048 UTF-8 bytes, without C0, DEL, or C1 control characters.
    pub fn with_message(mut self, message: &'a str) -> Result<Self, ProgramStatusError> {
        validate_text(message, 2048, "msg")?;
        self.message = Some(message);
        Ok(self)
    }

    /// Sets why the program is blocked. Returns an error for any other state.
    pub fn with_kind(mut self, kind: BlockedKind) -> Result<Self, ProgramStatusError> {
        if self.state != ProgramState::Blocked {
            return Err(ProgramStatusError("kind"));
        }
        self.kind = Some(kind);
        Ok(self)
    }

    /// Sets progress from 0 through 100. Only working and blocked reports support progress.
    pub const fn with_progress(mut self, progress: u8) -> Result<Self, ProgramStatusError> {
        if progress > 100 || !matches!(self.state, ProgramState::Working | ProgramState::Blocked) {
            return Err(ProgramStatusError("progress"));
        }
        self.progress = Some(progress);
        Ok(self)
    }
}

// Every field is bounded before construction, so even the largest encoded report is under
// the protocol's 4096-byte cap. Serialization cannot fail validation after output has started.
impl Command for SetProgramStatus<'_> {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        write!(f, "\x1b]7501;state={}", self.state.as_str())?;
        if let Some(id) = self.id {
            write!(f, ":id={id}")?;
        }
        if let Some(app) = self.app {
            write!(f, ":app={app}")?;
        }
        if let Some(kind) = self.kind {
            write!(f, ":kind={}", kind.as_str())?;
        }
        if let Some(progress) = self.progress {
            write!(f, ":progress={progress}")?;
        }
        for (key, value) in [("title", self.title), ("msg", self.message)] {
            if let Some(value) = value {
                let mut encoded = String::new();
                BASE64_STANDARD.encode_string(value, &mut encoded);
                write!(f, ":{key}={encoded}")?;
            }
        }
        f.write_str("\x1b\\")
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> std::io::Result<()> {
        Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "Program status is not supported by the Windows API.",
        ))
    }
}

impl_display!(for SetProgramStatus<'_>);

fn valid_segment(value: &str) -> bool {
    (1..=32).contains(&value.len())
        && value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"_.+-".contains(&b))
}

fn validate_text(value: &str, limit: usize, field: &'static str) -> Result<(), ProgramStatusError> {
    if value.len() > limit
        || value
            .chars()
            .any(|c| matches!(c, '\u{0}'..='\u{1f}' | '\u{7f}'..='\u{9f}'))
    {
        return Err(ProgramStatusError(field));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Serialize directly so these tests do not depend on Windows console ANSI support.
    fn encoded(command: &SetProgramStatus<'_>) -> String {
        let mut output = String::new();
        command.write_ansi(&mut output).unwrap();
        output
    }

    #[test]
    fn encodes_complete_report() {
        let status = SetProgramStatus::new(ProgramState::Blocked)
            .with_id("build/test")
            .unwrap()
            .with_app("cargo")
            .unwrap()
            .with_kind(BlockedKind::Question)
            .unwrap()
            .with_progress(37)
            .unwrap()
            .with_title("é")
            .unwrap()
            .with_message("Go?")
            .unwrap();
        assert_eq!(
            encoded(&status),
            "\x1b]7501;state=blocked:id=build/test:app=cargo:kind=question:progress=37:title=w6k=:msg=R28/\x1b\\"
        );
    }

    #[test]
    fn encodes_states_and_clear_targets() {
        for (state, name) in [
            (ProgramState::Idle, "idle"),
            (ProgramState::Working, "working"),
            (ProgramState::Done, "done"),
            (ProgramState::Blocked, "blocked"),
            (ProgramState::Error, "error"),
            (ProgramState::Clear, "clear"),
        ] {
            assert_eq!(
                encoded(&SetProgramStatus::new(state)),
                format!("\x1b]7501;state={name}\x1b\\")
            );
        }
        assert_eq!(
            encoded(
                &SetProgramStatus::new(ProgramState::Clear)
                    .with_id("build")
                    .unwrap()
            ),
            "\x1b]7501;state=clear:id=build\x1b\\"
        );
        for (kind, name) in [
            (BlockedKind::Permission, "permission"),
            (BlockedKind::Question, "question"),
            (BlockedKind::Auth, "auth"),
        ] {
            assert_eq!(
                encoded(
                    &SetProgramStatus::new(ProgramState::Blocked)
                        .with_kind(kind)
                        .unwrap()
                ),
                format!("\x1b]7501;state=blocked:kind={name}\x1b\\")
            );
        }
    }

    #[test]
    fn validates_identifiers() {
        let status = SetProgramStatus::new(ProgramState::Working);
        for id in [
            "",
            "/a",
            "a/",
            "a//b",
            "a:b",
            "a=b",
            "é",
            "a/b/c/d/e/f/g/h/i",
        ] {
            assert!(status.clone().with_id(id).is_err(), "{id}");
        }
        let segment = "x".repeat(32);
        assert!(status.clone().with_app(&segment).is_ok());
        assert!(status.clone().with_app(&"x".repeat(33)).is_err());
        assert!(status.clone().with_id(&"x".repeat(33)).is_err());
        assert!(status.clone().with_app("Aa0_.+-").is_ok());
        assert!(status.clone().with_id("Aa0_.+-/child").is_ok());
        for app in ["", "a/b", "a:b", "a=b", "a b", "é", "a\x1b\\"] {
            assert!(status.clone().with_app(app).is_err(), "{app:?}");
        }
        assert!(status.clone().with_id("a/b/c/d/e/f/g/h").is_ok());
        let id = format!("{segment}/{segment}/{segment}/{}", "x".repeat(29));
        assert_eq!(id.len(), 128);
        assert!(status.clone().with_id(&id).is_ok());
        assert!(status.with_id(&format!("{id}x")).is_err());
    }

    #[test]
    fn validates_text_bytes_and_controls() {
        let status = SetProgramStatus::new(ProgramState::Done);
        assert!(status.clone().with_title(&"é".repeat(96)).is_ok());
        assert!(status.clone().with_title(&"é".repeat(97)).is_err());
        assert!(status.clone().with_message(&"x".repeat(2048)).is_ok());
        assert!(status.clone().with_message(&"x".repeat(2049)).is_err());
        for code in (0..=31).chain(127..=159) {
            let text = char::from_u32(code).unwrap().to_string();
            assert!(status.clone().with_title(&text).is_err());
            assert!(status.clone().with_message(&text).is_err());
        }
        assert!(status.clone().with_message("a:;=雪").is_ok());
        assert_eq!(
            encoded(&status.with_message("").unwrap()),
            "\x1b]7501;state=done:msg=\x1b\\"
        );
    }

    #[test]
    fn validates_state_specific_fields() {
        for state in [ProgramState::Working, ProgramState::Blocked] {
            for progress in 0..=u8::MAX {
                assert_eq!(
                    SetProgramStatus::new(state).with_progress(progress).is_ok(),
                    progress <= 100
                );
            }
        }
        for state in [
            ProgramState::Idle,
            ProgramState::Done,
            ProgramState::Error,
            ProgramState::Clear,
        ] {
            assert!(SetProgramStatus::new(state).with_progress(50).is_err());
            assert!(
                SetProgramStatus::new(state)
                    .with_kind(BlockedKind::Auth)
                    .is_err()
            );
        }
        assert!(
            SetProgramStatus::new(ProgramState::Working)
                .with_kind(BlockedKind::Auth)
                .is_err()
        );
    }

    #[test]
    fn largest_report_fits_protocol_limits() {
        let id = format!("{0}/{0}/{0}/{1}", "x".repeat(32), "x".repeat(29));
        let app = "x".repeat(32);
        let title = "x".repeat(192);
        let msg = "x".repeat(2048);
        let report = SetProgramStatus::new(ProgramState::Blocked)
            .with_id(&id)
            .unwrap()
            .with_app(&app)
            .unwrap()
            .with_title(&title)
            .unwrap()
            .with_message(&msg)
            .unwrap()
            .with_kind(BlockedKind::Permission)
            .unwrap()
            .with_progress(100)
            .unwrap();
        assert!(encoded(&report).len() < 3300);
    }

    #[test]
    fn serialization_forwards_formatter_errors() {
        struct FailingFormatter;
        impl fmt::Write for FailingFormatter {
            fn write_str(&mut self, _: &str) -> fmt::Result {
                Err(fmt::Error)
            }
        }
        assert!(
            SetProgramStatus::new(ProgramState::Done)
                .write_ansi(&mut FailingFormatter)
                .is_err()
        );
    }

    #[cfg(windows)]
    #[test]
    fn winapi_is_explicitly_unsupported() {
        assert_eq!(
            SetProgramStatus::new(ProgramState::Working)
                .execute_winapi()
                .unwrap_err()
                .kind(),
            std::io::ErrorKind::Unsupported
        );
    }

    #[cfg(not(windows))]
    #[test]
    fn command_api_preserves_queue_flush_and_error_semantics() {
        use std::io::{self, Write};

        use crate::{ExecutableCommand, QueueableCommand};

        #[derive(Default)]
        struct Writer {
            bytes: Vec<u8>,
            flushes: usize,
            fail_write: bool,
            fail_flush: bool,
        }

        impl Write for Writer {
            fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
                if self.fail_write {
                    return Err(io::ErrorKind::BrokenPipe.into());
                }
                // Force short writes; a single write() would truncate the report.
                let count = bytes.len().min(3);
                self.bytes.extend_from_slice(&bytes[..count]);
                Ok(count)
            }

            fn flush(&mut self) -> io::Result<()> {
                self.flushes += 1;
                if self.fail_flush {
                    return Err(io::ErrorKind::Other.into());
                }
                Ok(())
            }
        }

        let status = SetProgramStatus::new(ProgramState::Working)
            .with_app("cargo")
            .unwrap()
            .with_progress(37)
            .unwrap()
            .with_message("Build")
            .unwrap();
        let mut writer = Writer::default();
        writer.queue(&status).unwrap();
        assert_eq!(
            writer.bytes,
            b"\x1b]7501;state=working:app=cargo:progress=37:msg=QnVpbGQ=\x1b\\"
        );
        assert_eq!(writer.flushes, 0);
        writer
            .execute(SetProgramStatus::new(ProgramState::Done))
            .unwrap();
        assert_eq!(writer.bytes, b"\x1b]7501;state=working:app=cargo:progress=37:msg=QnVpbGQ=\x1b\\\x1b]7501;state=done\x1b\\");
        assert_eq!(writer.flushes, 1);
        assert_eq!(
            status.to_string(),
            "\x1b]7501;state=working:app=cargo:progress=37:msg=QnVpbGQ=\x1b\\"
        );

        writer.fail_write = true;
        let error = writer.execute(&status).err().unwrap();
        assert_eq!(error.kind(), io::ErrorKind::BrokenPipe);
        assert_eq!(writer.flushes, 1);
        writer.fail_write = false;
        writer.fail_flush = true;
        let error = writer.execute(&status).err().unwrap();
        assert_eq!(error.kind(), io::ErrorKind::Other);
        assert_eq!(writer.flushes, 2);
    }
}
