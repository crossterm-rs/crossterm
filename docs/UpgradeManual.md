## Upgrade crossterm to 0.4.0

***WARNING*** 

This new version contains some cool features but to get those features working I needed to add some user API braking changes. 
I really did not want to do this but it had to be done for some reasons.

#### 1. You need to pass a reference to an `Screen` to the modules: `cursor(), color(), terminal()`

```
use crossterm::terminal::terminal;
use crossterm::cursor::cursor;
use crossterm::style::color;

/// Old situation
use crossterm::Context;

let context: Rc<Context> = Context::new();

let cursor = cursor(&context);
let terminal = terminal(&context);
let color = color(&context);

/// new situation 
use crossterm::Screen;

let screen: Screen = Screen::default();

let cursor = cursor(&screen);
let terminal = terminal(&screen);
let color = color(&screen);
```

#### 2. The `::crossterm::Crossterm::paint()`  function does not exits anymore like before:

Instead you could do it like the following:

```
use crossterm::Crossterm;
use crossterm::style::{Color, input, style};

// 1: use the `Crossterm` type
let crossterm = Crossterm::new();
let styled_object = crossterm.style("Red font on Black background").with(Color::Red).on(Color::Black);
styled_object.paint(&screen);

// 2: use the `Terminal` type
let styled_object = style("Red font on Black background").with(Color::Red).on(Color::Black);
styled_object.paint(&screen);

```

#### 3. Alternate Screen and Raw Screen 
Also I have changed how the alternate and raw screen are working.

```
// could not be used any more
::crossterm::AlternateScreen::from();
// cannot put any Write into raw mode.
::std::io::Write::into_raw_mode()
```

This now should be done with the `Screen` type like:

```
use crossterm::Screen;
use crossterm::cursor::cursor;

// this will create a default screen.
let screen = Screen::default();

// this will create a new screen with raw modes enabled.
let screen = Screen::new(true);

// false specifies whether the alternate screen should be in raw modes.
if let Ok(alternate) = screen.enable_alternate_modes(false)
{
    let cursor = cursor(&alternate.screen);
}
```

#### Other
- ::crossterm::Crossterm::write() is gone.
- Context type is removed
- StateManager is removed
- ScreenManager type is renamed to Stdout.

## Upgrade crossterm 0.2.1 to 0.3.0

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
