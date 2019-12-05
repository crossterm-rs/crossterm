cfg_if::cfg_if! {
    if #[cfg(unix)] {
        pub(crate) mod unix;
        #[cfg(feature = "event-stream")]
        pub(crate) use unix::Waker;
    } else if #[cfg(windows)] {
        pub(crate) mod windows;
        #[cfg(feature = "event-stream")]
        pub(crate) use windows::Waker;
    }
}
