## Screen Buffer
A screen buffer is a two-dimensional array of characters and color data to be output in a console window. 
An terminal can have multiple of those screen buffers, and the active screen buffer is the one that is displayed on the screen.

Crossterm allows you to switch between those buffers; the screen you are working in is called the 'main screen'.  We call the other screen the 'alternate screen'. 
One note to take is that crossterm does not support the creation and switching between several buffers.

### Alternate Screen
Normally you are working on the main screen but an alternate screen is somewhat different from a normal screen.
For example, it has the exact dimensions of the terminal window, without any scrollback region. An example of this is vim when it is launched from bash.

Vim uses the entirety of the screen to edit the file, then exits to bash leaving the original buffer unchanged.

Crossterm provides the ability to switch to the alternate screen, make some changes, and then go back to the main screen. 
The main screen will still have its original data since we made all the edits on the alternate screen.

## Raw screen
To understand the concept of a 'raw screen' let's look at the following points:

**No line buffering.**
Normally the terminals use line buffering. This means that the input will be sent to the terminal line by line. With raw mode, the input will send one byte at a time.

**Input**

 All input has to be written to the screen buffer manually by the programmer.

**Characters**

The characters are not processed by the terminal driver. Also, special character have no meaning. For example, backspace will not be interpreted as backspace but instead will be sent directly to the terminal.

**Escape Characters**
Note that in raw mode `\n` `\r` will move the cursor to a new line but it will be at the same position as it was on the previous line.

_example of what I mean_
 ```
 some text
          some text
 ```

To start at the beginning of the next line, use `\n\r`.

---------------------------------------------------------------------------------------------------------------------------------------------
More examples could be found [over here](https://github.com/TimonPost/crossterm/blob/master/examples/).