use std::{
    io,
    pin::Pin,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
        mpsc::{self, SyncSender},
    },
    task::{Context, Poll},
    thread,
    time::Duration,
};

use futures_core::stream::Stream;
use parking_lot::Mutex;

use crate::event::{
    Event,
    filter::EventFilter,
    internal::{self, InternalEvent},
    sys::Waker,
};

/// A stream of `Result<Event>`.
///
/// **This type is not available by default. You have to use the `event-stream` feature flag
/// to make it available.**
///
/// It implements the [Stream](futures_core::stream::Stream)
/// trait and allows you to receive [`Event`]s with [`smol`](https://crates.io/crates/smol)
/// or [`tokio`](https://crates.io/crates/tokio) crates.
///
/// Check the [examples](https://github.com/crossterm-rs/crossterm/tree/master/examples) folder to see how to use
/// it (`event-stream-*`).
#[derive(Debug)]
pub struct EventStream {
    poll_internal_waker: Waker,
    stream_waker: Arc<SharedWaker>,
    stream_wake_task_executed: Arc<AtomicBool>,
    stream_wake_task_should_shutdown: Arc<AtomicBool>,
    task_sender: SyncSender<Task>,
}

impl Default for EventStream {
    fn default() -> Self {
        let (task_sender, receiver) = mpsc::sync_channel::<Task>(1);

        thread::spawn(move || {
            while let Ok(task) = receiver.recv() {
                loop {
                    if let Ok(true) = internal::poll(None, &EventFilter) {
                        break;
                    }

                    if task.stream_wake_task_should_shutdown.load(Ordering::SeqCst) {
                        break;
                    }
                }
                task.stream_wake_task_executed
                    .store(false, Ordering::SeqCst);
                task.stream_waker.wake();
            }
        });

        EventStream {
            poll_internal_waker: internal::lock_event_reader().waker(),
            stream_waker: Arc::new(SharedWaker::default()),
            stream_wake_task_executed: Arc::new(AtomicBool::new(false)),
            stream_wake_task_should_shutdown: Arc::new(AtomicBool::new(false)),
            task_sender,
        }
    }
}

impl EventStream {
    /// Constructs a new instance of `EventStream`.
    pub fn new() -> EventStream {
        EventStream::default()
    }
}

/// The waker of the task that most recently polled the stream.
///
/// The thread waiting for input can outlive the poll that started it, so it wakes whichever task
/// polled last instead of the one that happened to start the wait.
#[derive(Debug, Default)]
struct SharedWaker(Mutex<Option<std::task::Waker>>);

impl SharedWaker {
    fn register(&self, waker: &std::task::Waker) {
        let mut slot = self.0.lock();
        match &mut *slot {
            Some(current) if current.will_wake(waker) => {}
            slot => *slot = Some(waker.clone()),
        }
    }

    fn wake(&self) {
        // Wake outside the lock so a waker that polls the stream again cannot deadlock.
        let waker = self.0.lock().clone();
        if let Some(waker) = waker {
            waker.wake();
        }
    }
}

struct Task {
    stream_waker: Arc<SharedWaker>,
    stream_wake_task_executed: Arc<AtomicBool>,
    stream_wake_task_should_shutdown: Arc<AtomicBool>,
}

// Note to future me
//
// We need two wakers in order to implement EventStream correctly.
//
// 1. futures::Stream waker
//
// Stream::poll_next can return Poll::Pending which means that there's no
// event available. We are going to spawn a thread with the
// poll_internal(None, &EventFilter) call. This call blocks until an
// event is available and then we have to wake up the executor with notification
// that the task can be resumed.
//
// 2. poll_internal waker
//
// There's no event available, Poll::Pending was returned, stream waker thread
// is up and sitting in the poll_internal. User wants to drop the EventStream.
// We have to wake up the poll_internal (force it to return Ok(false)) and quit
// the thread before we drop.
impl Stream for EventStream {
    type Item = io::Result<Event>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match internal::poll(Some(Duration::from_secs(0)), &EventFilter) {
            Ok(true) => match internal::read(&EventFilter) {
                Ok(InternalEvent::Event(event)) => Poll::Ready(Some(Ok(event))),
                Err(e) => Poll::Ready(Some(Err(e))),
                #[cfg(unix)]
                _ => unreachable!(),
            },
            Ok(false) => {
                // Register before looking at the flag: a wait that is about to finish then still
                // wakes this task.
                self.stream_waker.register(cx.waker());
                if !self
                    .stream_wake_task_executed
                    .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
                    // https://github.com/rust-lang/rust/issues/80486#issuecomment-752244166
                    .unwrap_or_else(|x| x)
                {
                    let stream_waker = self.stream_waker.clone();
                    let stream_wake_task_executed = self.stream_wake_task_executed.clone();
                    let stream_wake_task_should_shutdown =
                        self.stream_wake_task_should_shutdown.clone();

                    stream_wake_task_should_shutdown.store(false, Ordering::SeqCst);

                    let _ = self.task_sender.send(Task {
                        stream_waker,
                        stream_wake_task_executed,
                        stream_wake_task_should_shutdown,
                    });
                }
                Poll::Pending
            }
            Err(e) => Poll::Ready(Some(Err(e))),
        }
    }
}

impl Drop for EventStream {
    fn drop(&mut self) {
        self.stream_wake_task_should_shutdown
            .store(true, Ordering::SeqCst);
        let _ = self.poll_internal_waker.wake();
    }
}

#[cfg(test)]
mod shared_waker_tests {
    use std::{
        sync::atomic::{AtomicUsize, Ordering},
        task::Wake,
    };

    use super::*;

    #[derive(Default)]
    struct Counter(AtomicUsize);

    impl Wake for Counter {
        fn wake(self: Arc<Self>) {
            self.0.fetch_add(1, Ordering::SeqCst);
        }
    }

    fn count(counter: &Arc<Counter>) -> usize {
        counter.0.load(Ordering::SeqCst)
    }

    #[test]
    fn wake_without_a_registered_waker_does_nothing() {
        SharedWaker::default().wake();
    }

    #[test]
    fn wakes_only_the_latest_registered_waker() {
        let shared = SharedWaker::default();
        let (first, second) = (Arc::new(Counter::default()), Arc::new(Counter::default()));

        shared.register(&std::task::Waker::from(first.clone()));
        shared.register(&std::task::Waker::from(second.clone()));
        shared.wake();

        assert_eq!((count(&first), count(&second)), (0, 1));
    }

    #[test]
    fn keeps_waking_the_waker_until_another_one_is_registered() {
        let shared = SharedWaker::default();
        let counter = Arc::new(Counter::default());
        let waker = std::task::Waker::from(counter.clone());

        shared.register(&waker);
        shared.register(&waker);
        shared.wake();
        shared.wake();

        assert_eq!(count(&counter), 2);
    }
}

#[cfg(all(test, unix, feature = "event-stream"))]
mod pty_tests {
    use std::{
        ffi::OsStr,
        fs::File,
        io::{BufRead, BufReader, Write},
        os::unix::{ffi::OsStrExt, process::CommandExt},
        process::{Command, Stdio},
        sync::mpsc::{self, RecvTimeoutError},
        task::Wake,
    };

    use rustix::{
        process::setsid,
        pty::{OpenptFlags, grantpt, openpt, ptsname, unlockpt},
    };

    use super::*;

    struct NamedWaker {
        name: &'static str,
        sender: Mutex<mpsc::Sender<&'static str>>,
    }

    impl Wake for NamedWaker {
        fn wake(self: Arc<Self>) {
            let _ = self.sender.lock().send(self.name);
        }
    }

    // Tasks A and B poll the same stream, so the stream must wake B. The parent gives the child a
    // private terminal on stdin and sends the key press only after both polls have returned.
    #[test]
    fn wakes_the_task_that_polled_last() {
        let pty = openpt(OpenptFlags::RDWR).unwrap();
        grantpt(&pty).unwrap();
        unlockpt(&pty).unwrap();
        let name = ptsname(&pty, Vec::new()).unwrap();
        let terminal = File::options()
            .read(true)
            .write(true)
            .open(OsStr::from_bytes(name.to_bytes()))
            .unwrap();

        let mut child = Command::new(std::env::current_exe().unwrap());
        child
            .args([
                "--ignored",
                "--exact",
                "--nocapture",
                "event::stream::pty_tests::check_latest_waker_is_woken",
            ])
            .stdin(Stdio::from(terminal))
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit());
        // The child needs the terminal on stdin as its only terminal, so drop /dev/tty.
        // SAFETY: The pre_exec hook only calls async-signal-safe setsid.
        unsafe {
            child.pre_exec(|| setsid().map(|_| ()).map_err(std::io::Error::from));
        }
        let mut child = child.spawn().unwrap();

        let mut output = BufReader::new(child.stdout.take().unwrap());
        let mut keyboard = File::from(pty);
        let mut transcript = String::new();
        let mut line = String::new();
        while output.read_line(&mut line).unwrap() > 0 {
            transcript.push_str(&line);
            // The last character of the line is the key to type.
            if line.starts_with("polled ") {
                keyboard
                    .write_all(&line.trim_end().as_bytes()[line.trim_end().len() - 1..])
                    .unwrap();
            }
            line.clear();
        }

        let status = child.wait().unwrap();
        assert!(status.success(), "{transcript}");
        for expected in ["woken B", "woken D", "stream checked"] {
            assert!(transcript.contains(expected), "{transcript}");
        }
    }

    #[test]
    #[ignore = "launched by wakes_the_task_that_polled_last with a private terminal"]
    fn check_latest_waker_is_woken() {
        crate::terminal::enable_raw_mode().unwrap();
        let mut stream = EventStream::new();
        let (sender, woken) = mpsc::channel();
        let waker = |name| {
            std::task::Waker::from(Arc::new(NamedWaker {
                name,
                sender: Mutex::new(sender.clone()),
            }))
        };
        let poll = |stream: &mut EventStream, waker: &std::task::Waker| {
            Pin::new(stream).poll_next(&mut Context::from_waker(waker))
        };

        // Run two rounds so the second one covers a stream whose first wait has finished.
        for (first, second, key) in [("A", "B", 'x'), ("C", "D", 'y')] {
            assert!(poll(&mut stream, &waker(first)).is_pending());
            assert!(poll(&mut stream, &waker(second)).is_pending());
            println!("polled {first} then {second}, key {key}");

            let name = match woken.recv_timeout(Duration::from_secs(5)) {
                Ok(name) => name,
                Err(RecvTimeoutError::Timeout) => panic!("no task was woken"),
                Err(error) => panic!("{error}"),
            };
            println!("woken {name}");
            assert_eq!(name, second);
            let event = poll(&mut stream, &waker(second));
            assert!(
                matches!(&event, Poll::Ready(Some(Ok(Event::Key(k)))) if k.code == crate::event::KeyCode::Char(key)),
                "{event:?}"
            );
        }
        crate::terminal::disable_raw_mode().unwrap();
        println!("stream checked");
    }
}
