There are some problems I discovered during development. 
And I don't think it has to do anything with crossterm but it has to do whit how terminals handle ANSI or WinApi. 

# Winapi-problems
- Power shell does not interpreter 'DarkYellow' and is instead using gray instead, cmd is working perfectly fine.
- Power shell inserts an '\n' (enter) when the program starts, this enter is the one you pressed when running the command.
- After the program ran, power shell will reset the background color.

# UNIX-terminals
The Arc and Manjaro KDE Konsole's are not seeming to resize the terminal instead they are resizing the buffer. 