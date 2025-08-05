use std::collections::HashMap;
use std::sync::Arc;

use crossterm::event::{
    poll, KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags,
};
use std::time::Duration;

use crossbeam_channel::{unbounded, Receiver, Sender};
use crossterm::event::NoTtyEvent;
use crossterm::{
    cursor::position,
    event::{
        read, DisableBracketedPaste, DisableFocusChange, DisableMouseCapture, EnableBracketedPaste,
        EnableFocusChange, EnableMouseCapture, Event, KeyCode,
    },
    execute, queue,
    terminal::WindowSize,
};
use russh::keys::ssh_key::PublicKey;
use russh::server::*;
use russh::{Channel, ChannelId, Pty};
use tokio::sync::Mutex;

struct App {
    pub pty: NoTtyEvent,
    pub send: Sender<Vec<u8>>,
    pub recv: Receiver<Vec<u8>>,
}

#[derive(Clone)]
struct AppServer {
    clients: Arc<Mutex<HashMap<usize, App>>>,
    id: usize,
}

impl AppServer {
    pub fn new() -> Self {
        Self {
            clients: Arc::new(Mutex::new(HashMap::new())),
            id: 0,
        }
    }

    pub async fn run(&mut self) -> Result<(), anyhow::Error> {
        let config = Config {
            inactivity_timeout: Some(std::time::Duration::from_secs(3600)),
            auth_rejection_time: std::time::Duration::from_secs(3),
            auth_rejection_time_initial: Some(std::time::Duration::from_secs(0)),
            keys: vec![russh::keys::PrivateKey::random(
                &mut rand_core::OsRng,
                russh::keys::Algorithm::Ed25519,
            )
            .unwrap()],
            nodelay: true,
            ..Default::default()
        };

        self.run_on_address(Arc::new(config), ("127.0.0.1", 2222))
            .await?;
        Ok(())
    }
}

impl Server for AppServer {
    type Handler = Self;
    fn new_client(&mut self, _: Option<std::net::SocketAddr>) -> Self {
        let s = self.clone();
        self.id += 1;
        s
    }
}

impl Handler for AppServer {
    type Error = russh::Error;

    async fn channel_open_session(
        &mut self,
        _channel: Channel<Msg>,
        _session: &mut Session,
    ) -> Result<bool, Self::Error> {
        let (app_send, term_recv) = unbounded();
        let (psudo_tty, app_recv) = NoTtyEvent::new(term_recv);
        let app = App {
            pty: psudo_tty,
            send: app_send,
            recv: app_recv,
        };

        let mut clients = self.clients.lock().await;
        clients.insert(self.id, app);

        Ok(true)
    }

    async fn auth_publickey(&mut self, _: &str, _: &PublicKey) -> Result<Auth, Self::Error> {
        Ok(Auth::Accept)
    }

    async fn data(
        &mut self,
        _channel: ChannelId,
        data: &[u8],
        _session: &mut Session,
    ) -> Result<(), Self::Error> {
        let mut clients = self.clients.lock().await;
        let app = clients.get_mut(&self.id).unwrap();
        let _ = app.send.send(data.into()).unwrap();
        if data == [3] {
            return Err(russh::Error::Disconnect);
        }

        Ok(())
    }

    /// The client's window size has changed.
    async fn window_change_request(
        &mut self,
        _channel: ChannelId,
        col_width: u32,
        row_height: u32,
        pix_width: u32,
        pix_height: u32,
        _session: &mut Session,
    ) -> Result<(), Self::Error> {
        let mut clients = self.clients.lock().await;
        let app = clients.get_mut(&self.id).unwrap();
        *app.pty.window_size.lock() = WindowSize {
            rows: row_height as u16,
            columns: col_width as u16,
            width: pix_width as u16,
            height: pix_height as u16,
        };

        let mut win_raw = Vec::from(b"\x1B[W");
        let col = (col_width as u16).to_string();
        let row = (row_height as u16).to_string();
        win_raw.extend_from_slice(col.as_bytes());
        win_raw.push(b';');
        win_raw.extend_from_slice(row.as_bytes());
        win_raw.push(b'R');
        let _ = app.send.send(win_raw);

        Ok(())
    }

    /// The client requests a pseudo-terminal with the given
    /// specifications.
    ///
    /// NOTE: Success or failure should be communicated to the client by calling
    /// `session.channel_success(channel)` or `session.channel_failure(channel)` respectively.
    async fn pty_request(
        &mut self,
        channel: ChannelId,
        _: &str,
        col_width: u32,
        row_height: u32,
        pix_width: u32,
        pix_height: u32,
        _: &[(Pty, u32)],
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        let mut clients = self.clients.lock().await;
        let app = clients.get_mut(&self.id).unwrap();

        *app.pty.window_size.lock() = WindowSize {
            rows: row_height as u16,
            columns: col_width as u16,
            width: pix_width as u16,
            height: pix_height as u16,
        };

        session.channel_success(channel)?;

        Ok(())
    }
    async fn shell_request(
        &mut self,
        channel: ChannelId,
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        let mut clients = self.clients.lock().await;
        let app = clients.get_mut(&self.id).unwrap();
        let pty = app.pty.clone();
        let handle = session.handle();
        let (tx, mut rx) = tokio::sync::mpsc::channel::<Vec<u8>>(5);
        let tx2 = tx.clone();
        const HELP: &str = "Blocking read()\r\n- Keyboard, mouse, focus and terminal resize events enabled\r\n- Hit \"c\" to print current cursor position\r\n- Use Esc to quit\r\n";
        let _ = handle.data(channel, HELP.into()).await;
        tokio::task::spawn_blocking(move || {
            let supports_keyboard_enhancement = matches!(
                crossterm::terminal::supports_keyboard_enhancement(&pty),
                Ok(true)
            );
            let mut tx = SenderWriter(tx);

            if supports_keyboard_enhancement {
                let _ = queue!(
                    tx,
                    PushKeyboardEnhancementFlags(
                        KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
                            | KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES
                            | KeyboardEnhancementFlags::REPORT_ALTERNATE_KEYS
                            | KeyboardEnhancementFlags::REPORT_EVENT_TYPES
                    )
                );
            }

            let _ = execute!(
                tx,
                EnableBracketedPaste,
                EnableFocusChange,
                EnableMouseCapture,
            );

            loop {
                // Blocking read
                let event = match read(&pty) {
                    Ok(e) => e,
                    Err(_) => {
                        continue;
                    }
                };

                let data = format!("Event: {event:?}\r\n");
                let _ = tx.0.blocking_send(data.into());

                if event == Event::Key(KeyCode::Char('c').into()) {
                    let data = format!("Cursor position: {:?}\r\n", position(&pty));
                    let _ = tx.0.blocking_send(data.into());
                }

                if let Event::Resize(x, y) = event {
                    let (original_size, new_size) = flush_resize_events(&pty, (x, y));
                    let data = format!("Resize from: {original_size:?}, to: {new_size:?}\r\n");
                    let _ = tx.0.blocking_send(data.into());
                }

                if event == Event::Key(KeyCode::Esc.into()) {
                    break;
                }
            }
            if supports_keyboard_enhancement {
                let _ = queue!(tx, PopKeyboardEnhancementFlags);
            }

            let _ = execute!(
                tx,
                DisableBracketedPaste,
                DisableFocusChange,
                DisableMouseCapture
            );
        });
        let r = app.recv.clone();
        tokio::task::spawn_blocking(move || loop {
            if let Ok(d) = r.recv() {
                let _ = tx2.blocking_send(d);
            } else {
                break;
            }
        });
        tokio::spawn(async move {
            loop {
                if let Some(data) = rx.recv().await {
                    let _ = handle.data(channel, data.into()).await;
                } else {
                    let _ = handle.close(channel).await;
                }
            }
        });
        session.channel_success(channel)?;
        Ok(())
    }
}

// Resize events can occur in batches.
// With a simple loop they can be flushed.
// This function will keep the first and last resize event.
fn flush_resize_events(event: &NoTtyEvent, first_resize: (u16, u16)) -> ((u16, u16), (u16, u16)) {
    let mut last_resize = first_resize;
    while let Ok(true) = poll(event, Duration::from_millis(50)) {
        if let Ok(Event::Resize(x, y)) = read(event) {
            last_resize = (x, y);
        }
    }

    (first_resize, last_resize)
}

impl Drop for AppServer {
    fn drop(&mut self) {
        let id = self.id;
        let clients = self.clients.clone();
        tokio::spawn(async move {
            let mut clients = clients.lock().await;
            clients.remove(&id);
        });
    }
}

struct SenderWriter(tokio::sync::mpsc::Sender<Vec<u8>>);
impl std::io::Write for SenderWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0
            .blocking_send(buf.to_vec())
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        // mpsc is unbuffered; nothing to flush
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let mut server = AppServer::new();
    server.run().await.expect("Failed running server");
}
