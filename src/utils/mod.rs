mod args;
mod buffer;
mod io;

pub use args::Args;
pub use buffer::{Buffer, BufferError, BufferResult};
pub use io::FdWriter;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        let mut writer = $crate::utils::FdWriter::STDOUT;
        let _ = core::fmt::write(&mut writer, format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! println {
    () => { $crate::print!("\n") };
    ($($arg:tt)*) => {
        $crate::print!("{}\n", format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => {{
        let mut writer = $crate::utils::FdWriter::STDERR;
        let _ = core::fmt::write(&mut writer, format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! eprintln {
    () => { $crate::eprint!("\n") };
    ($($arg:tt)*) => {
        $crate::eprint!("{}\n", format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! dbg {
    () => {
        $crate::eprintln!("[{}:{}]", file!(), line!());
    };
    ($val:expr $(,)?) => {
        match $val {
            tmp => {
                $crate::eprintln!("[{}:{}] {} = {:#?}",
                    file!(), line!(), stringify!($val), &tmp);
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($($crate::dbg!($val)),+,)
    };
}
