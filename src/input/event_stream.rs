use crate::input::event_iterator::{EventIterator, IntoEventIterator};
use crate::input::events::InternalEvent;
use crate::input::spmc::EventConsumer;

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
pub(crate) struct EventStream {
    channel_reader: EventConsumer,
}

impl<'a> EventStream {
    /// Constructs a new `EventStream` by passing in the consumer responsible for receiving events.
    pub(crate) fn new(channel_reader: EventConsumer) -> EventStream {
        EventStream { channel_reader }
    }

    /// Returns an iterator over the pressed `InputEvent`s.
    pub(crate) fn events(&mut self) -> EventIterator<InternalEvent> {
        self.channel_reader.read_all().into_event_iterator()
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
