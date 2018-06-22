//! This module contains the commands that can be used for windows systems.

use super::IStateCommand;
use { StateManager, Context };

use kernel::windows_kernel::{kernel, ansi_support};
use winapi::shared::minwindef::DWORD;
use winapi::um::wincon;
use winapi::um::wincon::{ENABLE_VIRTUAL_TERMINAL_PROCESSING ,SMALL_RECT, COORD, CHAR_INFO};
use std::mem;

use std::rc::Rc;
use std::sync::Mutex;

/// This command is used for enabling and disabling ANSI code support for windows systems,
/// For more info check: https://docs.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences.
#[derive(Clone, Copy)]
pub struct EnableAnsiCommand
{
    mask:  DWORD,
}

impl EnableAnsiCommand
{
    pub fn new() -> Box<EnableAnsiCommand> {
        let command = EnableAnsiCommand { mask: ENABLE_VIRTUAL_TERMINAL_PROCESSING };
        Box::from(command)
    }
}


impl IStateCommand for EnableAnsiCommand
{
    fn execute(&mut self,) -> bool
    {
        // we need to check whether we tried to enable ansi before. If we have we can just return if that had succeeded.
        if ansi_support::has_been_tried_to_enable_ansi() && ansi_support::ansi_enabled()
            {
                return ansi_support::windows_supportable();
            } else {
            let output_handle = kernel::get_output_handle();

            let mut dw_mode: DWORD = 0;
            if !kernel::get_console_mode(&output_handle, &mut dw_mode)
                {
                    return false;
                }

            dw_mode |= self.mask;
            if !kernel::set_console_mode(&output_handle, dw_mode)
                {
                    return false;
                }

            return true;
        }
    }

    fn undo(&mut self) -> bool
    {
        if ansi_support::ansi_enabled()
        {
            let output_handle = kernel::get_output_handle();

            let mut dw_mode: DWORD = 0;
            if !kernel::get_console_mode(&output_handle, &mut dw_mode)
                {
                    return false;
                }

            dw_mode &= !self.mask;
            if !kernel::set_console_mode(&output_handle, dw_mode)
            {
                return false;
            }

            ansi_support::set_ansi_enabled(false);
        }
        return true;
    }
}

/// This command is used for enabling and disabling raw mode for windows systems.
/// For more info check: https://docs.microsoft.com/en-us/windows/console/high-level-console-modes.
#[derive(Clone, Copy)]
pub struct EnableRawModeCommand
{
    mask: DWORD,
    key: u16
}

impl EnableRawModeCommand
{
    pub fn new(state_manager: &Mutex<StateManager>) -> u16 {
        use self::wincon::{ENABLE_LINE_INPUT,ENABLE_PROCESSED_INPUT, ENABLE_ECHO_INPUT};

        let mut state = state_manager.lock().unwrap();
        {
            let key = state.get_changes_count();
            let command = EnableRawModeCommand { mask: ENABLE_LINE_INPUT | ENABLE_PROCESSED_INPUT  | ENABLE_ECHO_INPUT, key: key };
            state.register_change(Box::from(command), key);
            key
        }
    }
}

impl IStateCommand for EnableRawModeCommand
{
    fn execute(&mut self) -> bool
    {
        let input_handle = kernel::get_input_handle();

        let mut dw_mode: DWORD = 0;
        if !kernel::get_console_mode(&input_handle, &mut dw_mode)
        {
            return false;
        }

        let new_mode = dw_mode & !self.mask;

        if !kernel::set_console_mode(&input_handle, new_mode)
        {
            return false;
        }

        true
    }

    fn undo(&mut self) -> bool
    {
        let output_handle = kernel::get_output_handle();

        let mut dw_mode: DWORD = 0;
        if !kernel::get_console_mode(&output_handle, &mut dw_mode)
        {
            return false;
        }

        let new_mode = dw_mode | self.mask;

        if !kernel::set_console_mode(&output_handle, new_mode)
        {
            return false;
        }

        true
    }
}

/// This command is used for switching to alternate screen and back to main screen.
/// check https://docs.microsoft.com/en-us/windows/console/reading-and-writing-blocks-of-characters-and-attributes for more info
pub struct ToAlternateScreenBufferCommand
{
    context: Rc<Context>
}

impl ToAlternateScreenBufferCommand
{
    pub fn new(context: Rc<Context>) -> u16
    {
        let key = 2;

        let mut state = context.state_manager.lock().unwrap();
        {
            let command = ToAlternateScreenBufferCommand {context: context.clone()};

            state.register_change(Box::from(command), key);
            key
        }
    }
}

impl IStateCommand for ToAlternateScreenBufferCommand
{
    fn execute(&mut self) -> bool
    {
        use super::super::super::kernel::windows_kernel::ansi_support;
        use super::super::super::manager::WinApiScreenManager;

        let does_support = ansi_support::try_enable_ansi_support();
        let mut screen_manager = self.context.screen_manager.lock().unwrap();

        screen_manager.toggle_is_alternate_screen(true);

        let mut chi_buffer: [CHAR_INFO;160] = unsafe {mem::zeroed() };

        let handle = kernel::get_output_handle();

        // create a new screen buffer to copy to.
        let new_handle = kernel::create_console_screen_buffer();

        // Make the new screen buffer the active screen buffer.
        kernel::set_active_screen_buffer(new_handle);

        let b: &mut WinApiScreenManager = match screen_manager.as_any().downcast_mut::<WinApiScreenManager>() {
            Some(b) => { b },
            None => panic!("&a isn't a B!")
        };

        b.set_alternate_handle(new_handle);

        // Set the source rectangle.
        let mut srct_read_rect = SMALL_RECT
        {
            Top: 0,
            Left: 0 ,
            Bottom: 1,
            Right: 79,
        };

        // The temporary buffer size is 2 rows x 80 columns.
        let coord_buffer_size = COORD
        {
            X: 2,
            Y: 80
        };

        // The top left destination cell of the temporary buffer is
        // row 0, col 0.
        let coord_buffer_coord = COORD
        {
            X: 0,
            Y: 0,
        };

        // Copy the block from the screen buffer to the temp. buffer.
        kernel::read_console_output(&handle, &mut chi_buffer, coord_buffer_size, coord_buffer_coord, &mut srct_read_rect);

        // Set the destination rectangle.
        let mut srct_write_rect = SMALL_RECT
        {
            Top: 10,
            Left: 0,
            Bottom: 11,
            Right: 19,
        };

        // Copy from the temporary buffer to the new screen buffer.
        kernel::write_console_output(&new_handle, &mut chi_buffer, coord_buffer_size, coord_buffer_coord, &mut srct_write_rect);

        true
    }

    fn undo(&mut self) -> bool
    {
        let handle = kernel::get_output_handle();
        kernel::set_active_screen_buffer(handle);

        {
            let mut screen_manager = self.context.screen_manager.lock().unwrap();
            screen_manager.toggle_is_alternate_screen(false);
        }

        true
    }
}
