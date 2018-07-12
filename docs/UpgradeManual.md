## Upgrade crossterm 0.2.1 to 0.2.3

***WARNING*** 

This new version contains some cool features but to get those features working I needed to add some user API braking changes. 
I really did not want to do this but it had to be done for some reasons. Check `LINK (updates crossterm version)` for more info about why. 

First thing  that has changed is that you need to pass a reference to an `Rc<Context>` to the modules: `cursor(), color(), terminal()`

```

use crossterm::terminal::terminal;
use crossterm::cursor::cursor;
use crossterm::style::color;
/// Old situation
let cursor = cursor();
let terminal = terminal();
let color = color();

/// new situation 
use crossterm::Context;

let context: Rc<Context> = Context::new();

let cursor = cursor(&context);
let terminal = terminal(&context);
let color = color(&context);
```

Also the `::crossterm::style::paint()`  function does not exits anymore like before:

Instead you could do it like the following:

```
use crossterm::Crossterm;
use crossterm::style::Color;
use crossterm::terminal::terminal;

// 1: use the `Crossterm` type
let crossterm = Crossterm::new();
let mut color = crossterm.paint("Red on Blue").with(Color::Red).on(Color::Blue);

// 2: use the `Terminal` type
let context: Rc<Context> = Context::new();
let terminal = terminal(&context).paint("Red on Blue").with(Color::Red).on(Color::Blue);

```

And you do not need `mut` for a lot of function calls anymore. 

## Upgrade crossterm 0.2 to 0.2.1

Namespaces:
I have changed the namespaces. I found the namsespaces to long so I have shortened them like the following:

```
Old: crossterm::crossterm_style 
New: crossterm::style

Old: crossterm::crossterm_terminal
New: crossterm::terminal

Old: crossterm::crossterm_cursor 
New: crossterm::cursor

```

Method names that changed [Issue 4](https://github.com/TimonPost/crossterm/issues/4): 

```
Old:  ::crossterm::crossterm_cursor::get();
New:  ::crossterm::cursor::cursor();

Old:  ::crossterm::crossterm_terminal::get();
New:  ::crossterm::terminal::terminal();

Old:  ::crossterm::crossterm_style::color::get();
New:  ::crossterm::style::color::color();
```