use crate::InputEvent;

pub mod fake;
#[cfg(unix)]
pub mod tty;
#[cfg(windows)]
pub mod winapi;

pub trait EventSource: Sync + Send {
    fn read_event(&mut self) -> crate::Result<Option<InputEvent>>;
}

//pub struct ModifierState {
//    alt_pressed: bool,
//    ctrl_pressed: bool,
//    shift_pressed: bool,
//}
//
//impl ModifierState {
//    pub fn with_alt_press(self) -> Self {
//        self
//    }
//
//    pub fn with_ctrl_press(self) -> Self {
//        self
//    }
//
//    pub fn with_shift_press(self) -> Self {
//        self
//    }
//
//    pub fn alt_pressed(&self) -> bool {
//        self.alt_pressed
//    }
//
//    pub fn ctrl_pressed(&self) -> bool {
//        self.ctrl_pressed
//    }
//
//    pub fn shift_pressed(&self) -> bool {
//        self.shift_pressed
//    }
//}
