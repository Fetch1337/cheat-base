#[cfg(debug_assertions)]
#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        tracing::info!($($arg)*);
    };
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        let _ = format_args!($($arg)*);
    };
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        tracing::error!($($arg)*);
    };
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        let _ = format_args!($($arg)*);
    };
}
