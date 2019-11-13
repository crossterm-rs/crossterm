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
