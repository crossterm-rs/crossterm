Crossterm provides options for you to style your font and terminal. Take for example coloring output and applying attributes.

**Color support**
Windows systems with versions less than 10 only have support for 16 colors and don't have any support for attributes.

## Colors
There are 16 base colors which available for almost all terminals even windows 7 and 8.

| Light Variant  | Dark Variant    |
| :-------------| :------------- |
|       Grey     |      Black      | 
|       Red      |      DarkRed    | 
|       Green    |      DarkGreen  | 
|       Yellow   |      DarkYellow | 
|       Blue     |      DarkBlue   | 
|       Magenta  |      DarkMagenta| 
|       Cyan     |      DarkCyan   | 
|       White    |      DarkWhite  | 

In addition to 16 colours, most UNIX terminals and Windows 10 consoles are also supporting more colors.
Those colors could be: [True color (24-bit)](https://en.wikipedia.org/wiki/Color_depth#True_color_(24-bit)) coloring scheme, which allows you to use [RGB](https://nl.wikipedia.org/wiki/RGB-kleursysteem), and [256 (Xterm, 8-bit)](https://jonasjacek.github.io/colors/) colors.

## Attributes
Only UNIX and Windows 10 terminals are supporting attributes on top of text. Crossterm allows you to add attributes to the text.
Not all attributes are widely supported for all terminals, keep that in mind when working with this.

**Unix Attributes**

| Attribute      | Note         |
| :-------------: | :-------------: |
|       Bold       |      _ | 
|       Underlined |      _| 
|       Dim        |      _| 
|       SlowBlink  |    less than 150 per minute  | 
|       CrosseOut  |    characters legible, but marked for deletion. | 
|       Italic     |    not widely supported; Sometimes treated as inverse  | 
|       RapidBlink |    not widely supported; MS-DOS ANSI.SYS; 150+ per minute  | 
|       Reverse    |    not widely supported | 
|       Hidden     |    not widely supported | 

**Windows Attributes**

| Attribute      | Note         |
| :-------------: | :-------------: |
|       Reset      |     _        | 
|       Underlined |     _        | 
|       NoUnderline |    _        |
|       Negative |       _        |
|       Positive |       _        |

Now we have covered the basics of styling lets go some [examples](styling_example.md).
 
---------------------------------------------------------------------------------------------------------------------------------------------
Next up: [Examples](styling_example.md)