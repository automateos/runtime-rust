//! Module for execution tracing.
//! Most of his functionalities are available only on debug release
#[macro_export]
macro_rules! console_log {
    () => {
        println!();
    };
    ($($arg:tt)*) => {{
        println!($($arg)*);
    }};
}


