use super::handle;

use std::io::{Result, Error};

use std::{
    mem::{size_of_val, zeroed},
    os::windows::io::FromRawHandle,
    ptr::{null, null_mut},
};

use winapi::{
    um::{
        consoleapi::{AllocConsole, GetConsoleCP, GetConsoleOutputCP, GetNumberOfConsoleInputEvents, ReadConsoleInputW, PeekConsoleInputA},
        fileapi::{CreateFileW, OPEN_EXISTING},
        handleapi::INVALID_HANDLE_VALUE,
        wincon::{AttachConsole, CHAR_INFO, CONSOLE_FONT_INFOEX, CONSOLE_SCREEN_BUFFER_INFO, CONSOLE_SCREEN_BUFFER_INFOEX, CONSOLE_TEXTMODE_BUFFER, COORD, FlushConsoleInputBuffer, FOCUS_EVENT, FreeConsole, INPUT_RECORD, KEY_EVENT, MENU_EVENT, MOUSE_EVENT, SetConsoleActiveScreenBuffer, SetConsoleCP, SetConsoleOutputCP, SetConsoleScreenBufferInfoEx, SMALL_RECT, WINDOW_BUFFER_SIZE_EVENT, WriteConsoleOutputW},
        winnt::HANDLE
    },
    shared::minwindef::{DWORD, FALSE},
};

pub struct InputBuffer(HANDLE);

impl InputBuffer {
    pub fn new() -> Result<InputBuffer> {
        let handle = handle::get_current_in_handle()?;
        Ok(InputBuffer(handle))
    }

    /// The number of input that is available to read
    pub fn available_input(&self) -> Result<u32> {
        let mut num = 0;
        let res = unsafe { GetNumberOfConsoleInputEvents(self.0, &mut num) };
        if res == 0 { return Err(Error::last_os_error()) }
        Ok(num)
    }
    /// Reads a bunch of input events
    pub fn read_input(&self) -> Result<Vec<Input>> {
        let mut buf: [INPUT_RECORD; 0x1000] = unsafe { zeroed() };
        let mut size = 0;
        let res = unsafe { PeekConsoleInputA(
            self.0, buf.as_mut_ptr(), buf.len() as DWORD, &mut size,
        )};
        if res == 0 { return Err(Error::last_os_error()) }
        Ok(buf[..(size as usize)].iter().map(|input| {
            unsafe { match input.EventType {
                KEY_EVENT => {
                    let e = input.Event.KeyEvent();
                    Input::Key {
                        key_down: e.bKeyDown != 0,
                        repeat_count: e.wRepeatCount,
                        key_code: e.wVirtualKeyCode,
                        scan_code: e.wVirtualScanCode,
                        wide_char: *e.uChar.UnicodeChar(),
                        control_key_state: e.dwControlKeyState,
                    }
                },
                MOUSE_EVENT => {
                    let e = input.Event.MouseEvent();
                    Input::Mouse {
                        position: (e.dwMousePosition.X, e.dwMousePosition.Y),
                        button_state: e.dwButtonState,
                        control_key_state: e.dwControlKeyState,
                        event_flags: e.dwEventFlags,
                    }
                },
                WINDOW_BUFFER_SIZE_EVENT => {
                    let s = input.Event.WindowBufferSizeEvent().dwSize;
                    Input::WindowBufferSize(s.X, s.Y)
                },
                MENU_EVENT => Input::Menu(input.Event.MenuEvent().dwCommandId),
                FOCUS_EVENT => Input::Focus(input.Event.FocusEvent().bSetFocus != 0),
                e => unreachable!("invalid event type: {}", e),
            } }
        }).collect())
    }
    /// Clears all pending input
    pub fn flush_input(&self) -> Result<()> {
        let res = unsafe { FlushConsoleInputBuffer(self.0) };
        if res == 0 { return Err(Error::last_os_error()) }
        Ok(())
    }
}

#[repr(transparent)] #[derive(Copy, Clone)]
pub struct FontInfoEx(CONSOLE_FONT_INFOEX);

#[derive(Copy, Clone, Debug)]
pub enum Input {
    Key {
        key_down: bool,
        repeat_count: u16,
        key_code: u16,
        scan_code: u16,
        wide_char: u16,
        control_key_state: u32,
    },
    Mouse {
        position: (i16, i16),
        button_state: u32,
        control_key_state: u32,
        event_flags: u32,
    },
    WindowBufferSize(i16, i16),
    Menu(u32),
    Focus(bool),
}

#[repr(transparent)] #[derive(Copy, Clone)]
pub struct CharInfo(CHAR_INFO);

impl CharInfo {
    pub fn new(ch: u16, attr: u16) -> CharInfo {
        let mut ci: CHAR_INFO = unsafe { zeroed() };
        unsafe { *ci.Char.UnicodeChar_mut() = ch };
        ci.Attributes = attr;
        CharInfo(ci)
    }
    pub fn character(&self) -> u16 { unsafe { *self.0.Char.UnicodeChar() } }
    pub fn attributes(&self) -> u16 { self.0.Attributes }
}