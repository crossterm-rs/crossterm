use crate::input::event_iterator::IntoEventIterator;
use crate::input::spmc::EventConsumer;
use crate::EventIterator;
use crate::{InputEvent, KeyEvent, MouseEvent};

/// An event stream that can be used to read occurred events.
///
/// Use [`event_stream`](..link) to obtain an `EventStream`.
///
/// ```no_run
/// use crossterm::event_stream;
///
/// fn main() {
///     let mut event_stream = event_stream();
///
///     let occurred_key_events = event_stream.key_events();
///     let occurred_mouse_events = event_stream.mouse();
///     let occurred_key_events = event_stream.events();
/// ```
pub struct EventStream {
    channel_reader: EventConsumer,
    event_cache: Vec<InputEvent>,
}

impl<'a> EventStream {
    /// Constructs a new `EventStream` by passing in the consumer responsible for receiving events.
    pub(crate) fn new(channel_reader: EventConsumer) -> EventStream {
        EventStream {
            channel_reader,
            event_cache: Vec::new(),
        }
    }

    /// Returns an iterator over the pressed `KeyEvent`s.
    pub fn key_events(&mut self) -> EventIterator<KeyEvent> {
        self.update_local_cache();

        self.drain_events(|e| match e {
            InputEvent::Keyboard(event) => Some(event.to_owned()),
            _ => None,
        })
        .into_event_iterator()
    }

    /// Returns an iterator over the pressed `MouseEvent`s.
    pub fn mouse_events(&mut self) -> EventIterator<MouseEvent> {
        self.update_local_cache();
        self.drain_events(|e| match e {
            InputEvent::Mouse(event) => Some(event.to_owned()),
            _ => None,
        })
        .into_event_iterator()
    }

    /// Returns an iterator over the pressed `InputEvent`s.
    pub fn events(&mut self) -> EventIterator<InputEvent> {
        self.update_local_cache();
        self.drain_events(|e| Some(e.to_owned()))
            .into_event_iterator()
    }

    /// Drains input events from the local cache based on the given criteria.
    fn drain_events<T>(&mut self, mut filter: impl FnMut(&InputEvent) -> Option<T>) -> Vec<T> {
        // TODO: nightly: `Vec::drain_filter`
        let mut drained = Vec::with_capacity(self.event_cache.len());
        let mut i = 0;
        while i != self.event_cache.len() {
            if let Some(event) = filter(&self.event_cache[i]) {
                self.event_cache.remove(i);
                drained.push(event);
            } else {
                i += 1;
            }
        }
        drained
    }

    /// Receives input events from receiver and write them to the local cache.
    fn update_local_cache(&mut self) {
        self.event_cache.extend(self.channel_reader.read_all());
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, RwLock};

    use shrev::EventChannel;

    use crate::input::spmc::EventConsumer;
    use crate::EventStream;
    use crate::{InputEvent, KeyEvent, MouseEvent};

    #[test]
    pub fn test_consumer_consumes_channel_key_events() {
        let (channel, consumer) = event_consumer();

        let mut input_stream = EventStream::new(consumer);

        channel
            .write()
            .unwrap()
            .single_write(InputEvent::Keyboard(KeyEvent::Tab));

        assert_eq!(input_stream.key_events().next(), Some(KeyEvent::Tab));
    }

    #[test]
    pub fn test_consumer_consumes_channel_mouse_events() {
        let (channel, consumer) = event_consumer();

        let mut input_stream = EventStream::new(consumer);

        // produce event
        channel
            .write()
            .unwrap()
            .single_write(InputEvent::Mouse(MouseEvent::Unknown));

        // consume events
        assert_eq!(
            input_stream.mouse_events().next(),
            Some(MouseEvent::Unknown)
        );
        assert_eq!(input_stream.key_events().next(), None);
        assert_eq!(input_stream.events().next(), None);
    }

    #[test]
    pub fn test_consumer_consumes_channel_input_events() {
        let (channel, consumer) = event_consumer();

        let mut input_stream = EventStream::new(consumer);

        // produce events
        channel.write().unwrap().single_write(InputEvent::Unknown);
        channel
            .write()
            .unwrap()
            .single_write(InputEvent::Unsupported(vec![]));

        // consume events
        let mut event_iterator = input_stream.events();

        assert_eq!(event_iterator.next(), Some(InputEvent::Unknown));
        assert_eq!(event_iterator.next(), Some(InputEvent::Unsupported(vec![])));
    }

    fn event_consumer() -> (Arc<RwLock<EventChannel<InputEvent>>>, EventConsumer) {
        let mut channel = Arc::new(RwLock::new(EventChannel::new()));
        (channel.clone(), EventConsumer::new(channel))
    }
}
