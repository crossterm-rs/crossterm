# Styling Module

Crossterm provides options for you to style your font and terminal. Take for example coloring output and applying attributes.

**Color support**
Windows systems with versions less than 10 will only have support for 16 colors and don't have any support for attributes. Most UNIX-terminal is supporting lots of colors and attributes.

## Colors
There are 16 base colors which available for almost all terminals even windows 7 and 8.

| Light Variant  | Dark Variant    |
| :-------------| :-------------   |
|       Grey     |      Black      | 
|       Red      |      DarkRed    | 
|       Green    |      DarkGreen  | 
|       Yellow   |      DarkYellow | 
|       Blue     |      DarkBlue   | 
|       Magenta  |      DarkMagenta| 
|       Cyan     |      DarkCyan   | 
|       White    |      DarkWhite  | 

In addition to 16 colors, most UNIX terminals and Windows 10 consoles are also supporting more colors.
Those colors could be: [True color (24-bit)](https://en.wikipedia.org/wiki/Color_depth#True_color_(24-bit)) coloring scheme, which allows you to use [RGB](https://nl.wikipedia.org/wiki/RGB-kleursysteem), and [256 (Xterm, 8-bit)](https://jonasjacek.github.io/colors/) colors.
Checkout the examples on how to use this feature.

## Attributes
Only UNIX and Windows 10 terminals are supporting attributes on top of the text. Crossterm allows you to add attributes to the text.
Not all attributes are widely supported for all terminals, keep that in mind when working with this.

Crossterm implements almost all attributes shown in this [Wikipedia-list](https://en.wikipedia.org/wiki/ANSI_escape_code#SGR_(Select_Graphic_Rendition)_parameters). 

 | Attribute                      |     Support                                             |  Note         |
| :-------------:                |  :-------------:                                         | :-------------: |
|       Reset                    |  Windows, UNIX                                           |  This will reset all current set attributes.     | 
|       Bold                     |  Windows, UNIX                                           |  This will increase the font sensitivity also known as bold.     | 
|       Dim                      |  Windows, UNIX                                           |  This will decrease the font sensitivity also known as bold.   |
|       Italic                   |  Not widely supported, sometimes treated as inverse.     |  This will make the font italic.   |
|       Underlined               |  Windows, UNIX                                           |  An line under a word, especially in order to show its importance.   |                                        
|       SlowBlink                |  Not widely supported, sometimes treated as inverse.     |  less than 150 per minute  | 
|       RapidBlink               |  Not widely supported                                    |  MS-DOS ANSI.SYS; 150+ per minute;  | 
|       Reverse                  |  Windows, UNIX                                           |   foreground and background colors |                                       
|       Hidden                   |  Windows, UNIX |                                         |  Also known as 'Conceal'                                    
|       Fraktur                  |  UNIX                                                    |  characters legible, but marked for deletion. | 
|       DefaultForegroundColor   |  Unknown                                                 |  Implementation defined (according to standard) | 
|       DefaultBackgroundColor   |  Unknown                                                 |  Implementation defined (according to standard) | 
|       Framed                   |  Not widely supported                                    |  Framed font. 
|       Encircled                |  Unknown                                                 |  This will turn on the encircled attribute. | 
|       OverLined                |  Unknown                                                 |  This will draw a line at the top of the font. | 

(There are a few attributes who disable one of the above attributes, I did not write those down to keep the list short).

Now we have covered the basics of styling lets go some [examples](styling_example.md).
 
---------------------------------------------------------------------------------------------------------------------------------------------
Next up: [Examples](styling_example.md)