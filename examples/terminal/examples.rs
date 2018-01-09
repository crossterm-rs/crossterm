fn main()
{
    let mut terminal = terminal::get();

     // clear all cells in terminal.
    terminal.clear(ClearType::All);
    // clear all cells after the cursor position in terminal.
    terminal.clear(ClearType::AfterCursor);
    // clear all cells before cursor in terminal.
    terminal.clear(ClearType::BeforeCursor);
    // clear current line cells in terminal.
    terminal.clear(ClearType::CurrentLine);
    // clear all cells until new line in terminal.
    terminal.clear(ClearType::UntilNewLine);

    // get terminal size (x,y)
    let size = terminal.terminal_size();
    println!("{:?}", size);

     // scrolling in terminal
    terminal.scroll_up();
    terminal.scroll_down();

}