use crate::event::InternalEvent;

/// Interface for filtering an `InternalEvent`.
pub(crate) trait Filter: Send + Sync + 'static {
    /// Returns whether the given event fulfills the filter.
    fn eval(&self, event: &InternalEvent) -> bool;
}

#[cfg(unix)]
pub(crate) struct CursorPositionFilter;

#[cfg(unix)]
impl Filter for CursorPositionFilter {
    fn eval(&self, event: &InternalEvent) -> bool {
        if let &InternalEvent::CursorPosition(_, _) = event {
            true
        } else {
            false
        }
    }
}

pub(crate) struct EventFilter;

impl Filter for EventFilter {
    fn eval(&self, event: &InternalEvent) -> bool {
        if let &InternalEvent::Event(_) = event {
            true
        } else {
            false
        }
    }
}

pub(crate) struct InternalEventFilter;

impl Filter for InternalEventFilter {
    fn eval(&self, _: &InternalEvent) -> bool {
        true
    }
}

#[cfg(test)]
#[cfg(unix)]
mod tests {
    use super::{CursorPositionFilter, EventFilter, Filter, InternalEventFilter};
    use crate::event::{Event, InternalEvent, KeyCode, KeyEvent};

    #[test]
    fn test_cursor_position_filter_filters_cursor_position() {
        assert_eq!(
            CursorPositionFilter.eval(&InternalEvent::Event(Event::Resize(10, 10))),
            false
        );
        assert_eq!(CursorPositionFilter.eval(&InternalEvent::CursorPosition(0, 0), true));
    }

    #[test]
    fn test_event_filter_filters_events() {
        assert_eq!(
            EventFilter.eval(&InternalEvent::Event(Event::Resize(10, 10))),
            true
        );
        assert_eq!(EventFilter.eval(&InternalEvent::CursorPosition(0, 0), false));
    }

    #[test]
    fn test_event_filter_filters_internal_events() {
        assert_eq!(
            InternalEventFilter.eval(&InternalEvent::Event(Event::Resize(10, 10))),
            true
        );
        assert_eq!(InternalEventFilter.eval(&InternalEvent::CursorPosition(0, 0), true));
    }
}
