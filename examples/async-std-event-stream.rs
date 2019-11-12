use std::pin::Pin;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use std::time::Duration;

use async_std::task;
use futures::{
    task::{Context, Poll},
    Stream, StreamExt,
};

use crossterm::{
    cursor::position,
    event::{poll, read, Event, KeyEvent},
    screen::RawScreen,
    Result,
};

#[derive(Default)]
struct EventReader {
    wake_thread_spawned: Arc<AtomicBool>,
    wake_thread_shutdown: Arc<AtomicBool>,
}

impl Stream for EventReader {
    type Item = Result<Event>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let result = match poll(Some(Duration::from_secs(0))) {
            Ok(true) => Poll::Ready(Some(read())),
            Ok(false) => {
                if !self
                    .wake_thread_spawned
                    .compare_and_swap(false, true, Ordering::SeqCst)
                {
                    let waker = cx.waker().clone();
                    let wake_thread_spawned = self.wake_thread_spawned.clone();
                    let wake_thread_shutdown = self.wake_thread_shutdown.clone();

                    wake_thread_shutdown.store(false, Ordering::SeqCst);

                    thread::spawn(move || {
                        loop {
                            if let Ok(true) = poll(Some(Duration::from_secs(50))) {
                                break;
                            }

                            if wake_thread_shutdown.load(Ordering::SeqCst) {
                                break;
                            }
                        }
                        wake_thread_spawned.store(false, Ordering::SeqCst);
                        waker.wake();
                        eprintln!(" - wake thread exit\r");
                    });

                    eprintln!(" - wake thread spawned\r");
                }
                Poll::Pending
            }
            Err(e) => Poll::Ready(Some(Err(e))),
        };
        result
    }
}

impl Drop for EventReader {
    fn drop(&mut self) {
        self.wake_thread_shutdown.store(true, Ordering::SeqCst);
    }
}

async fn print_events() {
    let mut reader = EventReader::default();

    while let Some(maybe_event) = reader.next().await {
        if let Ok(event) = maybe_event {
            println!("{:?}\r", event);

            if event == Event::Key(KeyEvent::Char('c')) {
                println!("Cursor position: {:?}\r", position());
            }

            if event == Event::Key(KeyEvent::Esc) {
                break;
            }
        }
    }
}

fn main() {
    let _r = RawScreen::into_raw_mode().unwrap();
    task::block_on(print_events());
}
