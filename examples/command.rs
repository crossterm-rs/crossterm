#![allow(dead_code)]

use std::io::{stdout, Write};

use crossterm::{
    execute, queue, Clear, ClearType, ExecutableCommand, Goto, Output, QueueableCommand, Result,
};

/// execute commands by using normal functions
fn execute_command_directly_using_functions() -> Result<()> {
    // single command
    stdout().execute(Output("Text1 ".to_string()))?;

    // multiple commands
    stdout()
        .execute(Output("Text2 ".to_string()))?
        .execute(Output("Text3 ".to_string()))?;

    Ok(())
}

/// execute commands by using macro's
fn execute_command_directly_using_macros() -> Result<()> {
    // single command
    execute!(stdout(), Output("Text1 ".to_string()))?;

    // multiple commands
    execute!(
        stdout(),
        Output("Text2 ".to_string()),
        Output("Text 3".to_string())
    )
}

/// queue commands without executing them directly by using normal functions
fn later_execution_command_using_functions() -> Result<()> {
    let mut sdout = stdout();

    // single command
    sdout.queue(Output("Text1 ".to_string()))?;

    // multiple commands
    sdout
        .queue(Clear(ClearType::All))?
        .queue(Goto(5, 5))?
        .queue(Output(
            "console cleared, and moved to coord X: 5 Y: 5 ".to_string(),
        ))?;

    ::std::thread::sleep(std::time::Duration::from_millis(2000));

    // when you call this all commands will be executed
    sdout.flush()?;

    Ok(())
}

/// queue commands without executing them directly by using macro's
fn later_execution_command_directly_using_macros() -> Result<()> {
    let mut stdout = stdout();

    // single command
    queue!(stdout, Output("Text1 ".to_string()))?;

    // multiple commands
    queue!(
        stdout,
        Clear(ClearType::All),
        Goto(5, 5),
        Output("console cleared, and moved to coord X: 5 Y: 5 ".to_string())
    )?;

    ::std::thread::sleep(std::time::Duration::from_millis(2000));

    // when you call this all commands will be executed
    stdout.flush()?;

    Ok(())
}

// cargo run --example command
fn main() -> Result<()> {
    later_execution_command_directly_using_macros()
    //    later_execution_command_using_functions()
    //    execute_command_directly_using_macros()
    //    execute_command_directly_using_functions()
}
