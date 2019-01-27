I would really appreciate any contributing to this crate. However there are some things that are handy to know.

## How it works
Crossterm is using ANSI escape codes by default for both Unix and Windows systems. 
But for Windows, it is a bit more complicated since Windows versions 8 or lower are not supporting ANSI escape codes. 
This is why we use WinApi for those machines. 

## Architecture
Here I will discuss the architecture of crossterm. 
Crossterm wraps 5 crates: cursor, input, style, terminal, screen. 

### The different crates

If you would like to contribute to Crossterm, than please design the code as it is now. 
For example, a module like cursor has the following file structure:

- module name
    - mod.rs
    
      This file contains some trait, in this case, `ITerminalCursor`, for other modules to implement. So that it can work at a specific platform.
      
    - cursor.rs
    
      The end user will call this module to access the cursor functionalities. This module will decide which implementation to use based on the current platform.
    - winapi_cursor
    
      This is the cursor trait (located in mod.rs) implementation with WinApi.
    - ansi_cursor
    
      This is the cursor trait (located in mod.rs) implementation with ANSI escape codes.
 - sys
 
    contains platform specific logic.
  
The above structure is the same for the other modules. 

Why I have chosen for this design:
- Because you can easily extend to multiple platforms by implementing the trait int the mod.rs.
- You keep the functionalities for different platforms separated in different files. 
- Also, you have one API the user can call like in the `cursor.rs` above. This file should be avoided to change that much. All the other code could change a lot because it has no impact on the user side.