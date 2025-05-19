use crate::sbi::console_putchar;
use core::fmt::{self, Write};

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        s.chars().for_each(|c| console_putchar(c as usize));
        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    };
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    };
}

// mod colors {
//     pub const RESET: &str = "\x1b[0m";
//     pub const RED: &str = "\x1b[31m";
//     pub const GREEN: &str = "\x1b[32m";
//     pub const YELLOW: &str = "\x1b[33m";
//     pub const BLUE: &str = "\x1b[34m";
//     pub const MAGENTA: &str = "\x1b[35m";
//     pub const CYAN: &str = "\x1b[36m";
//     pub const WHITE: &str = "\x1b[37m";
// }

#[macro_export]
macro_rules! info {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(
            concat!("\x1b[32m[INFO] ", $fmt, "\x1b[0m\n"),  // 绿色 INFO
            $( $($arg)+ )?
        ));
    };
}

#[macro_export]
macro_rules! warn {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(
            concat!("\x1b[33m[WARN] ", $fmt, "\x1b[0m\n"),  // 黄色 WARN
            $( $($arg)+ )?
        ));
    };
}

#[macro_export]
macro_rules! error {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(
            concat!("\x1b[31m[ERROR] ", $fmt, "\x1b[0m\n"),  // 红色 ERROR
            $( $($arg)+ )?
        ));
    };
}

#[macro_export]
macro_rules! debug {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(
            concat!("\x1b[36m[DEBUG] ", $fmt, "\x1b[0m\n"),  // 青色 DEBUG
            $( $($arg)+ )?
        ));
    };
}
