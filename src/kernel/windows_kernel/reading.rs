use super::handle::get_current_in_handle;
use std::io::{self, Error, Result};

use std::{
    mem::{self, zeroed},
    ptr::{null, null_mut},
};

use winapi::{
    shared::minwindef::{LPVOID, ULONG},
    um::consoleapi::{ReadConsoleInputW, ReadConsoleW},
    um::wincon::CONSOLE_READCONSOLE_CONTROL,
    um::wincon::{CHAR_INFO, CONSOLE_FONT_INFOEX, INPUT_RECORD, PCONSOLE_READCONSOLE_CONTROL},
};

use std::io::Write;

/// Could be used to read a line from the stdin.
/// Note that this is a blocking call and it continues when user pressed enter.
pub fn read_line(buf: &mut Vec<u8>) -> io::Result<usize> {
    let handle = get_current_in_handle()?;

    let mut utf16 = vec![0u16; 0x1000];
    let mut num = 0;
    let mut input_control = readconsole_input_control(CTRL_Z_MASK);

    unsafe {
        ReadConsoleW(
            handle,
            utf16.as_mut_ptr() as LPVOID,
            utf16.len() as u32,
            &mut num,
            &mut input_control as PCONSOLE_READCONSOLE_CONTROL,
        )
    };

    utf16.truncate(num as usize);

    let mut data = match String::from_utf16(&utf16) {
        Ok(utf8) => utf8.into_bytes(),
        Err(..) => return Err(invalid_encoding()),
    };

    if let Some(&last_byte) = data.last() {
        if last_byte == CTRL_Z {
            data.pop();
        }
    };

    let a = &data
        .into_iter()
        .filter(|&x| x != 10 || x != 13)
        .collect::<Vec<u8>>();

    buf.write(a);
    Ok(num as usize)
}

pub fn readconsole_input_control(wakeup_mask: ULONG) -> CONSOLE_READCONSOLE_CONTROL {
    CONSOLE_READCONSOLE_CONTROL {
        nLength: mem::size_of::<CONSOLE_READCONSOLE_CONTROL>() as ULONG,
        nInitialChars: 0,
        dwCtrlWakeupMask: wakeup_mask,
        dwControlKeyState: 0,
    }
}

fn invalid_encoding() -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, "text was not valid unicode")
}

const CTRL_Z: u8 = 0x1A;
const CTRL_Z_MASK: ULONG = 0x4000000; //1 << 0x1A
