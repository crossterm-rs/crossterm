use crate::event::InternalEvent;

pub(crate) trait EventMask {
    fn filter(&self, event: &InternalEvent) -> bool;
}

#[cfg(unix)]
pub(crate) struct CursorEventMask;

#[cfg(unix)]
impl EventMask for CursorEventMask {
    fn filter(&self, event: &InternalEvent) -> bool {
        if let &InternalEvent::CursorPosition(_, _) = event {
            return true;
        }

        false
    }
}

pub(crate) struct EventOnlyMask;

impl EventMask for EventOnlyMask {
    fn filter(&self, event: &InternalEvent) -> bool {
        match event {
            &InternalEvent::Event(_) => true,
            #[cfg(unix)]
            _ => false,
        }
    }
}

pub(crate) struct InternalEventMask;

impl EventMask for InternalEventMask {
    fn filter(&self, _: &InternalEvent) -> bool {
        return true;
    }
}
