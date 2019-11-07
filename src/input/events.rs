#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents an input event.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialOrd, PartialEq, Hash, Clone)]
pub enum Event {
    /// A single key or a combination of keys.
    Keyboard(KeyEvent),
    /// A mouse event.
    Mouse(MouseEvent),
    /// An unsupported event.
    ///
    /// You can ignore this type of event, because it isn't used.
    Unsupported(Vec<u8>), // TODO Not used, should be removed.
    /// An unknown event.
    Unknown,
}

/// Represents a mouse event.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialOrd, PartialEq, Hash, Clone, Copy)]
pub enum MouseEvent {
    /// Pressed mouse button at the location (column, row).
    Press(MouseButton, u16, u16),
    /// Released mouse button at the location (column, row).
    Release(u16, u16),
    /// Mouse moved with a pressed left button to the new location (column, row).
    Hold(u16, u16),
    /// An unknown mouse event.
    Unknown,
}

/// Represents a mouse button/wheel.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialOrd, PartialEq, Hash, Clone, Copy)]
pub enum MouseButton {
    /// Left mouse button.
    Left,
    /// Right mouse button.
    Right,
    /// Middle mouse button.
    Middle,
    /// Wheel scrolled up.
    WheelUp,
    /// Wheel scrolled down.
    WheelDown,
}

/// Represents a key or a combination of keys.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum KeyEvent {
    /// Backspace key.
    Backspace,
    /// Enter key.
    Enter,
    /// Left arrow key.
    Left,
    /// Right arrow key.
    Right,
    /// Up arrow key.
    Up,
    /// Down arrow key.
    Down,
    /// Home key.
    Home,
    /// End key.
    End,
    /// Page up key.
    PageUp,
    /// Page dow key.
    PageDown,
    /// Tab key.
    Tab,
    /// Shift + Tab key.
    BackTab,
    /// Delete key.
    Delete,
    /// Insert key.
    Insert,
    /// F key.
    ///
    /// `KeyEvent::F(1)` represents F1 key, etc.
    F(u8),
    /// A character.
    ///
    /// `KeyEvent::Char('c')` represents `c` character, etc.
    Char(char),
    /// Alt key + character.
    ///
    /// `KeyEvent::Alt('c')` represents `Alt + c`, etc.
    Alt(char),
    /// Ctrl key + character.
    ///
    /// `KeyEvent::Ctrl('c') ` represents `Ctrl + c`, etc.
    Ctrl(char),
    /// Null.
    Null,
    /// Escape key.
    Esc,
    /// Ctrl + up arrow key.
    CtrlUp,
    /// Ctrl + down arrow key.
    CtrlDown,
    /// Ctrl + right arrow key.
    CtrlRight,
    /// Ctrl + left arrow key.
    CtrlLeft,
    /// Shift + up arrow key.
    ShiftUp,
    /// Shift + down arrow key.
    ShiftDown,
    /// Shift + right arrow key.
    ShiftRight,
    /// Shift + left arrow key.
    ShiftLeft,
}

/// An internal event.
///
/// Encapsulates publicly available `InputEvent` with additional internal
/// events that shouldn't be publicly available to the crate users.
#[derive(Debug, PartialOrd, PartialEq, Hash, Clone)]
pub enum InternalEvent {
    /// An input event.
    Input(Event),
    /// A cursor position (`x`, `y`).
    CursorPosition(u16, u16),
}
