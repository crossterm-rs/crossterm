use mio::net::{TcpListener, TcpStream};
use mio::*;
use signal_hook::iterator::Signals;
use std::thread;
use std::time::Duration;

// Setup some tokens to allow us to identify which event is
// for which socket.
const TOKEN: Token = Token(0);

fn main() {
    let signals = Signals::new(&[signal_hook::SIGWINCH]).unwrap();

    // Create storage for events
    let mut events = Events::with_capacity(1024);

    // Create a poll instance
    let poll = Poll::new().unwrap();

    // Start listening for incoming connections
    poll.register(&signals, TOKEN, Ready::readable(), PollOpt::level())
        .unwrap();

    loop {
        read_signal(&poll, &mut events);
    }
}

/// Having this as an function is done to simulate the way we read the other events.
fn read_signal(poll: &Poll, events: &mut Events) {
    poll.poll(events, None).unwrap();

    println!("events: length {:?}", events.len());

    for event in events.iter() {
        match event.token() {
            TOKEN => {
                println!(
                    "resize event occurred new size: {:?}",
                    crossterm::terminal::size()
                );
                break;
            }
            _ => unreachable!(),
        }
    }

    thread::sleep(Duration::from_millis(30));
}
