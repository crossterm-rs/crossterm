use std::io;
use std::io::Write;
use std::char;
use std::sync::mpsc;
use std::thread;

use super::super::terminal::terminal;
//use super::super::kernel::unix_kernel::terminal::get_tty;
use super::{ Key, ITerminalInput, AsyncReader };

pub struct UnixInput;

impl UnixInput
{
    pub fn new() -> UnixInput
    {
        UnixInput {}
    }
}
//    fn read_line(&self) -> io::Result<String>
//    {
//        let mut rv = String::new();
//        io::stdin().read_line(&mut rv)?;
//        let len = rv.trim_right_matches(&['\r', '\n'][..]).len();
//        rv.truncate(len);
//        Ok(rv)
//    }
//
//    fn read_char(&self) -> io::Result<char>
//    {
//        let mut buf = [0u8; 20];
//        let mut termios = termios::Termios::from_fd(fd)?;
//        let original = termios.clone();
//        termios::cfmakeraw(&mut termios);
//        termios::tcsetattr(fd, termios::TCSADRAIN, &termios)?;
//        let rv = unsafe {
//            let read = libc::read(fd, buf.as_mut_ptr() as *mut libc::c_void, 20);
//            if read < 0 {
//                Err(io::Error::last_os_error())
//            } else if buf[0] == b'\x03' {
//                Err(io::Error::new(io::ErrorKind::Interrupted, "read interrupted"))
//            } else {
//                Ok(key_from_escape_codes(&buf[..read as usize]))
//            }
//        };
//        termios::tcsetattr(fd, termios::TCSADRAIN, &original)?;
//
//        // if the user hit ^C we want to signal SIGINT to outselves.
//        if let Err(ref err) = rv {
//            if err.kind() == io::ErrorKind::Interrupted {
//                unsafe { libc::raise(libc::SIGINT); }
//            }
//        }
//
//        rv
//    }
//
//    fn read_pressed_key(&self) -> io::Result<Key>
//    {
//        use Context;
//        let context = Context::new();
//
//        let buf: [u8; 1024] = unsafe { ::std::mem::zeroed() };
////        reading::read(&mut buf, &context.screen_manager);
//
//        Ok(Key::Unknown)
////        let pressed_char = unsafe { _getwch() };
////
////        // if 0 or 0xe0 we need to listen again because the next key will be an special key
////        if pressed_char == 0 || pressed_char == 0xe0 {
////            let special_key: i32 = unsafe { _getwch() };
////            println!("spkey {}",special_key);
////            return Ok(key_from_key_code(0x26));
////        } else {
////            match char::from_u32(pressed_char as u32)
////            {
////                Some(c) => return Ok(Key::Char(c)),
////                None => { panic!("Some error needs to be returned") }
////            }
////        }
//    }
//
//    fn read_async(&self) -> AsyncReader
//    {
//        let (send, recv) = mpsc::channel();
//
//        thread::spawn(move || for i in get_tty().unwrap().bytes() {
//
//            match i {
//                Ok(byte) => {
//                    let end_of_stream = &byte == &delimiter;
//                    let send_error = send.send(Ok(byte)).is_err();
//
//                    if end_of_stream || send_error { return; }
//                },
//                Err(_) => { return; }
//            }
//        });
//
//        AsyncReader { recv: recv }
//    }
//
//    fn read_until_async(&self, delimiter: u8) -> AsyncReader
//    {
//        let (tx, rx) = mpsc::channel();
//
//        thread::spawn(move || {
//            loop
//            {
//                let pressed_char: u8 = (unsafe { _getwch() }) as u8;
//
//                let end_of_stream = (pressed_char == delimiter);
//
//                // we could return error but maybe option to keep listening until valid character is inputted.
//                if pressed_char == 0 || pressed_char == 0xe0 || end_of_stream {
//                    return;
//                }
//
//                tx.send(Ok(pressed_char as u8));
//            }
//        });
//
//        AsyncReader { recv: rx }
//    }
//}

