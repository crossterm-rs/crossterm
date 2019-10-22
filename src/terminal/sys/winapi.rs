use crossterm_winapi::ScreenBuffer;

use crate::utils::Result;

pub(crate) fn exit() {
    ::std::process::exit(256);
}

pub(crate) fn get_terminal_size() -> Result<(u16, u16)> {
    let terminal_size = ScreenBuffer::current()?.info()?.terminal_size();
    // windows starts counting at 0, unix at 1, add one to replicated unix behaviour.
    Ok((
        (terminal_size.width + 1) as u16,
        (terminal_size.height + 1) as u16,
    ))
}
