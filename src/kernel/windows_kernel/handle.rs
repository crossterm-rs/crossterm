extern crate winapi;
extern crate kernel32;
use std::os::windows::io::{AsRawHandle, RawHandle};
use self::winapi::{HANDLE, STD_OUTPUT_HANDLE, STD_INPUT_HANDLE};
use std::mem;

static mut CONSOLE_INPUT_HANDLE:Option<HANDLE> = None;

static mut CONSOLE_OUTPUT_HANDLE:Option<HANDLE> = None;

/// Get the std_output_handle of the console
pub fn get_output_handle() -> Option<HANDLE>
{
    unsafe
    {
        //  if let Some(handle) = CONSOLE_OUTPUT_HANDLE
        //  {
        //      match handle_check(&handle)
        //      {
        //          true => {
        //              CONSOLE_OUTPUT_HANDLE = Some(handle);
        //              CONSOLE_OUTPUT_HANDLE
        //          },
        //          false => None
        //      }

        //  } else {
            let handle = kernel32::GetStdHandle(STD_OUTPUT_HANDLE);

         match handle_check(&handle)
         {
                 true => {
                     // CONSOLE_OUTPUT_HANDLE = Some(handle);
                     Some(handle)
                 },
                 false => None
             }
         }
    // }
}

/// Get the std_input_handle of the console
pub fn get_input_handle() -> Option<HANDLE>
{
    unsafe
    {
        if let Some(handle) = CONSOLE_INPUT_HANDLE
        {
            match handle_check(&handle)
            {
                true => {
                    CONSOLE_INPUT_HANDLE = Some(handle);
                    CONSOLE_INPUT_HANDLE
                },
                false => None
            }
        }
        else
        {
            let handle = kernel32::GetStdHandle(STD_INPUT_HANDLE);
            match handle_check(&handle)
            {
                true => {
                    CONSOLE_INPUT_HANDLE = Some(handle);
                    CONSOLE_INPUT_HANDLE
                },
                false => None
            }
        }
    }
}

pub fn as_raw_handle() -> Option<(HANDLE, winapi::CONSOLE_SCREEN_BUFFER_INFO)>
{
    unsafe
    {
        let hand = kernel32::GetStdHandle(STD_OUTPUT_HANDLE) as RawHandle;

        let mut csbi: winapi::CONSOLE_SCREEN_BUFFER_INFO = mem::zeroed() ;

        match kernel32::GetConsoleScreenBufferInfo(hand, &mut csbi)
        {
            0 => None,
            _ => Some((hand, csbi)),
        }
    }
}

/// Checks if the console handle is an invalid handle value.
fn handle_check(handle: &HANDLE) -> bool
{
    if *handle == winapi::INVALID_HANDLE_VALUE
    {
        false
    }else{
        true
    }
}
