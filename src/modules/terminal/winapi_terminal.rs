//! This is an `WINAPI` specific implementation for terminal related action.
//! This module is used for non supporting `ANSI` windows terminals.
//!
//! Windows versions lower then windows 10 are not supporting ANSI codes. Those versions will use this implementation instead.

use super::*;

use kernel::windows_kernel::{Console, Coord, Cursor, Handle, ScreenBuffer, Size};

/// This struct is an winapi implementation for terminal related actions.
pub struct WinApiTerminal;

impl WinApiTerminal {
    pub fn new() -> WinApiTerminal {
        WinApiTerminal {}
    }
}

impl ITerminal for WinApiTerminal {
    fn clear(&self, clear_type: ClearType, _stdout: &Option<&Arc<TerminalOutput>>) {
        let screen_buffer = ScreenBuffer::current().unwrap();
        let csbi = screen_buffer.info().unwrap();

        let pos = csbi.cursor_pos();
        let buffer_size = csbi.buffer_size();
        let current_attribute = csbi.attributes();

        match clear_type {
            ClearType::All => {
                clear_entire_screen(buffer_size, current_attribute);
            }
            ClearType::FromCursorDown => clear_after_cursor(pos, buffer_size, current_attribute),
            ClearType::FromCursorUp => clear_before_cursor(pos, buffer_size, current_attribute),
            ClearType::CurrentLine => clear_current_line(pos, buffer_size, current_attribute),
            ClearType::UntilNewLine => clear_until_line(pos, buffer_size, current_attribute),
        };
    }

    fn terminal_size(&self, _stdout: &Option<&Arc<TerminalOutput>>) -> (u16, u16) {
        let csbi = ScreenBuffer::current().unwrap();
        csbi.info().unwrap().terminal_size().into()
    }

    fn scroll_up(&self, count: i16, _stdout: &Option<&Arc<TerminalOutput>>) {
        let csbi = ScreenBuffer::current().unwrap();
        let mut window = csbi.info().unwrap().terminal_window();

        // Check whether the window is too close to the screen buffer top
        if window.top >= count {
            window.top -= count; // move top down
            window.bottom = count; // move bottom down

            Console::new()
                .unwrap()
                .set_console_info(false, window)
                .unwrap();
        }
    }

    fn scroll_down(&self, count: i16, _stdout: &Option<&Arc<TerminalOutput>>) {
        let csbi = ScreenBuffer::current().unwrap();
        let mut window = csbi.info().unwrap().terminal_window();
        let buffer_size = csbi.info().unwrap().buffer_size();

        // Check whether the window is too close to the screen buffer top
        if window.bottom < buffer_size.height - count {
            window.top += count; // move top down
            window.bottom += count; // move bottom down

            Console::new()
                .unwrap()
                .set_console_info(false, window)
                .unwrap();
        }
    }

    /// Set the current terminal size
    fn set_size(&self, width: i16, height: i16, _stdout: &Option<&Arc<TerminalOutput>>) {
        if width <= 0 {
            panic!("Cannot set the terminal width lower than 1");
        }

        if height <= 0 {
            panic!("Cannot set the terminal height lower then 1")
        }

        // Get the position of the current console window
        let screen_buffer = ScreenBuffer::current().unwrap();
        let console = Console::from(**screen_buffer.get_handle());
        let csbi = screen_buffer.info().unwrap();

        let current_size = csbi.buffer_size();
        let window = csbi.terminal_window();

        let mut new_size = Size::new(current_size.width, current_size.height);

        // If the buffer is smaller than this new window size, resize the
        // buffer to be large enough.  Include window position.
        let mut resize_buffer = false;

        if current_size.width < window.left + width {
            if window.left >= i16::max_value() - width {
                panic!("Argument out of range when setting terminal width.");
            }

            new_size.width = window.left + width;
            resize_buffer = true;
        }
        if current_size.height < window.top + height {
            if window.top >= i16::max_value() - height {
                panic!("Argument out of range when setting terminal height");
            }

            new_size.height = window.top + height;
            resize_buffer = true;
        }

        if resize_buffer {
            if let Err(_) = screen_buffer.set_size(new_size.width, new_size.height) {
                panic!("Something went wrong when setting screen buffer size.");
            }
        }

        let mut window = window.clone();
        // Preserve the position, but change the size.
        window.bottom = window.top + height;
        window.right = window.left + width;
        console.set_console_info(true, window).unwrap();

        // If we resized the buffer, un-resize it.
        if resize_buffer {
            if let Err(_) = screen_buffer.set_size(current_size.width, current_size.height) {
                panic!("Something went wrong when setting screen buffer size.");
            }
        }

        let bounds = console.largest_window_size();

        if width > bounds.x {
            panic!(
                "Argument width: {} out of range when setting terminal width.",
                width
            );
        }
        if height > bounds.y {
            panic!(
                "Argument height: {} out of range when setting terminal height",
                height
            );
        }
    }

    fn exit(&self, stdout: &Option<&Arc<TerminalOutput>>) {
        if let Some(output) = stdout {
            // drop the screen with the current stdout. This will make sure when in raw mode this will be disabled first.
            let mut screen = Screen::from(output.to_owned().clone());
            drop(screen);
            functions::exit_terminal();
        }
    }
}

pub fn clear_after_cursor(location: Coord, buffer_size: Size, current_attribute: u16) {
    let (mut x, mut y) = (location.x, location.y);

    // if cursor position is at the outer right position
    if x as i16 > buffer_size.width {
        y += 1;
        x = 0;
    }

    // location where to start clearing
    let start_location = Coord::new(x, y);

    // get sum cells before cursor
    let cells_to_write = buffer_size.width as u32 * buffer_size.height as u32;

    clear(start_location, cells_to_write, current_attribute);
}

pub fn clear_before_cursor(location: Coord, buffer_size: Size, current_attribute: u16) {
    let (xpos, ypos) = (location.x, location.y);

    // one cell after cursor position
    let x = 0;
    // one at row of cursor position
    let y = 0;

    // location where to start clearing
    let start_location = Coord::new(x, y);

    // get sum cells before cursor
    let cells_to_write = (buffer_size.width as u32 * ypos as u32) + (xpos as u32 + 1);

    // clear everything before cursor position
    clear(start_location, cells_to_write, current_attribute);
}

pub fn clear_entire_screen(buffer_size: Size, current_attribute: u16) {
    // get sum cells before cursor
    let cells_to_write = buffer_size.width as u32 * buffer_size.height as u32;

    // location where to start clearing
    let start_location = Coord::new(0, 0);

    // clear the entire screen
    clear(start_location, cells_to_write, current_attribute);

    // put the cursor back at cell 0,0
    let cursor = Cursor::new().unwrap();
    cursor.goto(0, 0);
}

pub fn clear_current_line(location: Coord, buffer_size: Size, current_attribute: u16) {
    // location where to start clearing
    let start_location = Coord::new(0, location.y);

    // get sum cells before cursor
    let cells_to_write = buffer_size.width as u32;

    // clear the whole current line
    clear(start_location, cells_to_write, current_attribute);

    // put the cursor back at cell 1 on current row
    let cursor = Cursor::new().unwrap();
    cursor.goto(0, location.y);
}

pub fn clear_until_line(location: Coord, buffer_size: Size, current_attribute: u16) {
    let (x, y) = (location.x, location.y);

    // location where to start clearing
    let start_location = Coord::new(x, y);

    // get sum cells before cursor
    let cells_to_write = (buffer_size.width - x as i16) as u32;

    // clear until the current line
    clear(start_location, cells_to_write, current_attribute);

    // put the cursor back at original cursor position before we did the clearing
    let cursor = Cursor::new().unwrap();
    cursor.goto(x, y);
}

fn clear(start_location: Coord, cells_to_write: u32, current_attribute: u16) {
    let console = Console::from(Handle::current_out_handle().unwrap());
    let _ = console
        .fill_whit_character(start_location, cells_to_write, ' ')
        .unwrap();
    console
        .fill_whit_attribute(start_location, cells_to_write, current_attribute)
        .unwrap();
}
