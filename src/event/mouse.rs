use super::KeyModifiers;

/// Represents a mouse event.
///
/// # Platform-specific Notes
///
/// ## Mouse Buttons
///
/// Some platforms/terminals do not report mouse button for the
/// `MouseEventKind::Up` and `MouseEventKind::Drag` events. `MouseButton::Left`
/// is returned if we don't know which button was used.
///
/// ## Key Modifiers
///
/// Some platforms/terminals does not report all key modifiers
/// combinations for all mouse event types. For example - macOS reports
/// `Ctrl` + left mouse button click as a right mouse button click.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
pub struct MouseEvent {
    /// The kind of mouse event that was caused.
    pub kind: MouseEventKind,
    /// The column that the event occurred on.
    pub column: u16,
    /// The row that the event occurred on.
    pub row: u16,
    /// The key modifiers active when the event occurred.
    pub modifiers: KeyModifiers,
}

impl MouseEvent {
    /// Creates a mouse event.
    ///
    /// # Examples
    ///
    /// ```
    /// use crossterm::event::{KeyModifiers, MouseButton, MouseEvent, MouseEventKind};
    ///
    /// let event = MouseEvent::new(MouseEventKind::Down(MouseButton::Left), 3, 7, KeyModifiers::NONE);
    /// assert_eq!((event.column, event.row), (3, 7));
    /// ```
    pub const fn new(
        kind: MouseEventKind,
        column: u16,
        row: u16,
        modifiers: KeyModifiers,
    ) -> MouseEvent {
        MouseEvent {
            kind,
            column,
            row,
            modifiers,
        }
    }
}

/// A mouse event kind.
///
/// # Platform-specific Notes
///
/// ## Mouse Buttons
///
/// Some platforms/terminals do not report mouse button for the
/// `MouseEventKind::Up` and `MouseEventKind::Drag` events. `MouseButton::Left`
/// is returned if we don't know which button was used.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
pub enum MouseEventKind {
    /// Pressed mouse button. Contains the button that was pressed.
    Down(MouseButton),
    /// Released mouse button. Contains the button that was released.
    Up(MouseButton),
    /// Moved the mouse cursor while pressing the contained mouse button.
    Drag(MouseButton),
    /// Moved the mouse cursor while not pressing a mouse button.
    Moved,
    /// Scrolled mouse wheel downwards (towards the user).
    ScrollDown,
    /// Scrolled mouse wheel upwards (away from the user).
    ScrollUp,
    /// Scrolled mouse wheel left (mostly on a laptop touchpad).
    ScrollLeft,
    /// Scrolled mouse wheel right (mostly on a laptop touchpad).
    ScrollRight,
}

impl MouseEventKind {
    /// Returns the mouse button connected with the event.
    ///
    /// This will return `Some(MouseButton)` for `Down`, `Up`, and `Drag` events.
    /// For other events, like `Moved` or `Scroll...`, it returns `None`.
    pub fn button(&self) -> Option<MouseButton> {
        match self {
            MouseEventKind::Down(button)
            | MouseEventKind::Up(button)
            | MouseEventKind::Drag(button) => Some(*button),
            _ => None,
        }
    }
}

impl_is_variant! {
    MouseEventKind {
        Down(..) => is_down,
        Up(..) => is_up,
        Drag(..) => is_drag,
        Moved => is_moved,
        ScrollDown => is_scroll_down,
        ScrollUp => is_scroll_up,
        ScrollLeft => is_scroll_left,
        ScrollRight => is_scroll_right,
    }
}

/// Represents a mouse button.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
pub enum MouseButton {
    /// Left mouse button.
    Left,
    /// Right mouse button.
    Right,
    /// Middle mouse button.
    Middle,
}

impl_is_variant! {
    MouseButton {
        Left => is_left,
        Right => is_right,
        Middle => is_middle,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enum_variant_is() {
        let mouse_kind = MouseEventKind::Down(MouseButton::Left);
        assert!(mouse_kind.is_down());
        assert!(!mouse_kind.is_up());

        assert!(MouseButton::Left.is_left());
        assert!(!MouseButton::Left.is_right());
    }

    #[test]
    fn event_button() {
        let event = MouseEventKind::Down(MouseButton::Left);
        assert_eq!(event.button(), Some(MouseButton::Left));

        let event = MouseEventKind::Up(MouseButton::Right);
        assert_eq!(event.button(), Some(MouseButton::Right));

        let event = MouseEventKind::Drag(MouseButton::Middle);
        assert_eq!(event.button(), Some(MouseButton::Middle));

        let event = MouseEventKind::Moved;
        assert_eq!(event.button(), None);
    }
}
