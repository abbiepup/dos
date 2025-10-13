//! Standard library macros
//!
//! This module contains a set of macros which are exported from the standard
//! library. Each macro is available for use when linking against the standard
//! library.

/// Prints to the standard output.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        use core::fmt::Write;
        write!(&mut $crate::io::stdout(), $($arg)*);
    };
}

/// Prints to the standard error.
#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => {
        use core::fmt::Write;
        write!(&mut $crate::io::stderr(), $($arg)*);
    };
}
