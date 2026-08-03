use std::fmt;

use crate::{impl_display, osc, Command};

/// A command that starts an [OSC 8 hyperlink].
///
/// Text printed after this command will be a clickable hyperlink in
/// supported terminals until [`EndHyperlink`] is printed.
///
/// [OSC 8 hyperlink]: https://gist.github.com/egmontkob/eb114294efbcd5adb1944c9f3cb5feda
///
/// # Notes
///
/// Commands must be executed/queued for execution otherwise they do nothing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StartHyperlink<T> {
    pub url: T,
    pub params: Vec<(String, String)>,
}

impl<T> StartHyperlink<T>
where
    T: AsRef<str>,
{
    pub fn new(url: T) -> Self {
        Self {
            url,
            params: Vec::new(),
        }
    }

    pub fn param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.params.push((key.into(), value.into()));
        self
    }
}

impl<T> Command for StartHyperlink<T>
where
    T: AsRef<str>,
{
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        // OSC 8 ; params ; uri ST
        // params are key=value pairs separated by ':'
        f.write_str("\x1B]8;")?;
        for (i, (k, v)) in self.params.iter().enumerate() {
            if i > 0 {
                f.write_char(':')?;
            }
            f.write_str(k)?;
            f.write_char('=')?;
            f.write_str(v)?;
        }
        f.write_char(';')?;
        f.write_str(self.url.as_ref())?;
        f.write_str("\x1B\\")
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> std::io::Result<()> {
        Ok(())
    }
}

/// A command that ends an [OSC 8 hyperlink].
///
/// [OSC 8 hyperlink]: https://gist.github.com/egmontkob/eb114294efbcd5adb1944c9f3cb5feda
///
/// # Notes
///
/// Commands must be executed/queued for execution otherwise they do nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EndHyperlink;

impl Command for EndHyperlink {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        f.write_str(osc!("8;;"))
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> std::io::Result<()> {
        Ok(())
    }
}

impl_display!(for StartHyperlink<T> where T: AsRef<str>);
impl_display!(for EndHyperlink);

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use super::*;

    #[test]
    fn start_no_params() {
        let mut buf = String::new();
        StartHyperlink::new("https://example.com")
            .write_ansi(&mut buf)
            .unwrap();
        assert_eq!(buf, "\x1B]8;;https://example.com\x1B\\");
    }

    #[test]
    fn start_with_param() {
        let mut buf = String::new();
        StartHyperlink::new("https://example.com")
            .param("id", "link1")
            .write_ansi(&mut buf)
            .unwrap();
        assert_eq!(buf, "\x1B]8;id=link1;https://example.com\x1B\\");
    }

    #[test]
    fn start_with_multiple_params() {
        let mut buf = String::new();
        StartHyperlink::new("https://example.com")
            .param("id", "link1")
            .param(String::from("foo"), String::from("bar"))
            .param(Cow::Borrowed("baz"), Cow::Borrowed("fuz"))
            .write_ansi(&mut buf)
            .unwrap();
        assert_eq!(
            buf,
            "\x1B]8;id=link1:foo=bar:baz=fuz;https://example.com\x1B\\"
        );
    }

    #[test]
    fn start_owned_string() {
        let mut buf = String::new();
        StartHyperlink::new(String::from("https://example.com"))
            .param("id", "link1")
            .write_ansi(&mut buf)
            .unwrap();
        assert_eq!(buf, "\x1B]8;id=link1;https://example.com\x1B\\");
    }

    #[test]
    fn start_cow() {
        let mut buf = String::new();
        StartHyperlink::new(Cow::Borrowed("https://example.com"))
            .param("id", "link1")
            .write_ansi(&mut buf)
            .unwrap();
        assert_eq!(buf, "\x1B]8;id=link1;https://example.com\x1B\\");
    }

    #[test]
    fn end_hyperlink() {
        let mut buf = String::new();
        EndHyperlink.write_ansi(&mut buf).unwrap();
        assert_eq!(buf, "\x1B]8;;\x1B\\");
    }
}
