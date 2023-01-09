/// Macro to print error message.
#[macro_export]
macro_rules! parser_error {
    ($span:expr, $fmt:expr) => {
        panic!("{}: {}", $span.start_pos, format!($fmt));
    };
    ($span:expr, $fmt:expr, $($arg:tt)*) => {
        panic!("{}: {}", $span.start_pos, format!($fmt, $($arg)*));
    };
}
