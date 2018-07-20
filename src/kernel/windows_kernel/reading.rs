use { Context, ScreenManager };
use std::rc::Rc;
use std::sync::Mutex;

use winapi::um::consoleapi::ReadConsoleW;
use winapi::um::winnt::HANDLE;
use winapi::um::wincon::{ COORD, PSMALL_RECT, ReadConsoleOutputA, CHAR_INFO, };
use winapi::shared::minwindef::{ DWORD, LPDWORD, LPVOID };
use winapi::shared::ntdef::NULL;

use super::kernel;
use winapi::ctypes::c_void;

pub fn read(buf: &mut [u8], screen_manager: &Rc<Mutex<ScreenManager>>) {
//    // Read more if the buffer is empty
//    let mut utf16: Vec<u16> = Vec::new();
//    let mut num: DWORD = 0;
//
//    let handle = kernel::get_current_handle(&screen_manager);
//
//    unsafe {
//        ReadConsoleW(handle,
//                                utf16.as_mut_ptr() as LPVOID,
//                                utf16.len() as u32,
//                                &mut num as LPDWORD,
//                                ptr::mut_null())
//    };
//
//    utf16.truncate(num as uint);
//    let utf8 = match from_utf16(utf16.as_slice()) {
//        Some(utf8) => utf8.into_bytes(),
//        None => {}
//    };
//
//    panic!(utf8);

}

pub fn read_line(screen_manager: &Rc<Mutex<ScreenManager>>) -> ::std::io::Result<String>
{
    const BUFFER_LENGHT: u32 = 1024;
    let mut buffer: &mut [CHAR_INFO; BUFFER_LENGHT as usize] = unsafe {::std::mem::zeroed() };

    let handle = kernel::get_current_handle(&screen_manager);

    let mut dw_mode: DWORD = 0;
    let console_mode = kernel::get_console_mode(&handle, &mut dw_mode);

    let ptr = buffer.as_ptr() as *const _ as *mut c_void;
    let mut chars_read: u32 = 0;

    panic!();
    unsafe
    {
        ReadConsoleW(handle, ptr, BUFFER_LENGHT , &mut chars_read, unsafe {::std::mem::zeroed() });
    }

    Ok(String::new())
}

/// Read the console outptut.
pub fn read_console_output(
    read_buffer: &HANDLE,
    copy_buffer: &mut [CHAR_INFO; 160],
    buffer_size: COORD,
    buffer_coord: COORD,
    source_buffer: PSMALL_RECT,
) {


    unsafe {
        if !kernel::is_true(
            ReadConsoleOutputA(
                *read_buffer,             // screen buffer to read from
                copy_buffer.as_mut_ptr(), // buffer to copy into
                buffer_size,              // col-row size of chiBuffer
                buffer_coord,             // top left dest. cell in chiBuffer
                source_buffer,
            ), // screen buffer source rectangle
        ) {
            panic!("Cannot read console output");
        }
    }
}
