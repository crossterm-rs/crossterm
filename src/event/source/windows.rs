use std::time::Duration;

use crossterm_winapi::{Console, Handle, InputRecord};

use crate::event::{
    Event,
    sys::windows::{parse::MouseButtonsPressed, poll::WinApiPoll},
};

#[cfg(feature = "event-stream")]
use crate::event::sys::Waker;
use crate::event::{
    internal::InternalEvent,
    source::EventSource,
    sys::windows::parse::{handle_key_event, handle_mouse_event},
    timeout::PollTimeout,
};

pub(crate) struct WindowsEventSource {
    console: Console,
    poll: WinApiPoll,
    surrogate_buffer: Option<u16>,
    mouse_buttons_pressed: MouseButtonsPressed,
}

impl WindowsEventSource {
    pub(crate) fn new() -> std::io::Result<WindowsEventSource> {
        let console = Console::from(Handle::current_in_handle()?);
        Ok(WindowsEventSource {
            console,

            #[cfg(not(feature = "event-stream"))]
            poll: WinApiPoll::new(),
            #[cfg(feature = "event-stream")]
            poll: WinApiPoll::new()?,

            surrogate_buffer: None,
            mouse_buttons_pressed: MouseButtonsPressed::default(),
        })
    }
}

impl EventSource for WindowsEventSource {
    fn try_read(&mut self, timeout: Option<Duration>) -> std::io::Result<Option<InternalEvent>> {
        let poll_timeout = PollTimeout::new(timeout);

        loop {
            if let Some(event_ready) = self.poll.poll(poll_timeout.leftover())? {
                let number = self.console.number_of_console_input_events()?;
                if event_ready && number != 0 {
                    let event = match self.console.read_single_input_event()? {
                        InputRecord::KeyEvent(record) => {
                            handle_key_event(record, &mut self.surrogate_buffer)
                        }
                        InputRecord::MouseEvent(record) => {
                            let mouse_event =
                                handle_mouse_event(record, &self.mouse_buttons_pressed);
                            self.mouse_buttons_pressed = MouseButtonsPressed {
                                left: record.button_state.left_button(),
                                right: record.button_state.right_button(),
                                middle: record.button_state.middle_button(),
                            };

                            mouse_event
                        }
                        InputRecord::WindowBufferSizeEvent(record) => {
                            // windows starts counting at 0, unix at 1, add one to replicate unix
                            // behaviour.
                            Some(Event::Resize(
                                (record.size.x as i32 + 1) as u16,
                                (record.size.y as i32 + 1) as u16,
                            ))
                        }
                        InputRecord::FocusEvent(record) => {
                            let event = if record.set_focus {
                                Event::FocusGained
                            } else {
                                Event::FocusLost
                            };
                            Some(event)
                        }
                        _ => None,
                    };

                    if let Some(event) = event {
                        return Ok(Some(InternalEvent::Event(event)));
                    }

                    // A record that maps to no event, such as the modifier key records the
                    // console adds around a shifted character, must not end a poll with a zero
                    // timeout while more records are queued.
                    if self.console.number_of_console_input_events()? != 0 {
                        continue;
                    }
                }
            }

            if poll_timeout.elapsed() {
                return Ok(None);
            }
        }
    }

    #[cfg(feature = "event-stream")]
    fn waker(&self) -> Waker {
        self.poll.waker()
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crossterm_winapi::Handle;
    use serial_test::serial;
    use winapi::um::{
        wincon::{
            FlushConsoleInputBuffer, INPUT_RECORD, KEY_EVENT, SHIFT_PRESSED, WriteConsoleInputW,
        },
        winuser::VK_SHIFT,
    };

    use super::WindowsEventSource;
    use crate::event::{
        Event, KeyCode, KeyEvent, KeyModifiers, internal::InternalEvent, source::EventSource,
    };

    fn key_down_record(virtual_key_code: i32, u_char: u16) -> INPUT_RECORD {
        // SAFETY: INPUT_RECORD is plain data, so all zero bytes is a valid value.
        let mut record: INPUT_RECORD = unsafe { std::mem::zeroed() };
        record.EventType = KEY_EVENT;
        // SAFETY: EventType marks the union as holding a KEY_EVENT_RECORD.
        let key_event = unsafe { record.Event.KeyEvent_mut() };
        key_event.bKeyDown = 1;
        key_event.wRepeatCount = 1;
        key_event.wVirtualKeyCode = virtual_key_code as u16;
        key_event.dwControlKeyState = SHIFT_PRESSED;
        // SAFETY: The wide character variant is the one crossterm reads.
        unsafe { *key_event.uChar.UnicodeChar_mut() = u_char };
        record
    }

    #[test]
    #[serial]
    fn zero_timeout_read_skips_records_without_events() {
        let records = [
            key_down_record(VK_SHIFT, 0),
            key_down_record('A' as i32, 'A' as u16),
        ];
        let handle = Handle::current_in_handle().unwrap();
        let mut written = 0;
        // SAFETY: The handle is a console input handle and `records` outlives the call.
        unsafe {
            assert_ne!(FlushConsoleInputBuffer(*handle), 0);
            assert_ne!(
                WriteConsoleInputW(
                    *handle,
                    records.as_ptr(),
                    records.len() as u32,
                    &mut written
                ),
                0
            );
        }
        assert_eq!(written, records.len() as u32);

        let mut source = WindowsEventSource::new().unwrap();
        assert_eq!(
            source.try_read(Some(Duration::ZERO)).unwrap(),
            Some(InternalEvent::Event(Event::Key(KeyEvent::new(
                KeyCode::Char('A'),
                KeyModifiers::SHIFT
            ))))
        );
    }
}
