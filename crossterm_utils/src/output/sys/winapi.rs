//! This module contains the logic to write to the terminal.

use winapi::ctypes::c_void;
use winapi::shared::ntdef::NULL;
use winapi::um::consoleapi::WriteConsoleW;
use winapi::um::wincon::{WriteConsoleOutputA, CHAR_INFO, COORD, PSMALL_RECT};
use winapi::um::winnt::HANDLE;

use crossterm_winapi::{is_true, ScreenBuffer};

use std::io::{self, Result};
use std::str;

