use anes;

use super::{Event, InternalEvent, KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent};
use anes::parser;

impl From<parser::KeyModifiers> for KeyModifiers {
    fn from(akm: parser::KeyModifiers) -> Self {
        let mut modifiers = KeyModifiers::empty();

        if akm.contains(parser::KeyModifiers::SHIFT) {
            modifiers |= KeyModifiers::SHIFT;
        }

        if akm.contains(parser::KeyModifiers::ALT) {
            modifiers |= KeyModifiers::ALT;
        }

        if akm.contains(parser::KeyModifiers::CONTROL) {
            modifiers |= KeyModifiers::CONTROL;
        }

        // TODO - crossterm lacks ::META

        modifiers
    }
}

impl From<parser::KeyCode> for KeyCode {
    fn from(akc: parser::KeyCode) -> Self {
        match akc {
            parser::KeyCode::Up => KeyCode::Up,
            parser::KeyCode::Down => KeyCode::Down,
            parser::KeyCode::Left => KeyCode::Left,
            parser::KeyCode::Right => KeyCode::Right,
            parser::KeyCode::Backspace => KeyCode::Backspace,
            parser::KeyCode::Enter => KeyCode::Enter,
            parser::KeyCode::Home => KeyCode::Home,
            parser::KeyCode::End => KeyCode::End,
            parser::KeyCode::PageUp => KeyCode::PageUp,
            parser::KeyCode::PageDown => KeyCode::PageDown,
            parser::KeyCode::Tab => KeyCode::Tab,
            parser::KeyCode::BackTab => KeyCode::BackTab,
            parser::KeyCode::Delete => KeyCode::Delete,
            parser::KeyCode::Insert => KeyCode::Insert,
            parser::KeyCode::F(x) => KeyCode::F(x),
            parser::KeyCode::Char(ch) => KeyCode::Char(ch),
            parser::KeyCode::Null => KeyCode::Null,
            parser::KeyCode::Esc => KeyCode::Esc,
        }
    }
}

impl From<parser::MouseButton> for MouseButton {
    fn from(amb: parser::MouseButton) -> Self {
        match amb {
            parser::MouseButton::Left => MouseButton::Left,
            parser::MouseButton::Middle => MouseButton::Middle,
            parser::MouseButton::Right => MouseButton::Right,
            parser::MouseButton::Any => MouseButton::Middle
        }
    }
}

impl From<parser::Sequence> for InternalEvent {
    fn from(seq: parser::Sequence) -> Self {
        match seq {
            parser::Sequence::CursorPosition(x, y) => InternalEvent::CursorPosition(x - 1, y - 1),
            parser::Sequence::Key(code, modifiers) => InternalEvent::Event(Event::Key(KeyEvent {
                code: code.into(),
                modifiers: modifiers.into(),
            })),
            parser::Sequence::Mouse(mouse, modifiers) => {
                let mouse = match mouse {
                    parser::Mouse::Down(button, x, y) => {
                        MouseEvent::Down(button.into(), x - 1, y - 1, modifiers.into())
                    }
                    parser::Mouse::Up(button, x, y) => {
                        MouseEvent::Up(button.into(), x - 1, y - 1, modifiers.into())
                    }
                    parser::Mouse::Drag(button, x, y) => {
                        MouseEvent::Drag(button.into(), x - 1, y - 1, modifiers.into())
                    }
                    parser::Mouse::ScrollUp(x, y) => {
                        MouseEvent::ScrollUp(x - 1, y - 1, modifiers.into())
                    }
                    parser::Mouse::ScrollDown(x, y) => {
                        MouseEvent::ScrollDown(x - 1, y - 1, modifiers.into())
                    }
                };

                InternalEvent::Event(Event::Mouse(mouse))
            }
        }
    }
}
