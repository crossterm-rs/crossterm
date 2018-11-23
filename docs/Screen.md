# Basic usage of screen
As you may have seen crossterm has some type called `Screen`. This type should be used when working with alternate or raw screen. 
Before we coninue I'll explain what those concepts are.

## Screen Buffer
A screen buffer is a two-dimensional array of character and color data for output in a console window. 
A console/terminal can have multiple screen buffers. 
The active screen buffer is the one that is displayed on the screen.

Crossterm allows you to swich between those buffers, we call this hte 'alternate screen'.

### Alternate screen
Normaly you are working on the main screen but An alternate screen is somewhat different to a normal screen.  
It has for example exactly the dimensions of the terminal window, without any scrollback region.
For an example of this behavior, consider when vim is launched from bash.
Vim uses the entirety of the screen to edit the file, then returning to bash leaves the original buffer unchanged.

## Raw screen
To understeand the concept of 'raw screen' let's look at the following points.

- No line buffering.
   Normally the terminals uses line buffering. This means that the input will be send to the terminal line by line.
   With raw mode the input will be send one byte at a time.
- Input
  All input has to be written to the screen buffer manually by the programmer.
- Characters
  The characters are not processed by the terminal driver, but are sent straight through.
  Special character have no meaning, like backspace will not be interpret as backspace but instead will be directly send to the terminal.
- Escape characters
  Note that in raw modes `\n` `\r` will move to the new line but the cursor will be at the same position as before on the new line therefor use `\n\r` to start at the new line at the first cell.
 
 _example_
  ```
  some text\n
           some text
  ```

# Example
