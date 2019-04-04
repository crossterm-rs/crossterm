extern crate crossterm;

use crossterm::Screen;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex, mpsc::{self, Receiver, Sender}};
use std::thread::{self, JoinHandle};

/// This is an que that could be shared between threads safely.
#[derive(Clone)]
struct WorkQueue<T: Send + Clone> {
    inner: Arc<Mutex<VecDeque<T>>>,
}

impl<T: Send + Clone> WorkQueue<T> {
    fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    // get an item from the que if exists
    fn get_work(&self) -> Option<T> {
        let maybe_queue = self.inner.lock();

        if let Ok(mut queue) = maybe_queue {
            queue.pop_front()
        } else {
            panic!("WorkQueue::get_work() tried to lock a poisoned mutex");
        }
    }

    // add an item to the que
    fn add_work(&self, work: T) -> usize {
        if let Ok(mut queue) = self.inner.lock() {
            queue.push_back(work);
            queue.len()
        } else {
            panic!("WorkQueue::add_work() tried to lock a poisoned mutex");
        }
    }
}

#[derive(Clone)]
struct SyncFlagTx {
    inner: Arc<Mutex<bool>>,
}

impl SyncFlagTx {
    pub fn set(&mut self, state: bool) -> Result<(), ()> {
        if let Ok(mut v) = self.inner.lock() {
            *v = state;
            Ok(())
        } else {
            Err(())
        }
    }
}

#[derive(Clone)]
struct SyncFlagRx {
    inner: Arc<Mutex<bool>>,
}

impl SyncFlagRx {
    pub fn get(&self) -> Result<bool, ()> {
        if let Ok(v) = self.inner.lock() {
            Ok(*v)
        } else {
            Err(())
        }
    }
}

fn new_sync_flag(initial_state: bool) -> (SyncFlagTx, SyncFlagRx) {
    let state = Arc::new(Mutex::new(initial_state));
    let tx = SyncFlagTx {
        inner: state.clone(),
    };
    let rx = SyncFlagRx {
        inner: state.clone(),
    };

    return (tx, rx);
}

fn main() {
    let (_results_tx, _results_rx): (Sender<String>, Receiver<String>) = mpsc::channel();
    let (more_jobs_tx, more_jobs_rx) = new_sync_flag(true);

    // queue with all log entry's.
    let queue = WorkQueue::new();

    //  queue x logs with different threads.
    let _thread_handles = log_with_different_threads(more_jobs_tx.clone(), queue.clone());

    // a thread that will log all logs in the queue.
    handle_incoming_logs(more_jobs_rx.clone(), queue.clone());
}

fn handle_incoming_logs(more_jobs_rx: SyncFlagRx, queue: WorkQueue<String>) {
    thread::spawn(move || {
        let screen: Screen = Screen::default();

        // Loop while there's expected to be work, looking for work.
        while more_jobs_rx.get().unwrap() {
            // If work is available, do that work.
            if let Some(work) = queue.get_work() {
                let mut log = work;
                log.push('\n');

                // write the log
                screen.stdout.write_string(log);
            }
            std::thread::yield_now();
        }
    })
    .join();
}

// start different threads that log contiguously.
fn log_with_different_threads(
    more_jobs_tx: SyncFlagTx,
    queue: WorkQueue<String>,
) -> Vec<JoinHandle<()>> {
    // one vector that will have the thread handles in it.
    let mut threads = Vec::new();

    for thread_num in 1..5 {
        let mut more_jobs = more_jobs_tx.clone();
        let thread_queue = queue.clone();

        // create new thread
        let thread = thread::spawn(move || {
            // log 400 messages
            for log_entry_count in 1..400 {
                thread_queue.add_work(format!(
                    "Log {} from thread {} ",
                    log_entry_count, thread_num
                ));
                more_jobs.set(true);
            }
        });

        threads.push(thread);
    }

    println!("All logging threads started");
    return threads;
}
