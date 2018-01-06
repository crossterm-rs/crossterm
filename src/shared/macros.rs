/// This macro will take an ANSI input and combines it with some default ANSI characters and returns the result
#[macro_export]
macro_rules! csi {
    ($( $l:expr ),*) => { concat!("\x1B[", $( $l ),*) };
}
