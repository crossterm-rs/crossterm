use super::internal::InternalEvent;
use crate::event::source::no_tty::NoTtyInternalEventSource;
use crate::event::{filter::Filter, read::InternalEventReader};
use crate::terminal::WindowSize;
use bytes::Bytes;
use parking_lot::Mutex;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::sync::Mutex as AsyncMutex;

#[derive(Clone)]
pub struct NoTtyEvent {
    pub(crate) send: Sender<Bytes>,
    pub window_size: Arc<Mutex<WindowSize>>,
    inner: Arc<AsyncMutex<InternalEventReader>>,
}

impl NoTtyEvent {
    /// Creates a new no-tty event handle.
    ///
    /// `recv` is the channel that carries raw input bytes (e.g. from an SSH client) into
    /// crossterm's parser. The returned [`Receiver`] carries crossterm's outgoing query
    /// escape sequences (cursor position, keyboard enhancement) back to the host.
    pub fn new(recv: Receiver<Bytes>) -> (Self, Receiver<Bytes>) {
        let (s, r) = channel(16);
        let source = NoTtyInternalEventSource::new(recv);
        let source = source.ok().map(Box::new);
        let event = InternalEventReader::default().with_source(source);

        (
            Self {
                send: s,
                window_size: Arc::new(Mutex::new(WindowSize {
                    rows: 0,
                    columns: 0,
                    width: 0,
                    height: 0,
                })),
                inner: Arc::new(AsyncMutex::new(event)),
            },
            r,
        )
    }

    /// Polls to check if there are any `InternalEvent`s that can be read within the given duration.
    pub(crate) async fn poll<F>(
        &self,
        timeout: Option<Duration>,
        filter: &F,
    ) -> std::io::Result<bool>
    where
        F: Filter,
    {
        match timeout {
            // Bound the whole operation (lock acquisition + read) by the timeout. If it
            // elapses we report "no event", matching the old try_lock_for behavior.
            Some(timeout) => {
                match tokio::time::timeout(timeout, async {
                    let mut reader = self.inner.lock().await;
                    reader.poll_async(Some(timeout), filter).await
                })
                .await
                {
                    Ok(res) => res,
                    Err(_elapsed) => Ok(false),
                }
            }
            None => {
                let mut reader = self.inner.lock().await;
                reader.poll_async(None, filter).await
            }
        }
    }

    /// Reads a single `InternalEvent`.
    pub(crate) async fn read<F>(&self, filter: &F) -> std::io::Result<InternalEvent>
    where
        F: Filter,
    {
        let mut reader = self.inner.lock().await;
        reader.read_async(filter).await
    }

    /// Reads a single `InternalEvent`. Non-blocking.
    pub(crate) async fn try_read<F>(&self, filter: &F) -> Option<InternalEvent>
    where
        F: Filter,
    {
        let mut reader = self.inner.lock().await;
        reader.try_read(filter)
    }
}

/// An async writer over an mpsc channel, used to forward crossterm command output
/// (ANSI escape sequences) to the host that owns the receiving end.
#[derive(Clone)]
pub struct SenderWriter(tokio::sync::mpsc::Sender<Bytes>);

impl SenderWriter {
    pub fn new(sender: tokio::sync::mpsc::Sender<Bytes>) -> Self {
        Self(sender)
    }

    pub fn blocking_write_all(&self, buf: &[u8]) -> std::io::Result<()> {
        self.0
            .blocking_send(Bytes::copy_from_slice(buf))
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::BrokenPipe, e))
    }

    /// Sends the given bytes over the channel, awaiting capacity.
    pub async fn write_all(&self, buf: &[u8]) -> std::io::Result<()> {
        self.0
            .send(Bytes::copy_from_slice(buf))
            .await
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::BrokenPipe, e))
    }
}

#[cfg(test)]
mod tests {
    use super::NoTtyEvent;
    use crate::event::{read, Event, KeyCode, KeyModifiers};
    use bytes::Bytes;

    #[tokio::test]
    async fn read_parses_byte_fed_through_input_channel() {
        // Input channel: host -> crossterm parser.
        let (input_tx, input_rx) = tokio::sync::mpsc::channel::<Bytes>(8);
        let (pty, _query_rx) = NoTtyEvent::new(input_rx);

        // Feed a plain 'a' keystroke.
        input_tx.send(Bytes::from_static(b"a")).await.unwrap();

        let event = read(&pty).await.unwrap();
        assert_eq!(event, Event::Key(KeyCode::Char('a').into()));

        // Keep the modifiers assertion explicit so the test documents intent.
        if let Event::Key(key) = event {
            assert_eq!(key.modifiers, KeyModifiers::NONE);
        }
    }
}
