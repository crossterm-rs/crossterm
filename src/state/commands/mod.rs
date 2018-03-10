//! In this module I make use of the command pattern to wrap state changes.
//!
//! The `command pattern` is an OOP concept but what it does is very handy.
//! Shortly said what this pattern can do is having an command (struct) like `EnableRawModeCommand` this command has two methods one to `execute` that command and one to `undo`.
//! Every time you preform some action you can push it into an list and at the end when you want to revert all the commands you have executed than you can loop true that loop true that list and `undo` the actions.
//!
//! So where do whe use the `Commands` for? This is so that we can push all or terminal state changes into list.
//! When we do not need those changes we can revert all the changes by looping true the list and undo all the action.
//!
//! See the `Context` struct where we store the commands for more info.

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

/// This command can be used for simple commands witch just have an `undo()` and an `execute()`
pub trait ICommand
{
    fn new() -> Box<Self> where Self: Sized;
    fn execute(&mut self) -> bool;
    fn undo(&mut self) -> bool;
}

/// This command is used for complex commands whits change the terminal state.
/// By passing an `Context` instance this command will register it self to notify the terminal state change.
pub trait IContextCommand
{
    fn new(context: &mut Context) -> (Box<Self>, i16) where Self: Sized;
    fn execute(&mut self) -> bool;
    fn undo(&mut self) -> bool;
}

/// This generates an random key for the `ContextCommand`.
/// So that we can identify the `ContextCommand` in an list of commands.
fn generate_key() -> i16 {
    rand::random::<i16>()
}