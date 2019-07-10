use crate::{schedule, supports_ansi, write_cout, ErrorKind, Result};
use std::fmt::Display;
use std::fmt::{self, Error, Formatter};
use std::intrinsics::write_bytes;
use std::io::Write;

pub trait Command {
    type AnsiType: Display;

    fn get_ansi_code(&self) -> Self::AnsiType;

    fn execute(&self) -> Result<()> {
        //        format!()!()()
        write_cout!(self.get_ansi_code());
        Ok(())
    }

    #[cfg(windows)] // Not sure if these are possible in traits.
    fn execute_winapi(&self) -> Result<()>;
}

impl<T> Display for Command<AnsiType = T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut Formatter) -> ::std::result::Result<(), Error> {
        match schedule!(f, self) {
            Err(ErrorKind::FmtError(e)) => Err(e),
            _ => Ok(()),
        }
    }
}

pub struct Output(pub String);

impl Command for Output {
    type AnsiType = String;

    fn get_ansi_code(&self) -> Self::AnsiType {
        return self.0.clone();
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        print!("{}", self.0);
        Ok(())
    }
}
