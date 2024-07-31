use std::io::Write;

use crossterm::{cursor, execute, style::Print, SynchronizedUpdate};

fn render_slowly<W>(w: &mut W) -> std::io::Result<()>
where
    W: Write,
{
    for i in 1..10 {
        execute!(w, Print(format!("{}", i)))?;
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
    Ok(())
}

fn test_slow_rendering<W>(w: &mut W) -> std::io::Result<()>
where
    W: Write,
{
    execute!(w, Print("Rendering without synchronized update:"))?;
    execute!(w, cursor::MoveToNextLine(1))?;
    std::thread::sleep(std::time::Duration::from_millis(50));
    render_slowly(w)?;

    execute!(w, cursor::MoveToNextLine(1))?;
    execute!(w, Print("Rendering with synchronized update:"))?;
    execute!(w, cursor::MoveToNextLine(1))?;
    std::thread::sleep(std::time::Duration::from_millis(50));
    w.sync_update(render_slowly)??;

    execute!(w, cursor::MoveToNextLine(1))?;
    Ok(())
}

pub fn run<W>(w: &mut W) -> std::io::Result<()>
where
    W: Write,
{
    run_tests!(w, test_slow_rendering,);
    Ok(())
}
