extern crate crossterm;

use crossterm::{
    execute, queue, Clear, ClearType, Color, Colorize, ExecutableCommand, Goto, Output,
    PrintStyledFont, QueueableCommand,
};
use std::fmt::Display;
use std::io::{stdout, Stdout, Write};

/// execute commands by using normal functions
fn execute_command_directly_using_functions() {
    // single command
    stdout().execute(Output("Text1 ".to_string()));

    // multiple commands
    stdout()
        .execute(Output("Text2 ".to_string()))
        .execute(Output("Text3 ".to_string()));
}

/// execute commands by using macro's
fn execute_command_directly_using_macros() {
    // single command
    execute!(stdout(), Output("Text1 ".to_string()));

    // multiple commands
    execute!(
        stdout(),
        Output("Text2 ".to_string()),
        Output("Text 3".to_string())
    );
}

/// queue commands without executing them directly by using normal functions
fn later_execution_command_using_functions() {
    let mut sdout = stdout();

    // single command
    sdout = sdout.queue(Output("Text1 ".to_string()));

    // multiple commands
    sdout = sdout
        .queue(Clear(ClearType::All))
        .queue(Goto(5, 5))
        .queue(Output(
            "console cleared, and moved to coord X: 5 Y: 5 ".to_string(),
        ));

    ::std::thread::sleep(std::time::Duration::from_millis(2000));

    // when you call this all commands will be executed
    sdout.flush();
}

/// queue commands without executing them directly by using macro's
fn later_execution_command_directly_using_macros() {
    let mut stdout = stdout();

    // single command
    queue!(stdout, Output("Text1 ".to_string()));

    // multiple commands
    queue!(
        stdout,
        Clear(ClearType::All),
        Goto(5, 5),
        Output("console cleared, and moved to coord X: 5 Y: 5 ".to_string())
    );

    ::std::thread::sleep(std::time::Duration::from_millis(2000));

    // when you call this all commands will be executed
    stdout.flush();
}

fn main() {}
