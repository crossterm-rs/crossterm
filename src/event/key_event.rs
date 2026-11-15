use std::cmp::Ordering;
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
///
/// Build one with [`KeyEvent::new`] and the `with_*` methods. More fields may be added in minor
/// releases, so struct literals and exhaustive patterns are not available outside crossterm.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
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
    /// Creates a key press event with the given code and modifiers.
    ///
    /// The kind is [`KeyEventKind::Press`] and the state is empty. Use [`with_kind`] and
    /// [`with_state`] to change them.
    ///
    /// [`with_kind`]: Self::with_kind
    /// [`with_state`]: Self::with_state
    ///
    /// # Examples
    ///
    /// ```
    /// use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
    ///
    /// let event = KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL);
    /// assert_eq!(event.kind, KeyEventKind::Press);
    /// ```
    pub const fn new(code: KeyCode, modifiers: KeyModifiers) -> KeyEvent {
        KeyEvent {
            code,
            modifiers,
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        }
    }

    /// Creates a key event with the given code, modifiers and kind, and an empty state.
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

    /// Creates a key event with every field given.
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

    /// Returns this key event with its kind replaced.
    ///
    /// # Examples
    ///
    /// ```
    /// use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
    ///
    /// let event = KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE).with_kind(KeyEventKind::Release);
    /// assert!(event.is_release());
    /// ```
    #[must_use]
    pub const fn with_kind(mut self, kind: KeyEventKind) -> KeyEvent {
        self.kind = kind;
        self
    }

    /// Returns this key event with its state replaced.
    ///
    /// # Examples
    ///
    /// ```
    /// use crossterm::event::{KeyCode, KeyEvent, KeyEventState, KeyModifiers};
    ///
    /// let event = KeyEvent::new(KeyCode::Char('1'), KeyModifiers::NONE)
    ///     .with_state(KeyEventState::KEYPAD);
    /// assert!(event.state.contains(KeyEventState::KEYPAD));
    /// ```
    #[must_use]
    pub const fn with_state(mut self, state: KeyEventState) -> KeyEvent {
        self.state = state;
        self
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

impl PartialOrd for KeyEvent {
    fn partial_cmp(&self, other: &KeyEvent) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Compares the case-normalized events, so that ordering agrees with `PartialEq`.
impl Ord for KeyEvent {
    fn cmp(&self, other: &KeyEvent) -> Ordering {
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
        let lhs = (lhs_code, lhs_modifiers, lhs_kind, lhs_state);
        let rhs = (rhs_code, rhs_modifiers, rhs_kind, rhs_state);
        lhs.cmp(&rhs)
    }
}

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
    use std::cmp::Ordering;
    use std::collections::BTreeSet;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    use crate::event::Event;

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
    fn test_ordering_matches_equality() {
        let lowercase_d_with_shift = KeyEvent::new(KeyCode::Char('d'), KeyModifiers::SHIFT);
        let uppercase_d_with_shift = KeyEvent::new(KeyCode::Char('D'), KeyModifiers::SHIFT);
        let uppercase_d = KeyEvent::new(KeyCode::Char('D'), KeyModifiers::NONE);
        let events = [lowercase_d_with_shift, uppercase_d_with_shift, uppercase_d];
        for lhs in events {
            for rhs in events {
                assert_eq!(lhs.cmp(&rhs), Ordering::Equal, "{lhs:?} vs {rhs:?}");
                assert_eq!(lhs.partial_cmp(&rhs), Some(Ordering::Equal));
                assert_eq!(Event::Key(lhs).cmp(&Event::Key(rhs)), Ordering::Equal);
            }
        }

        let bindings = BTreeSet::from([uppercase_d]);
        assert!(bindings.contains(&uppercase_d_with_shift));
    }

    #[test]
    fn enum_variant_is() {
        assert!(KeyEventKind::Press.is_press());
        assert!(!KeyEventKind::Press.is_release());
    }
}
