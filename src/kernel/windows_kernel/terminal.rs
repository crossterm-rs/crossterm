use winapi;
use kernel32;
use super::{cursor, handle, kernel, Empty};

/// Get the terminal size (y,x)
pub fn terminal_size() -> Option<(u16, u16)> {
    let csbi = kernel::get_console_screen_buffer_info();

    Some((
        (csbi.srWindow.Bottom - csbi.srWindow.Top) as u16,
        (csbi.srWindow.Right - csbi.srWindow.Left) as u16,
    ))
}

/// Scroll down `n` rows
pub fn scroll_down(rows: i16) {
    let output_handle = handle::get_output_handle();
    let csbi = kernel::get_console_screen_buffer_info();
    let mut srct_window;

    // Set srctWindow to the current window size and location.
    srct_window = csbi.srWindow;

    // Check whether the window is too close to the screen buffer top
    if srct_window.Bottom < csbi.dwSize.Y - rows {
        srct_window.Top += rows; // move top down
        srct_window.Bottom += rows; // move bottom down

        unsafe {
            if kernel32::SetConsoleWindowInfo(output_handle, winapi::TRUE, &mut srct_window) != 1 {
                panic!("Something whent wrong when scrolling down");
            }
        }
    }
}

pub fn clear_after_cursor() {
    let output_handle = handle::get_output_handle();
    let csbi = kernel::get_console_screen_buffer_info();

    // one cell after cursor position
    let mut x = cursor::xpos() as i16 + 1;
    // one at row of cursor position
    let mut y = cursor::ypos() as i16;

    // if cursor position is at the outer right position
    if x > csbi.dwSize.X
    {
        y += 1;
        x = 0;
    }

    // location where to start clearing
    let start_loaction = winapi::COORD { X: x, Y: y };
    // get sum cells before cursor
    let cells_to_write = csbi.dwSize.X as u32  * csbi.dwSize.Y as u32;

    clear(output_handle, csbi, start_loaction,cells_to_write);
}

pub fn clear_before_cursor() {
    let output_handle = handle::get_output_handle();
    let csbi = kernel::get_console_screen_buffer_info();

    // one cell after cursor position
    let x = 0;
    // one at row of cursor position
    let y = 0;

    // location where to start clearing
    let start_loaction = winapi::COORD { X: x, Y: y };
     // get sum cells before cursor
    let cells_to_write = (csbi.dwSize.X as u32  * cursor::ypos() as u32) + (cursor::xpos() -1) as u32;

    // println!("{:?}", (csbi.dwSize.X as u32  * (cursor::ypos() - 1) as u32));
    clear(output_handle, csbi, start_loaction, cells_to_write);
}

pub fn clear_entire_screen() {
    let output_handle = handle::get_output_handle();
    let csbi = kernel::get_console_screen_buffer_info();

    // position x at start
    let x = 0;
    // position y at start
    let y = 0;

    // location where to start clearing
    let start_loaction = winapi::COORD { X: x, Y: y };
    // get sum cells before cursor
    
    let cells_to_write = csbi.dwSize.X as u32 * csbi.dwSize.Y as u32;

    clear(output_handle, csbi, start_loaction, cells_to_write);

    // put the cursor back at (0, 0)
    cursor::set(0, 0);
}

pub fn clear_current_line()
{
     let output_handle = handle::get_output_handle();
    let csbi = kernel::get_console_screen_buffer_info();

    // position x at start
    let x = 0;
    // position y at start
    let y = cursor::ypos();

    // location where to start clearing
    let start_loaction = winapi::COORD { X: x, Y: y };
    // get sum cells before cursor
    
    let cells_to_write = csbi.dwSize.X as u32;

    clear(output_handle, csbi, start_loaction, cells_to_write);

    // put the cursor back at (0, 0)
    cursor::set(x, y);
}

pub fn clear_until_line()
{
     let output_handle = handle::get_output_handle();
    let csbi = kernel::get_console_screen_buffer_info();

    // position x at start
    let x = cursor::xpos();
    // position y at start
    let y = cursor::ypos();

    // location where to start clearing
    let start_loaction = winapi::COORD { X: x -1, Y: y };
    // get sum cells before cursor    
    let cells_to_write = (csbi.dwSize.X - x) as u32 - 1;

    clear(output_handle, csbi, start_loaction, cells_to_write);
    print!("{:?}",start_loaction);
    // put the cursor back at (0, 0)
    cursor::set(x, y);
}

fn clear(
    handle: winapi::HANDLE,
    csbi: winapi::CONSOLE_SCREEN_BUFFER_INFO,
    start_loaction: winapi::COORD,
    cells_to_write: u32
) {
    let mut cells_written = 0;
    let mut success;

    unsafe {
        // fill the cetain cells in console with blanks
        success = kernel32::FillConsoleOutputCharacterA(
            handle,
            ' ' as i8,
            cells_to_write,
            start_loaction,
            &mut cells_written,
        );
    }

    if success == 0 {
        panic!("Couldnot clear screen after cursor");
    }

    cells_written = 0;

    unsafe {
        success = kernel32::FillConsoleOutputAttribute(
            handle,
            csbi.wAttributes,
            cells_to_write,
            start_loaction,
            &mut cells_written,
        );
    }

    if success == 0 {
        panic!("Couldnot reset attributes after cursor");
    }
}
