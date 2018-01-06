use winapi;
use super::Empty;

impl Empty for winapi::COORD {
    fn empty() -> winapi::COORD {
        winapi::COORD { X: 0, Y: 0 }
    }
}

impl Empty for winapi::SMALL_RECT {
    fn empty() -> winapi::SMALL_RECT {
        winapi::SMALL_RECT {
            Top: 0,
            Right: 0,
            Bottom: 0,
            Left: 0,
        }
    }
}

impl Empty for winapi::CONSOLE_SCREEN_BUFFER_INFO {
    fn empty() -> winapi::CONSOLE_SCREEN_BUFFER_INFO {
        winapi::CONSOLE_SCREEN_BUFFER_INFO {
            dwSize: winapi::COORD::empty(),
            dwCursorPosition: winapi::COORD::empty(),
            wAttributes: 0,
            srWindow: winapi::SMALL_RECT::empty(),
            dwMaximumWindowSize: winapi::COORD::empty(),
        }
    }
}
