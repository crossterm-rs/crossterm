use super::IScreenManager;
use kernel::windows_kernel::kernel;
use winapi::um::winnt::HANDLE;

pub struct WinApiScreenManager
{
    pub is_alternate_screen: bool,
    output: Box<HANDLE>,
}

impl IScreenManager for WinApiScreenManager where
{
    type Output = HANDLE;

    fn stdout(&mut self) -> &mut Self::Output
    {
        return &mut self.output
    }

    fn toggle_is_alternate_screen(&mut self, is_alternate_screen: bool)
    {
        self.is_alternate_screen = is_alternate_screen;
    }

    fn write_ansi(&mut self, string: String)
    {
//        write!(self.output, "{}", string);
//        self.flush();
    }

    fn write_ansi_str(&mut self, string: &str)
    {
//        write!(self.output, "{}", string);
//        self.flush();
    }
}

impl WinApiScreenManager {
    pub fn new() -> Self {
        WinApiScreenManager {
            output: (Box::from(kernel::get_output_handle())),
            is_alternate_screen: false
        }
    }
}

//impl<Output:Write> Write for AnsiScreenManager<Output>
//{
//    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
//        self.output.write(buf)
//    }
//
//    fn flush(&mut self) -> io::Result<()> {
//        self.stdout().flush()
//    }
//}