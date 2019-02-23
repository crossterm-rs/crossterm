## Upgrade crossterm to 0.6.0
#### Namespace refactor
Some namespaces have been changed. All types of could be used directly by `use crossterm::*;` instead of having to go to a specific module for importing a type.

_**old**_
```rust
crossterm::input::{TerminalInput, ...};
crossterm::style::style;
crossterm::cursor::*;
crossterm::teriminal::{Terminal, ...};
```
_**new**_
```rust
crossterm::{TerminalInput, style, TerminalColor, StyledObject, Terminal, ...}
```

#### Removed unclear methods

```rust
let screen = Screen::new();

// the below methods are not available anymore
cursor::from_screen(&screen);
input::from_screen(&screen);
terminal::from_screen(&screen);
color::from_screen(&screen);
```

Instead of this you should make use of `Crossterm` type

```rust
let screen = Screen::new();
let crossterm = Crossterm::from_screen(screen);
let cursor = crossterm.cursor();
....
```

Or you can use the `from_output()`; this takes in the output stored in `Screen`

```rust
let screen = Screen::new();
let cursor = TerminalCursor::from_output(&screen.stdout);
let input = TerminalInput::from_output(&screen.stdout);
```
 
#### Wait until takes in a self now.
_**old**_
```rust
TerminalInput::wait_until(KeyEvent::OnKeyPress(b'x'));
```
_**new**_
```rust
let terminal_input = TerminalInput::new();
terminal_input.wait_until(KeyEvent::OnKeyPress(b'x'));
```

## Upgrade crossterm to 0.5.0

***WARNING*** 

I workded on making the user API more convenient therefore I had to make some changes to the user API. The problem with `0.4` is that you need to pass a `Screen` to the modules: `cursor(), color(), terminal()`.

In the new situation you only have to do this when working with raw or alternate screen. When you just want to perform actions like styling on the main screen you don't have to to pass in the `Screen` any more. This will look like the following:

#### 1. Remove `Screen` from the function calls: `cursor(), color(), terminal(), input()`

_**old**_
```
let screen = Screen::default();

let color = color(&screen);
let cursor = cursor(&screen);
let input = input(&screen);
let terminal = terminal(&screen);
let crossterm = Crossterm::new(&screen);
let terminal = Terminal::new(&screen.stdout);
let cursor = TerminalCursor::new(&screen.stdout);
let color = TerminalColor::new(&screen.stdout);
let input = TerminalInput::new(&screen.stdout);
```
_**new**_
```
let color = color();
let cursor = cursor();
let input = input();
let terminal = terminal();
let crossterm = Crossterm::new();
let terminal = Terminal::new();
let cursor = TerminalCursor::new();
let color = TerminalColor::new();
let input = TerminalInput::new();
```

#### 2. When working with alternate or raw screen. 

When working with alternate and or raw screen you still have to provide a `Screen` instance since information of the alternate and raw screen is stored in it. When doing this, the actions of the module will be perfomed on the alternate screen. If you don't do this your actions will executed at the main screen.

```
use crossterm::cursor;
use crossterm::color;
use crossterm::input;
use crossterm::terminal;

let screen = Screen::default();

if let Ok(alternate) = screen.enable_alternate_modes(false) {
    let screen = alternate.screen;
    let color = color::from_screen(&screen);
    let cursor = cursor::from_screen(&screen);
    let input = input::from_screen(&screen);
    let terminal = terminal::from_screen(&screen);
    let crossterm = Crossterm::from_screen(&screen);
    
    let terminal = Terminal::from_output(&screen.stdout);
    let cursor = TerminalCursor::from_output(&screen.stdout);
    let color = TerminalColor::from_output(&screen.stdout);
    let input = TerminalInput::from_output(&screen.stdout);
}

```

## Upgrade crossterm to 0.4.0

***WARNING*** 

This new version contains some cool features but to get those features working I needed to add some user API braking changes. 
I really did not want to do this but it had to be done for some reasons.

#### 1. You need to pass a reference to an `Screen` to the modules: `cursor(), color(), terminal()`

_**old**_
```
use crossterm::terminal::terminal;
use crossterm::cursor::cursor;
use crossterm::style::color;

use crossterm::Context;

let context: Rc<Context> = Context::new();

let cursor = cursor(&context);
let terminal = terminal(&context);
let color = color(&context);
```
_**new**_
```
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

// `false` specifies whether the alternate screen should be in raw modes.
if let Ok(alternate) = screen.enable_alternate_modes(false)
{
    let cursor = cursor(&alternate.screen);
}
```

#### Other
- ::crossterm::Crossterm::write() is gone.
- ::crossterm::Crossterm::flush() is gone.
- Context type is removed
- StateManager is removed
- ScreenManager type is renamed to Stdout.

## Upgrade crossterm 0.2.1 to 0.3.0

***WARNING*** 

This new version contains some cool features but to get those features working I needed to add some user API braking changes. 
I really did not want to do this but it had to be done for some reasons. Check `LINK (updates crossterm version)` for more info about why. 

First thing  that has changed is that you need to pass a reference to an `Rc<Context>` to the modules: `cursor(), color(), terminal()`

_**old**_
```
use crossterm::terminal::terminal;
use crossterm::cursor::cursor;
use crossterm::style::color;

/// Old situation
let cursor = cursor();
let terminal = terminal();
let color = color();
```
_**new**_
```
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
