extern crate crossterm;

use crossterm::{execute, queue, Command, ExecutableCommand, QueueableCommand};
use std::fmt::Display;
use std::io::{stdout, Stdout, Write};

use crossterm::{Clear, ClearType, Goto, Output, PrintStyledFont};

fn execute_command_directly_using_functions() {
    // single command
    stdout().execute(Output("Text1 ".to_string()));

    // multiple commands
    stdout()
        .execute(Output("Text2 ".to_string()))
        .execute(Output("Text3 ".to_string()));
}

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

fn main() {
    //    later_execution_command_directly_using_macros();

    use crossterm::Colorize;
    //
    let mut stdout = stdout();

    execute!(stdout, Clear(ClearType::All));

    for y in 0..40 {
        for x in 0..150 {
            if (y == 0 || y == 40 - 1) || (x == 0 || x == 150 - 1) {
                queue!(stdout, Goto(x, y), PrintStyledFont("█".magenta()));
            }
        }
        stdout.flush();
    }

    //

    use crossterm::{Color, Colorize, PrintStyledFont};

    let mut stdout = stdout();

    stdout = stdout.execute(Clear(ClearType::All));

    for y in 0..40 {
        for x in 0..150 {
            if (y == 0 || y == 40 - 1) || (x == 0 || x == 150 - 1) {
                stdout = stdout
                    .queue(Goto(x, y))
                    .queue(PrintStyledFont("█".magenta()));
            }
            stdout.flush();
        }
    }
}
