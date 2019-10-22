use std::collections::vec_deque::VecDeque;
use std::iter::FromIterator;

/// An iterator over occurred input `E`.
#[derive(Debug)]
pub struct EventIterator<E> {
    events: VecDeque<E>,
}

impl<E> EventIterator<E> {
    /// Returns an `EventIterator` from the given `iterator`.
    pub(crate) fn from_iter<T: IntoIterator<Item = E>>(iter: T) -> EventIterator<E> {
        EventIterator {
            events: VecDeque::from_iter(iter),
        }
    }
}

impl<E> Iterator for EventIterator<E> {
    type Item = E;

    /// Returns the next input event.
    fn next(&mut self) -> Option<Self::Item> {
        self.events.pop_front()
    }
}

pub(crate) trait IntoEventIterator<T, E>
where
    T: IntoIterator<Item = E>,
{
    fn into_event_iterator(self) -> EventIterator<E>;
}

impl<E> IntoEventIterator<Vec<E>, E> for Vec<E> {
    fn into_event_iterator(self) -> EventIterator<E> {
        EventIterator::from_iter(self)
    }
}
