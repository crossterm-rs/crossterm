use anes;

use super::{Event, InternalEvent, KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent};

impl From<anes::KeyModifiers> for KeyModifiers {
    fn from(akm: anes::KeyModifiers) -> Self {
        let mut modifiers = KeyModifiers::empty();

        if akm.contains(anes::KeyModifiers::SHIFT) {
            modifiers |= KeyModifiers::SHIFT;
        }

        if akm.contains(anes::KeyModifiers::ALT) {
            modifiers |= KeyModifiers::ALT;
        }

        if akm.contains(anes::KeyModifiers::CONTROL) {
            modifiers |= KeyModifiers::CONTROL;
        }

        // TODO - crossterm lacks ::META

        modifiers
    }
}

impl From<anes::KeyCode> for KeyCode {
    fn from(akc: anes::KeyCode) -> Self {
        match akc {
            anes::KeyCode::Up => KeyCode::Up,
            anes::KeyCode::Down => KeyCode::Down,
            anes::KeyCode::Left => KeyCode::Left,
            anes::KeyCode::Right => KeyCode::Right,
            anes::KeyCode::Backspace => KeyCode::Backspace,
            anes::KeyCode::Enter => KeyCode::Enter,
            anes::KeyCode::Home => KeyCode::Home,
            anes::KeyCode::End => KeyCode::End,
            anes::KeyCode::PageUp => KeyCode::PageUp,
            anes::KeyCode::PageDown => KeyCode::PageDown,
            anes::KeyCode::Tab => KeyCode::Tab,
            anes::KeyCode::BackTab => KeyCode::BackTab,
            anes::KeyCode::Delete => KeyCode::Delete,
            anes::KeyCode::Insert => KeyCode::Insert,
            anes::KeyCode::F(x) => KeyCode::F(x),
            anes::KeyCode::Char(ch) => KeyCode::Char(ch),
            anes::KeyCode::Null => KeyCode::Null,
            anes::KeyCode::Esc => KeyCode::Esc,
        }
    }
}

impl From<anes::MouseButton> for MouseButton {
    fn from(amb: anes::MouseButton) -> Self {
        match amb {
            anes::MouseButton::Left => MouseButton::Left,
            anes::MouseButton::Middle => MouseButton::Middle,
            anes::MouseButton::Right => MouseButton::Right,
        }
    }
}

impl From<anes::Sequence> for InternalEvent {
    fn from(seq: anes::Sequence) -> Self {
        match seq {
            anes::Sequence::CursorPosition(x, y) => InternalEvent::CursorPosition(x - 1, y - 1),
            anes::Sequence::Key(code, modifiers) => InternalEvent::Event(Event::Key(KeyEvent {
                code: code.into(),
                modifiers: modifiers.into(),
            })),
            anes::Sequence::Mouse(mouse) => {
                let mouse = match mouse {
                    anes::Mouse::Down(button, x, y, modifiers) => {
                        MouseEvent::Down(button.into(), x - 1, y - 1, modifiers.into())
                    }
                    anes::Mouse::Up(button, x, y, modifiers) => {
                        MouseEvent::Up(button.into(), x - 1, y - 1, modifiers.into())
                    }
                    anes::Mouse::Drag(button, x, y, modifiers) => {
                        MouseEvent::Drag(button.into(), x - 1, y - 1, modifiers.into())
                    }
                    anes::Mouse::ScrollUp(x, y, modifiers) => {
                        MouseEvent::ScrollUp(x - 1, y - 1, modifiers.into())
                    }
                    anes::Mouse::ScrollDown(x, y, modifiers) => {
                        MouseEvent::ScrollDown(x - 1, y - 1, modifiers.into())
                    }
                };

                InternalEvent::Event(Event::Mouse(mouse))
            }
        }
    }
}
