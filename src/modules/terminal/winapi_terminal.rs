//! This is an `WINAPI` specific implementation for terminal related action.
//! This module is used for non supporting `ANSI` windows terminals.
//!
//! Windows versions lower then windows 10 are not supporting ANSI codes. Those versions will use this implementation instead.

use super::*;
use cursor::TerminalCursor;

use kernel::windows_kernel::{csbi, kernel, terminal, writing};
use winapi::um::wincon::{CONSOLE_SCREEN_BUFFER_INFO, COORD, SMALL_RECT};

/// This struct is an winapi implementation for terminal related actions.
pub struct WinApiTerminal;

impl WinApiTerminal {
    pub fn new() -> WinApiTerminal {
        WinApiTerminal {}
    }
}

impl ITerminal for WinApiTerminal {
    fn clear(&self, clear_type: ClearType, stdout: &Option<&Arc<TerminalOutput>>) {
        let csbi = csbi::get_csbi().unwrap();
        let pos = TerminalCursor::new().pos();

        match clear_type {
            ClearType::All => {
                clear_entire_screen(csbi, stdout);
            }
            ClearType::FromCursorDown => clear_after_cursor(pos, csbi, stdout),
            ClearType::FromCursorUp => clear_before_cursor(pos, csbi, stdout),
            ClearType::CurrentLine => clear_current_line(pos, csbi, stdout),
            ClearType::UntilNewLine => clear_until_line(pos, csbi, stdout),
        };
    }

    fn terminal_size(&self, _stdout: &Option<&Arc<TerminalOutput>>) -> (u16, u16) {
        terminal::terminal_size()
    }

    fn scroll_up(&self, count: i16, _stdout: &Option<&Arc<TerminalOutput>>) {
        let csbi = csbi::get_csbi().unwrap();

        // Set srctWindow to the current window size and location.
        let mut srct_window = csbi.srWindow;

        // Check whether the window is too close to the screen buffer top
        if srct_window.Top >= count {
            srct_window.Top -= count; // move top down
            srct_window.Bottom = count; // move bottom down

            let success = kernel::set_console_info(false, &mut srct_window);
            if success {
                panic!("Something went wrong when scrolling down");
            }
        }
    }

    fn scroll_down(&self, count: i16, _stdout: &Option<&Arc<TerminalOutput>>) {
        let csbi = csbi::get_csbi().unwrap();
        // Set srctWindow to the current window size and location.
        let mut srct_window = csbi.srWindow;

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
    fn set_size(&self, width: i16, height: i16, _stdout: &Option<&Arc<TerminalOutput>>) {
        if width <= 0 {
            panic!("Cannot set the terminal width lower than 1");
        }

        if height <= 0 {
            panic!("Cannot set the terminal height lower then 1")
        }

        // Get the position of the current console window
        let csbi = csbi::get_csbi().unwrap();
        let mut success = false;

        // If the buffer is smaller than this new window size, resize the
        // buffer to be large enough.  Include window position.
        let mut resize_buffer = false;
        let mut size = COORD {
            X: csbi.dwSize.X,
            Y: csbi.dwSize.Y,
        };

        if csbi.dwSize.X < csbi.srWindow.Left + width {
            if csbi.srWindow.Left >= i16::max_value() - width {
                panic!("Argument out of range when setting terminal width.");
            }

            size.X = csbi.srWindow.Left + width;
            resize_buffer = true;
        }
        if csbi.dwSize.Y < csbi.srWindow.Top + height {
            if csbi.srWindow.Top >= i16::max_value() - height {
                panic!("Argument out of range when setting terminal height");
            }

            size.Y = csbi.srWindow.Top + height;
            resize_buffer = true;
        }

        if resize_buffer {
            success = csbi::set_console_screen_buffer_size(size);

            if !success {
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
                csbi::set_console_screen_buffer_size(csbi.dwSize);
            }

            let bounds = kernel::get_largest_console_window_size();

            if width > bounds.X {
                panic!(
                    "Argument width: {} out of range when setting terminal width.",
                    width
                );
            }
            if height > bounds.Y {
                panic!(
                    "Argument height: {} out of range when setting terminal height",
                    height
                );
            }
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

pub fn clear_after_cursor(
    pos: (u16, u16),
    csbi: CONSOLE_SCREEN_BUFFER_INFO,
    _stdout: &Option<&Arc<TerminalOutput>>,
) {
    let (mut x, mut y) = pos;

    // if cursor position is at the outer right position
    if x as i16 > csbi.dwSize.X {
        y += 1;
        x = 0;
    }

    // location where to start clearing
    let start_location = COORD {
        X: x as i16,
        Y: y as i16,
    };
    // get sum cells before cursor
    let cells_to_write = csbi.dwSize.X as u32 * csbi.dwSize.Y as u32;

    clear(start_location, cells_to_write);
}

pub fn clear_before_cursor(
    pos: (u16, u16),
    csbi: CONSOLE_SCREEN_BUFFER_INFO,
    _stdout: &Option<&Arc<TerminalOutput>>,
) {
    let (xpos, ypos) = pos;

    // one cell after cursor position
    let x = 0;
    // one at row of cursor position
    let y = 0;

    // location where to start clearing
    let start_location = COORD {
        X: x as i16,
        Y: y as i16,
    };
    // get sum cells before cursor
    let cells_to_write = (csbi.dwSize.X as u32 * ypos as u32) + (xpos as u32 + 1);

    clear(start_location, cells_to_write);
}

pub fn clear_entire_screen(
    csbi: CONSOLE_SCREEN_BUFFER_INFO,
    stdout: &Option<&Arc<TerminalOutput>>,
) {
    // position x at start
    let x = 0;
    // position y at start
    let y = 0;

    // location where to start clearing
    let start_location = COORD {
        X: x as i16,
        Y: y as i16,
    };
    // get sum cells before cursor

    let cells_to_write = csbi.dwSize.X as u32 * csbi.dwSize.Y as u32;

    clear(start_location, cells_to_write);

    match stdout {
        Some(ref output) => {
            // put the cursor back at (0, 0)
            TerminalCursor::from_output(output).goto(0, 0);
        }
        None => {
            // put the cursor back at (0, 0)
            TerminalCursor::new().goto(0, 0);
        }
    }
}

pub fn clear_current_line(
    pos: (u16, u16),
    csbi: CONSOLE_SCREEN_BUFFER_INFO,
    stdout: &Option<&Arc<TerminalOutput>>,
) {
    // position x at start
    let x = 0;
    // position y at start
    let y = pos.1;

    // location where to start clearing
    let start_location = COORD {
        X: x as i16,
        Y: y as i16,
    };
    // get sum cells before cursor

    let cells_to_write = csbi.dwSize.X as u32;

    clear(start_location, cells_to_write);

    // put the cursor back at 1 cell on current row
    match stdout {
        Some(ref output) => {
            // put the cursor back at (0, 0)
            TerminalCursor::from_output(output).goto(0, y);
        }
        None => {
            // put the cursor back at (0, 0)
            TerminalCursor::new().goto(0, y);
        }
    }
}

pub fn clear_until_line(
    pos: (u16, u16),
    csbi: CONSOLE_SCREEN_BUFFER_INFO,
    stdout: &Option<&Arc<TerminalOutput>>,
) {
    let (x, y) = pos;

    // location where to start clearing
    let start_location = COORD {
        X: x as i16,
        Y: y as i16,
    };
    // get sum cells before cursor
    let cells_to_write = (csbi.dwSize.X - x as i16) as u32;

    clear(start_location, cells_to_write);

    // put the cursor back at original cursor position
    match stdout {
        Some(ref output) => {
            // put the cursor back at (0, 0)
            TerminalCursor::from_output(output).goto(x, y);
        }
        None => {
            // put the cursor back at (0, 0)
            TerminalCursor::new().goto(x, y);
        }
    }
}

fn clear(start_location: COORD, cells_to_write: u32) {
    let mut cells_written = 0;
    let mut success = false;

    success =
        writing::fill_console_output_character(&mut cells_written, start_location, cells_to_write);

    if !success {
        panic!("Could not clear screen after cursor");
    }

    cells_written = 0;

    success =
        writing::fill_console_output_attribute(&mut cells_written, start_location, cells_to_write);

    if !success {
        panic!("Could not reset attributes after cursor");
    }
}
