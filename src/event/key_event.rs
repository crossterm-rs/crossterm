use std::hash::{Hash, Hasher};

use bitflags::bitflags;

use super::{KeyCode, KeyModifiers};
#[cfg(doc)]
use super::{KeyboardEnhancementFlags, PushKeyboardEnhancementFlags};

/// Represents a keyboard event kind.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
pub enum KeyEventKind {
    Press,
    Repeat,
    Release,
}

impl_is_variant! {
    KeyEventKind {
        Press => is_press,
        Repeat => is_repeat,
        Release => is_release,
    }
}

bitflags! {
    /// Represents extra state about the key event.
    ///
    /// **Note:** This state can only be read if
    /// [`KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES`] has been enabled with
    /// [`PushKeyboardEnhancementFlags`].
    #[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(transparent))]
    pub struct KeyEventState: u8 {
        /// The key event origins from the keypad.
        const KEYPAD = 0b0000_0001;
        /// Caps Lock was enabled for this key event.
        ///
        /// **Note:** this is set for the initial press of Caps Lock itself.
        const CAPS_LOCK = 0b0000_0010;
        /// Num Lock was enabled for this key event.
        ///
        /// **Note:** this is set for the initial press of Num Lock itself.
        const NUM_LOCK = 0b0000_0100;
        const NONE = 0b0000_0000;
    }
}

/// Represents a key event.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialOrd, Ord, Clone, Copy)]
pub struct KeyEvent {
    /// The key itself.
    pub code: KeyCode,
    /// Additional key modifiers.
    pub modifiers: KeyModifiers,
    /// Kind of event.
    ///
    /// Only set if:
    /// - Unix: [`KeyboardEnhancementFlags::REPORT_EVENT_TYPES`] has been enabled with
    ///   [`PushKeyboardEnhancementFlags`].
    /// - Windows: always
    pub kind: KeyEventKind,
    /// Keyboard state.
    ///
    /// Only set if [`KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES`] has been enabled with
    /// [`PushKeyboardEnhancementFlags`].
    pub state: KeyEventState,
}

impl KeyEvent {
    pub const fn new(code: KeyCode, modifiers: KeyModifiers) -> KeyEvent {
        KeyEvent {
            code,
            modifiers,
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        }
    }

    pub const fn new_with_kind(
        code: KeyCode,
        modifiers: KeyModifiers,
        kind: KeyEventKind,
    ) -> KeyEvent {
        KeyEvent {
            code,
            modifiers,
            kind,
            state: KeyEventState::empty(),
        }
    }

    pub const fn new_with_kind_and_state(
        code: KeyCode,
        modifiers: KeyModifiers,
        kind: KeyEventKind,
        state: KeyEventState,
    ) -> KeyEvent {
        KeyEvent {
            code,
            modifiers,
            kind,
            state,
        }
    }

    // modifies the KeyEvent,
    // so that KeyModifiers::SHIFT is present iff
    // an uppercase char is present.
    fn normalize_case(mut self) -> KeyEvent {
        let c = match self.code {
            KeyCode::Char(c) => c,
            _ => return self,
        };

        if c.is_ascii_uppercase() {
            self.modifiers.insert(KeyModifiers::SHIFT);
        } else if self.modifiers.contains(KeyModifiers::SHIFT) {
            self.code = KeyCode::Char(c.to_ascii_uppercase())
        }
        self
    }

    /// Returns whether the key event is a press event.
    pub fn is_press(&self) -> bool {
        matches!(self.kind, KeyEventKind::Press)
    }

    /// Returns whether the key event is a release event.
    pub fn is_release(&self) -> bool {
        matches!(self.kind, KeyEventKind::Release)
    }

    /// Returns whether the key event is a repeat event.
    pub fn is_repeat(&self) -> bool {
        matches!(self.kind, KeyEventKind::Repeat)
    }
}

impl From<KeyCode> for KeyEvent {
    fn from(code: KeyCode) -> Self {
        KeyEvent {
            code,
            modifiers: KeyModifiers::empty(),
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        }
    }
}

impl PartialEq for KeyEvent {
    fn eq(&self, other: &KeyEvent) -> bool {
        let KeyEvent {
            code: lhs_code,
            modifiers: lhs_modifiers,
            kind: lhs_kind,
            state: lhs_state,
        } = self.normalize_case();
        let KeyEvent {
            code: rhs_code,
            modifiers: rhs_modifiers,
            kind: rhs_kind,
            state: rhs_state,
        } = other.normalize_case();
        (lhs_code == rhs_code)
            && (lhs_modifiers == rhs_modifiers)
            && (lhs_kind == rhs_kind)
            && (lhs_state == rhs_state)
    }
}

impl Eq for KeyEvent {}

impl Hash for KeyEvent {
    fn hash<H: Hasher>(&self, hash_state: &mut H) {
        let KeyEvent {
            code,
            modifiers,
            kind,
            state,
        } = self.normalize_case();
        code.hash(hash_state);
        modifiers.hash(hash_state);
        kind.hash(hash_state);
        state.hash(hash_state);
    }
}

#[cfg(test)]
mod tests {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    use super::*;

    #[test]
    fn test_equality() {
        let lowercase_d_with_shift = KeyEvent::new(KeyCode::Char('d'), KeyModifiers::SHIFT);
        let uppercase_d_with_shift = KeyEvent::new(KeyCode::Char('D'), KeyModifiers::SHIFT);
        let uppercase_d = KeyEvent::new(KeyCode::Char('D'), KeyModifiers::NONE);
        assert_eq!(lowercase_d_with_shift, uppercase_d_with_shift);
        assert_eq!(uppercase_d, uppercase_d_with_shift);
    }

    #[test]
    fn test_hash() {
        let lowercase_d_with_shift_hash = {
            let mut hasher = DefaultHasher::new();
            KeyEvent::new(KeyCode::Char('d'), KeyModifiers::SHIFT).hash(&mut hasher);
            hasher.finish()
        };
        let uppercase_d_with_shift_hash = {
            let mut hasher = DefaultHasher::new();
            KeyEvent::new(KeyCode::Char('D'), KeyModifiers::SHIFT).hash(&mut hasher);
            hasher.finish()
        };
        let uppercase_d_hash = {
            let mut hasher = DefaultHasher::new();
            KeyEvent::new(KeyCode::Char('D'), KeyModifiers::NONE).hash(&mut hasher);
            hasher.finish()
        };
        assert_eq!(lowercase_d_with_shift_hash, uppercase_d_with_shift_hash);
        assert_eq!(uppercase_d_hash, uppercase_d_with_shift_hash);
    }

    #[test]
    fn enum_variant_is() {
        assert!(KeyEventKind::Press.is_press());
        assert!(!KeyEventKind::Press.is_release());
    }
}
