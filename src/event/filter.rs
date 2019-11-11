use crate::event::InternalEvent;

/// Interface for filtering an `InternalEvent`.
pub(crate) trait Filter {
    /// Returns whether the given event fulfills the filter.
    fn filter(&self, event: &InternalEvent) -> bool;
}

#[cfg(unix)]
pub(crate) struct CursorPositionFilter;

#[cfg(unix)]
impl Filter for CursorPositionFilter {
    fn filter(&self, event: &InternalEvent) -> bool {
        if let &InternalEvent::CursorPosition(_, _) = event {
            return true;
        }

        false
    }
}

pub(crate) struct EventFilter;

impl Filter for EventFilter {
    fn filter(&self, event: &InternalEvent) -> bool {
        match event {
            &InternalEvent::Event(_) => true,
            #[cfg(unix)]
            _ => false,
        }
    }
}

pub(crate) struct InternalEventFilter;

impl Filter for InternalEventFilter {
    fn filter(&self, _: &InternalEvent) -> bool {
        return true;
    }
}
