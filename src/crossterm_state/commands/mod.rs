use rand;

#[cfg(unix)]
pub mod unix_command;
#[cfg(windows)]
pub mod win_commands;

pub mod shared_commands;

#[cfg(unix)]
pub use self::unix_command::*;

#[cfg(windows)]
pub use self::win_commands::*;

use super::Context;

pub enum CommandType
{
    Unix,
    Windows,
}

pub trait ICommand
{
    fn new() -> Box<Self> where Self: Sized;
    fn execute(&mut self) -> bool;
    fn undo(&mut self) -> bool;
}

pub trait IContextCommand
{
    fn new(context: &mut Context) -> (Box<Self>, i16) where Self: Sized;
    fn execute(&mut self) -> bool;
    fn undo(&mut self) -> bool;
}


fn generate_key() -> i16 {
//        let mut rng = rand::thread_rng();
//        if rng.gen() { // random bool
//            println!("i32: {}, u32: {}", rng.gen::<i32>(), rng.gen::<u32>())
//        }
    rand::random::<i16>()
}