//! This module contains all `unix` specific terminal related logic.

use std::io;

use super::super::error::{ErrorKind, Result};

pub fn wrap_with_result(result: i32) -> Result<bool> {
    if result == -1 {
        Err(ErrorKind::IoError(io::Error::last_os_error()))
    } else {
        Ok(true)
    }
}
