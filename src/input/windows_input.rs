use std::io;
use std::io::Write;
use std::char;
use std::sync::mpsc;
use std::thread;

use super::{ Key, ITerminalInput, AsyncReader };

use winapi::um::winnt::{ INT };
use winapi::um::winuser;

use super::super::terminal::terminal;
use super::super::kernel::windows_kernel::reading;
use Context;
use std::rc::Rc;

pub struct WindowsInput
{
    context: Rc<Context>,
    pub display_input: bool,
}

impl WindowsInput
{
    pub fn new(context: Rc<Context>) -> WindowsInput
    {
        WindowsInput { context, display_input: false }
    }
}

impl ITerminalInput for WindowsInput
{
    fn read_line(&self) -> io::Result<String>
    {
        let term = terminal(&self.context);
        let mut chars: Vec<char> = Vec::new();

        loop {
            let pressed_char = unsafe { _getwch() };

            // if 0 or 0xe0 we need to listen again because the next key will be an special key
            if pressed_char != 0 || pressed_char != 0xe0 {
                match char::from_u32(pressed_char as u32)
                {
                    Some(c) => {
                        if  is_line_end(c) { break; }
                        else { chars.push(c); }

                        if self.display_input
                        {
                            term.write(c);
                        }

                    },
                    None => { panic!("Some error needs to be returned") }
                };
            }
        }

        return Ok(chars.into_iter().collect());
    }

    fn read_char(&self) -> io::Result<char>
    {
        let term = terminal(&self.context);

        let pressed_char = unsafe { _getwch() };

        // we could return error but maybe option to keep listening until valid character is inputted.
        if pressed_char == 0 || pressed_char == 0xe0 {
            return Err(io::Error::new(io::ErrorKind::Other, "Given input char is not a valid char, mostly occurs when pressing special keys"));
        }

        match char::from_u32(pressed_char as u32)
        {
            Some(c) => {
                if self.display_input
                {
                    term.write(c);
                }
                return Ok(c);
            }
            None => Err(io::Error::new(io::ErrorKind::Other, "Could not parse given input to char"))
        }
    }

    fn read_pressed_key(&self) -> io::Result<Key>
    {
        use Context;
        let context = Context::new();

        let buf: [u8; 1024] = unsafe { ::std::mem::zeroed() };
//        reading::read(&mut buf, &context.screen_manager);

        Ok(Key::Unknown)
//        let pressed_char = unsafe { _getwch() };
//
//        // if 0 or 0xe0 we need to listen again because the next key will be an special key
//        if pressed_char == 0 || pressed_char == 0xe0 {
//            let special_key: i32 = unsafe { _getwch() };
//            println!("spkey {}",special_key);
//            return Ok(key_from_key_code(0x26));
//        } else {
//            match char::from_u32(pressed_char as u32)
//            {
//                Some(c) => return Ok(Key::Char(c)),
//                None => { panic!("Some error needs to be returned") }
//            }
//        }
    }

    fn read_async(&self) -> AsyncReader
    {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            loop
                {
                    let pressed_char: u8 = (unsafe { _getwch() }) as u8;

                    // we could return error but maybe option to keep listening until valid character is inputted.
                    if pressed_char == 0 || pressed_char == 0xe0 {
                        return;
                    }

                    tx.send(Ok(pressed_char as u8));

                    if pressed_char == 13
                    {
                        return;
                    }
                }
        });

        AsyncReader { recv: rx }
    }

    fn read_until_async(&self, delimiter: u8) -> AsyncReader
    {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            loop
            {
                let pressed_char: u8 = (unsafe { _getwch() }) as u8;

                let end_of_stream = (pressed_char == delimiter);

                // we could return error but maybe option to keep listening until valid character is inputted.
                if pressed_char == 0 || pressed_char == 0xe0 || end_of_stream {
                    return;
                }

                tx.send(Ok(pressed_char as u8));
            }
        });

        AsyncReader { recv: rx }
    }
}

fn is_line_end(key: char) -> bool
{
    if key as u8 == 13
    {
        return true;
    }

    return false;
}


//0 59 = F1
//0 60 = F2
//0 61 = F3
//0 62 = F4
//0 63 = F5
//0 64 = F6
//0 65 = F7
//0 66 = F8
//0 67 = F9
//0 68 = F10
//224 71 = Home
//224 72 = ↑ (up arrow)
//224 73 = Page Up
//224 75 = ← (left arrow)
//224 77 = → (right arrow)
//224 79 = End
//224 80 = ↓ (down arrow)
//224 81 = Page Down
//224 82 = Insert
//224 83 = Delete
//224 133 = F11
//224 134 = F12


fn key_from_key_code(code: INT) -> Key {

    println!("code: {}", code);
    println!("up winapi: {}", winuser::VK_UP);

    match code {
//        59  => Key::F1,
//        60  => Key::F2,
//        61  => Key::F3,
//        62  => Key::F4,
//        63  => Key::F5,
//        64  => Key::F6,
//        65  => Key::F7,
//        66  => Key::F8,
//        67  => Key::F9,
//        68  => Key::F10,
        winuser::VK_LEFT => Key::ArrowLeft,
        winuser::VK_RIGHT => Key::ArrowRight,
        winuser::VK_UP => Key::ArrowUp,
        winuser::VK_DOWN => Key::ArrowDown,
        winuser::VK_RETURN => Key::Enter,
        winuser::VK_ESCAPE => Key::Escape,
        winuser::VK_BACK => Key::Char('\x08'),
        winuser::VK_TAB => Key::Char('\x09'),
        _ => Key::Unknown,
    }
}

extern "C" {
    fn _getwch() -> INT;
    fn _getwch_nolock() -> INT;
}