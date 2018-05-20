//! This is an `WINAPI` specific implementation for terminal related action.
//! This module is used for windows 10 terminals and unix terminals by default.

use {Construct};
use cursor::cursor;
use super::{ClearType, ITerminal};
use winapi::um::wincon::{SMALL_RECT, COORD, CONSOLE_SCREEN_BUFFER_INFO,};
use kernel::windows_kernel::{kernel, terminal};

/// This struct is an windows implementation for terminal related actions.
pub struct WinApiTerminal;

impl Construct for WinApiTerminal {
    fn new() -> Box<WinApiTerminal> {
        Box::from(WinApiTerminal {})
    }
}

impl ITerminal for WinApiTerminal {

    fn clear(&self, clear_type: ClearType) {
        let csbi = kernel::get_console_screen_buffer_info();
        let pos = cursor().pos();

        match clear_type
        {
            ClearType::All => clear_entire_screen(csbi),
            ClearType::FromCursorDown => clear_after_cursor(pos,csbi),
            ClearType::FromCursorUp => clear_before_cursor(pos, csbi),
            ClearType::CurrentLine => clear_current_line(pos, csbi),
            ClearType::UntilNewLine => clear_until_line(pos, csbi),
        };
    }

    fn terminal_size(&self) -> (u16, u16) {
       terminal::terminal_size()
    }

    fn scroll_up(&self, count: i16) {
        // yet to be inplemented
    }

    fn scroll_down(&self, count: i16) {
        let csbi = kernel::get_console_screen_buffer_info();
        let mut srct_window;

        // Set srctWindow to the current window size and location.
        srct_window = csbi.srWindow;

        // Check whether the window is too close to the screen buffer top
        if srct_window.Bottom < csbi.dwSize.Y - count {
            srct_window.Top += count; // move top down
            srct_window.Bottom += count; // move bottom down

            let success = kernel::set_console_info(true, &mut srct_window);
            if success {
                panic!("Something went wrong when scrolling down");
            }
        }
    }

    /// Set the current terminal size
    fn set_size(&self, width: i16, height: i16) {
        if width <= 0
            {
                panic!("Cannot set the terminal width lower than 1");
            }

        if height <= 0
            {
                panic!("Cannot set the terminal height lower then 1")
            }

        // Get the position of the current console window
        let csbi = kernel::get_console_screen_buffer_info();
        let mut success = false;

        // If the buffer is smaller than this new window size, resize the
        // buffer to be large enough.  Include window position.
        let mut resize_buffer = false;
        let mut size = COORD { X: csbi.dwSize.X, Y: csbi.dwSize.Y };

        if csbi.dwSize.X < csbi.srWindow.Left + width
            {
                if csbi.srWindow.Left >= i16::max_value() - width
                    {
                        panic!("Argument out of range when setting terminal width.");
                    }

                size.X = csbi.srWindow.Left + width;
                resize_buffer = true;
            }
        if csbi.dwSize.Y < csbi.srWindow.Top + height {
            if csbi.srWindow.Top >= i16::max_value() - height
                {
                    panic!("Argument out of range when setting terminal height");
                }

            size.Y = csbi.srWindow.Top + height;
            resize_buffer = true;
        }

        if resize_buffer {
            success = kernel::set_console_screen_buffer_size(size);

            if !success
                {
                    panic!("Something went wrong when setting screen buffer size.");
                }
        }

        let mut fsr_window: SMALL_RECT = csbi.srWindow;
        // Preserve the position, but change the size.
        fsr_window.Bottom = fsr_window.Top + height;
        fsr_window.Right = fsr_window.Left + width;

        let success = kernel::set_console_info(true, &fsr_window);

        if success {
            // If we resized the buffer, un-resize it.
            if resize_buffer {
                kernel::set_console_screen_buffer_size(csbi.dwSize);
            }

            let bounds = kernel::get_largest_console_window_size();

            if width > bounds.X
            {
                panic!("Argument width: {} out of range when setting terminal width.", width);
            }
            if height > bounds.Y
            {
                panic!("Argument height: {} out of range when setting terminal height", height);
            }
        }
    }
}

pub fn clear_after_cursor(pos: (u16,u16), csbi: CONSOLE_SCREEN_BUFFER_INFO) {
    let (mut x,mut y) = pos;

    // if cursor position is at the outer right position
    if x as i16 > csbi.dwSize.X
    {
        y += 1;
        x = 0;
    }

    // location where to start clearing
    let start_location = COORD { X: x  as i16, Y: y  as i16};
    // get sum cells before cursor
    let cells_to_write = csbi.dwSize.X as u32  * csbi.dwSize.Y as u32;

    clear(start_location,cells_to_write);
}

pub fn clear_before_cursor(pos: (u16,u16), csbi: CONSOLE_SCREEN_BUFFER_INFO) {
    let (xpos,ypos) = pos;

    // one cell after cursor position
    let x = 0;
    // one at row of cursor position
    let y = 0;

    // location where to start clearing
    let start_location = COORD { X: x  as i16, Y: y  as i16};
    // get sum cells before cursor
    let cells_to_write = (csbi.dwSize.X as u32  * ypos as u32) + (xpos as u32 + 1);

    clear(start_location, cells_to_write);
}

pub fn clear_entire_screen(csbi: CONSOLE_SCREEN_BUFFER_INFO) {
    // position x at start
    let x = 0;
    // position y at start
    let y = 0;

    // location where to start clearing
    let start_location = COORD { X: x  as i16, Y: y  as i16};
    // get sum cells before cursor

    let cells_to_write = csbi.dwSize.X as u32 * csbi.dwSize.Y as u32;

    clear( start_location, cells_to_write);

    // put the cursor back at (0, 0)
    cursor().goto(0, 0);
}

pub fn clear_current_line(pos: (u16,u16), csbi: CONSOLE_SCREEN_BUFFER_INFO)
{
    // position x at start
    let x = 0;
    // position y at start
    let y = pos.1;

    // location where to start clearing
    let start_location = COORD { X: x  as i16, Y: y  as i16};
    // get sum cells before cursor

    let cells_to_write = csbi.dwSize.X as u32;

    clear(start_location, cells_to_write);

    // put the cursor back at 1 cell on current row
    cursor().goto(0, y);
}

pub fn clear_until_line(pos: (u16,u16), csbi: CONSOLE_SCREEN_BUFFER_INFO)
{
    let (x,y) = pos;

    // location where to start clearing
    let start_location = COORD { X: x  as i16, Y: y  as i16};
    // get sum cells before cursor
    let cells_to_write = (csbi.dwSize.X - x  as i16) as u32;

    clear(start_location, cells_to_write);

    // put the cursor back at original cursor position
    cursor().goto(x,y);
}

fn clear(
    start_loaction: COORD,
    cells_to_write: u32
) {
    let mut cells_written = 0;
    let mut success = false;

    success = kernel::fill_console_output_character(&mut cells_written, start_loaction, cells_to_write);

    if !success
    {
        panic!("Could not clear screen after cursor");
    }

    cells_written = 0;

    success = kernel::fill_console_output_attribute(&mut cells_written, start_loaction, cells_to_write);

    if !success {
        panic!("Couldnot reset attributes after cursor");
    }
}