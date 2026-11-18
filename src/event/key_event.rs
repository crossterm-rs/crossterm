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
#[derive(Debug, PartialOrd, Ord, Clone, Copy)]
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
    /// The key at the same physical position on a standard PC-101 keyboard, independent of the
    /// active keyboard layout.
    ///
    /// This is what makes layout-independent shortcuts possible: with a Cyrillic layout active,
    /// the physical `C` key reports `KeyCode::Char('с')` in [`code`](Self::code) and
    /// `Some(KeyCode::Char('c'))` here, so an application can match `Ctrl+C` regardless of the
    /// layout the user types in.
    ///
    /// Only set if [`KeyboardEnhancementFlags::REPORT_ALTERNATE_KEYS`] has been enabled with
    /// [`PushKeyboardEnhancementFlags`], and only on Unix — the Windows console API does not
    /// report it.
    ///
    /// This field deliberately takes no part in [`PartialEq`]/[`Hash`], so that comparing against
    /// a manually constructed event — the common
    /// `event == KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL)` pattern — keeps
    /// working once the enhancement is enabled.
    #[cfg_attr(feature = "serde", serde(default))]
    pub base_layout_code: Option<KeyCode>,
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
            base_layout_code: None,
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
            base_layout_code: None,
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
            base_layout_code: None,
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

    /// Returns this key event with its base layout key replaced.
    ///
    /// See [`base_layout_code`](Self::base_layout_code). It takes no part in equality, so the
    /// result still compares equal to the event it was built from.
    ///
    /// # Examples
    ///
    /// ```
    /// use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
    ///
    /// let event = KeyEvent::new(KeyCode::Char('с'), KeyModifiers::CONTROL)
    ///     .with_base_layout_code(Some(KeyCode::Char('c')));
    /// assert_eq!(event.base_layout_code, Some(KeyCode::Char('c')));
    /// assert_eq!(event, KeyEvent::new(KeyCode::Char('с'), KeyModifiers::CONTROL));
    /// ```
    #[must_use]
    pub const fn with_base_layout_code(mut self, base_layout_code: Option<KeyCode>) -> KeyEvent {
        self.base_layout_code = base_layout_code;
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
            base_layout_code: None,
        }
    }
}

impl PartialEq for KeyEvent {
    fn eq(&self, other: &KeyEvent) -> bool {
        // `base_layout_code` is metadata about the same key press, not part of its identity:
        // including it would break `event == KeyEvent::new(...)` comparisons as soon as
        // `REPORT_ALTERNATE_KEYS` is enabled.
        let KeyEvent {
            code: lhs_code,
            modifiers: lhs_modifiers,
            kind: lhs_kind,
            state: lhs_state,
            base_layout_code: _,
        } = self.normalize_case();
        let KeyEvent {
            code: rhs_code,
            modifiers: rhs_modifiers,
            kind: rhs_kind,
            state: rhs_state,
            base_layout_code: _,
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
        // Excluded for the same reason as in `PartialEq`, and to keep `Hash` consistent with it.
        let KeyEvent {
            code,
            modifiers,
            kind,
            state,
            base_layout_code: _,
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
