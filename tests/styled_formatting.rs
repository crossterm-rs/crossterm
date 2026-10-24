use crossterm::Command;
use crossterm::style::{PrintStyledContent, Stylize, force_color_output, style};

// Exercise ANSI formatting even when a Windows test runner has no console.
#[test]
fn styled_content_preserves_string_formatting() {
    temp_env::with_var("TERM", Some("xterm"), || {
        force_color_output(true);

        assert_eq!(
            format!("{:>20}", style("Hello, world!").red()),
            format!("\x1b[38;5;9m{:>20}\x1b[39m", "Hello, world!")
        );
        assert_eq!(
            format!("{:*^10.3}", style("héllo").red()),
            format!("\x1b[38;5;9m{:*^10.3}\x1b[39m", "héllo")
        );
        assert_eq!(format!("{:<8}", style("hello")), format!("{:<8}", "hello"));
    });
}

#[test]
fn styled_content_preserves_numeric_formatting() {
    temp_env::with_var("TERM", Some("xterm"), || {
        force_color_output(true);

        assert_eq!(
            format!("{:+08.2}", style(12.5).red()),
            format!("\x1b[38;5;9m{:+08.2}\x1b[39m", 12.5)
        );
        assert_eq!(
            format!("{:>10.1}", style(-12.5).bold()),
            format!("\x1b[1m{:>10.1}\x1b[0m", -12.5)
        );
    });
}

#[test]
fn styled_content_default_formatting_matches_command() {
    temp_env::with_var("TERM", Some("xterm"), || {
        force_color_output(true);

        let content = "hello".red().on_blue().bold();
        let mut output = String::new();
        PrintStyledContent(content).write_ansi(&mut output).unwrap();
        assert_eq!(format!("{content}"), output);
    });
}
