use crossterm_state::{Context};
use super::IContextCommand;

use kernel::unix_kernel::terminal::Termios;
use kernel::unix_kernel::terminal;

#[derive(Clone, Copy)]
pub struct NoncanonicaModeCommand
{
    key: i16
}

impl IContextCommand for NoncanonicalModeCommand
{
    fn new(context: &mut Context) -> (Box<NoncanonicalModeCommand>) {
//        println!("new new NoncanonicalModeCommand unix");
        let key = super::generate_key();
        let command = NoncanonicaModeCommand{ key: key };
        context.register_change(command,key);
        (Box::from(NoncanonicalModeCommand {}), key)
    }

    fn execute(&mut self) -> bool
    {
//        println!("execute NoncanonicalModeCommand uxix");
        // Set noncanonical mode
        let orig = Termios::from_fd(FD_STDIN)?;
        let mut noncan = orig.clone();
        noncan.c_lflag &= !ICANON;
        noncan.c_lflag &= !ECHO;
        noncan.c_lflag &= !CREAD;
        match tcsetattr(FD_STDIN, TCSAFLUSH, &noncan)
            {
                Ok(_) => return true,
                Err(_) => return false,
            };
    }

    fn undo(&mut self) -> bool
    {
//        println!("undo NoncanonicalModeCommand unix");
        // Disable noncanonical mode
        let orig = Termios::from_fd(FD_STDIN)?;
        let mut noncan = orig.clone();
        noncan.c_lflag &= ICANON;
        noncan.c_lflag &= ECHO;
        noncan.c_lflag &= CREAD;

        match tcsetattr(FD_STDIN, TCSAFLUSH, &noncan)
            {
                Ok(_) => return true,
                Err(_) => return false,
            };
    }
}

#[derive(Clone, Copy)]
pub struct EnableRawModeCommand
{
    original_mode: Option<Termios>,
    key: i16
}

impl IContextCommand for EnableRawModeCommand
{
    fn new(context: &Context) -> (Box<EnableRawModeCommand>, i16) {
//        println!("new EnableRawModeCommand unix");
        let key = super::generate_key();
        let command = EnableRawModeCommand { original_mode: None, key: key };
        context.state.register_change(Box::from(command), key);
        (Box::from(command),key)
    }

    fn execute(&mut self) -> bool
    {
//        println!("execute EnableRawModeCommand unix");
        self.original_mode = terminal::get_terminal_mode()?;
        let mut new_mode = self.original_mode.unwrap();
        new_mode.make_raw();
        terminal::set_terminal_mode(new_mode);
        true
    }

    fn undo(&mut self) -> bool
    {
//        println!("undo EnableRawModeCommand unix");
        let result = terminal::set_terminal_mode(self.original_mode).unwrap();

        match result
        {
            Ok(_) => true,
            Err(_) => false
        }
    }
}
