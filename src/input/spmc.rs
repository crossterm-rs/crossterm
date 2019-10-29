use std::sync::{Arc, LockResult, RwLock, RwLockWriteGuard};

use shrev::{self, ReaderId};

use crate::input::events::InternalEvent;

/// Single producer multiple consumers channel (SPMC) for event sharing.
pub(crate) struct EventChannel {
    event_channel: Arc<RwLock<shrev::EventChannel<InternalEvent>>>,
}

impl EventChannel {
    /// Constructs a new spmc `InputEventChannel`.
    pub(crate) fn channel(event_channel: shrev::EventChannel<InternalEvent>) -> EventChannel {
        EventChannel {
            event_channel: Arc::new(RwLock::new(event_channel)),
        }
    }

    /// Constructs a new consumer for consuming input events.
    pub(crate) fn new_consumer(&self) -> EventConsumer {
        EventConsumer::new(self.event_channel.clone())
    }

    /// Tries to acquire the producer that can sent input events to the consumers.
    pub(crate) fn producer(&mut self) -> ProducerLock<'_> {
        ProducerLock::from_lock_result(self.event_channel.write())
    }
}

/// The consumer that consumers input events from the producer.
pub(crate) struct EventConsumer {
    // TODO: I could't find a way to store the Reader Lock here instead of the whole channel.
    event_channel: Arc<RwLock<shrev::EventChannel<InternalEvent>>>,
    read_id: ReaderId<InternalEvent>,
}

impl EventConsumer {
    pub(crate) fn new(
        event_channel: Arc<RwLock<shrev::EventChannel<InternalEvent>>>,
    ) -> EventConsumer {
        EventConsumer {
            read_id: event_channel.write().unwrap().register_reader(),
            event_channel: event_channel.clone(),
        }
    }

    /// Returns all available input events for this consumer.
    pub(crate) fn read_all(&mut self) -> Vec<InternalEvent> {
        let lock = self
            .event_channel
            .read()
            .expect("Can not acquire read lock");

        lock.read(&mut self.read_id)
            .into_iter()
            .map(|x| x.clone())
            .collect::<Vec<InternalEvent>>()
    }
}

/// An acquired write lock to the event channel producer.
pub struct ProducerLock<'a> {
    lock_result: LockResult<RwLockWriteGuard<'a, shrev::EventChannel<InternalEvent>>>,
}

impl<'a> ProducerLock<'a> {
    pub(crate) fn from_lock_result(
        lock_result: LockResult<RwLockWriteGuard<'a, shrev::EventChannel<InternalEvent>>>,
    ) -> ProducerLock<'a> {
        ProducerLock { lock_result }
    }

    pub(crate) fn produce_event(&mut self, input_event: InternalEvent) {
        self.lock_result
            .as_mut()
            .expect("can not acquire write lock")
            .single_write(input_event);
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, RwLock};

    use shrev::EventChannel;

    use crate::input::spmc::EventConsumer;
    use crate::{Event, KeyEvent, MouseEvent};

    #[test]
    pub fn test_read_all_events() {
        let (channel, mut consumer) = event_consumer();

        let input_events = vec![
            Event::Unsupported(vec![]),
            Event::Unknown,
            Event::Mouse(MouseEvent::Unknown),
            Event::Keyboard(KeyEvent::Up),
        ];

        for event in input_events.iter() {
            channel.write().unwrap().single_write(event.clone());
        }

        assert_eq!(consumer.read_all(), input_events);
    }

    fn event_consumer() -> (Arc<RwLock<EventChannel<Event>>>, EventConsumer) {
        let mut channel = Arc::new(RwLock::new(EventChannel::new()));
        (channel.clone(), EventConsumer::new(channel))
    }
}
