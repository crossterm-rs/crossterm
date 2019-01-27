# Example

_setup the basics_
```rust
extern crate crossterm;

use crossterm::{style, Color, Attribute};

fn main() {
    /* your code here */
}
```

Let's dive into styling with crossterm. The easiest way to do this is by making use of the `style()` function.

```rust
let styled_object = style("This is some text converted into a styled object");
```

The function `style()` takes in any type that implement `Display`
and returns a `StyledObject`. 
A `StyledObject` is just a wrapper crossterm uses to store the text and style together.

The above code will not do any coloring magic yet. Lets play around with some colors to see it in working.

## Coloring
```rust
let styled_object = style("'Red' text on 'White' background")
                            .with(Color::Red)
                            .on(Color::White);

println!("{}", styled_object);
```

With the function `with()` you can decide the foreground color and with the function `on()` you can decide the background color of the text.
Because `StyledObject` you got from `style()` implements `Display` you are allowed to print it with: `print!, println!, write` etc.
When running the above code you are supposed to see colored text with foreground color 'red' and with the background color 'white'.

_note: you don't have to color both backround an foreground, if not specified they remain as they are_.

### RGB
Most UNIX terminals and all Windows 10 consoles are supporting [True color(24-bit)](https://en.wikipedia.org/wiki/Color_depth#True_color_(24-bit)) coloring scheme.
You can set the color of the terminal by using `Color::RGB(r,g,b)`.

```
let styled_object = style("'Light green' text on 'Black' background")
    .with(Color::Rgb {r: 0, g: 255, b: 128})
    .on(Color::Rgb {r: 0, g: 0, b: 0});
```
This will print some light green text on black background.

### Custom ANSI color value
When working on UNIX or Windows 10 you could also specify a custom ANSI value ranging up from 0 to 256.
See [256 (Xterm, 8-bit) colors](https://jonasjacek.github.io/colors/) for more information.

```
let styled_object = style("'Red' text on 'White' background")
    .with(Color::AnsiValue(9))
    .on(Color::AnsiValue(15));


println!("{}", styled_object);
```

## Attributes
When working with UNIX or Windows 10 terminals you could also use attributes to style your font. For example you could cross your text with a line and make it bold.
See [this](styling.md#Attributes) for more information.

```
let styled_object = style("'Red' text on 'White' background")
    .attr(Attribute::CrossedOut)
    .attr(Attribute::Bold);

println!("{}", styled_object);
```

---------------------------------------------------------------------------------------------------------------------------------------------
More examples could be found at this [link](https://github.com/TimonPost/crossterm/blob/master/examples/style.rs).