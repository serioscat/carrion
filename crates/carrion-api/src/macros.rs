

#[macro_export]
macro_rules! info {
    ($core:expr, $($arg:tt)*) => {
        $core._log($crate::LogLevel::Info, env!("CARGO_PKG_NAME").into(), format!($($arg)*).as_str().into())
    };
}

#[macro_export]
macro_rules! warn {
    ($core:expr, $($arg:tt)*) => {
        $core._log($crate::LogLevel::Warn, env!("CARGO_PKG_NAME").into(), format!($($arg)*).as_str().into())
    };
}

#[macro_export]
macro_rules! error {
    ($core:expr, $($arg:tt)*) => {
        $core._log($crate::LogLevel::Error, env!("CARGO_PKG_NAME").into(), format!($($arg)*).as_str().into())
    };
}

#[macro_export]
macro_rules! debug {
    ($core:expr, $($arg:tt)*) => {
        $core._log($crate::LogLevel::Debug, env!("CARGO_PKG_NAME").into(), format!($($arg)*).as_str().into())
    };
}