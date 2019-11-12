use std::pin::Pin;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use std::time::Duration;

use async_std::task;
use futures::{
    future::FutureExt,
    select,
    task::{Context, Poll},
    Stream, StreamExt,
};
use futures_timer::Delay;

use crossterm::{
    cursor::position,
    event::{poll, read, Event, KeyEvent},
    screen::RawScreen,
    Result,
};

#[derive(Default)]
struct EventReader {
    wake_thread_spawned: Arc<AtomicBool>,
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
                    eprintln!(" - wake thread spawned\r");
                    let waker = cx.waker().clone();
                    let wake_thread_spawned = self.wake_thread_spawned.clone();
                    thread::spawn(move || {
                        loop {
                            if let Ok(true) = poll(None) {
                                break;
                            }
                        }
                        wake_thread_spawned.store(false, Ordering::SeqCst);
                        waker.wake();
                    });
                }
                Poll::Pending
            }
            Err(e) => Poll::Ready(Some(Err(e))),
        };
        result
    }
}

async fn print_ticking_events() {
    let mut reader = EventReader::default();

    loop {
        let mut delay = Delay::new(Duration::from_millis(100)).fuse();
        let mut event = reader.next().fuse();

        select! {
            _ = delay => { println!("Tick tock\r"); },
            maybe_event = event => {
                if let Some(Ok(event)) = maybe_event {
                    println!("{:?}\r", event);

                    if event == Event::Key(KeyEvent::Char('c')) {
                        println!("Cursor position: {:?}\r", position());
                    }

                    if event == Event::Key(KeyEvent::Esc) {
                        break;
                    }
                }
            }
        };
    }
}

fn main() {
    let _r = RawScreen::into_raw_mode().unwrap();
    task::block_on(print_ticking_events());
}
