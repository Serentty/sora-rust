pub mod io;
pub mod serial;
pub mod vga;

macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

macro_rules! print {
    ($($arg:tt)*) => ({
            use core::fmt::Write;
            $crate::arch::vga::WRITER.lock().write_fmt(format_args!($($arg)*)).unwrap();
    });
}

macro_rules! printlns {
    ($fmt:expr) => (prints!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (prints!(concat!($fmt, "\n"), $($arg)*));
}

macro_rules! prints {
    ($($arg:tt)*) => ({
            use core::fmt::Write;
            $crate::arch::serial::WRITER.lock().write_fmt(format_args!($($arg)*)).unwrap();
    });
}