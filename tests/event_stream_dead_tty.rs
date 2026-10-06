#![cfg(all(unix, feature = "event-stream"))]

use std::io;
use std::thread;
use std::time::Duration;

use crossterm::event::EventStream;
use futures::StreamExt;
use rustix::stdio::dup2_stdin;
use rustix_openpty::openpty;

#[tokio::test(flavor = "current_thread")]
async fn event_stream_reports_dead_stdin_and_terminates() -> io::Result<()> {
    let pty = openpty(/*termios*/ None, /*winsize*/ None)?;
    dup2_stdin(&pty.user)?;
    drop(pty.user);

    let mut stream = EventStream::new();
    let closer = thread::spawn(move || {
        thread::sleep(Duration::from_millis(100));
        drop(pty.controller);
    });

    let result = tokio::time::timeout(Duration::from_secs(2), stream.next())
        .await
        .expect("EventStream did not report the closed stdin");
    let error = result
        .expect("EventStream terminated without reporting the closed stdin")
        .expect_err("EventStream unexpectedly yielded an event");
    assert_eq!(error.kind(), io::ErrorKind::UnexpectedEof);

    let result = tokio::time::timeout(Duration::from_millis(100), stream.next())
        .await
        .expect("EventStream did not terminate after reporting the error");
    assert!(result.is_none());

    closer.join().expect("stdin closer panicked");
    Ok(())
}
