use std::sync::OnceLock;

use abi_stable::{sabi_trait, std_types::{RBox, RStr}};

use crate::core::feature::{FeatureKey, FeatureRef};

static LOGGER: OnceLock<FeatureRef<LoggerApiObj>> = OnceLock::new();

pub const PLUGIN_ID: &str = "core";
pub const FEATURE_ID: &str = "logger";

pub type LoggerApiObj = LoggerApi_TO<'static, RBox<()>>;
pub const FEATURE_KEY: FeatureKey<LoggerApiObj> = FeatureKey::new(PLUGIN_ID, FEATURE_ID);


pub fn init_logger(l: FeatureRef<LoggerApiObj>) {
    let _ = LOGGER.set(l);
}

#[repr(u8)]
#[derive(abi_stable::StableAbi, Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}


#[sabi_trait]
pub trait LoggerApi: Sync + Send {
    #[doc(hidden)]
    fn _log(&self, level: LogLevel, target: RStr, message: RStr);
}

#[doc(hidden)]
pub fn __log(level: LogLevel, target: &str, args: std::fmt::Arguments<'_>) {
    let msg = args.to_string();
    match LOGGER.get() {
        Some(l) => {
            let obj: &LoggerApiObj = &**l;
            obj._log(level, target.into(), msg.as_str().into());
        }
        None => eprintln!("[{target}] {msg}"),
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __log_impl {
    ($level:ident, $($arg:tt)*) => {
        $crate::core::logger::__log(
            $crate::core::logger::LogLevel::$level,
            env!("CARGO_PKG_NAME"),
            format_args!($($arg)*),
        )
    };
}

#[macro_export]
macro_rules! debug { ($($arg:tt)*) => { $crate::__log_impl!(Debug, $($arg)*) }; }
#[macro_export]
macro_rules! info  { ($($arg:tt)*) => { $crate::__log_impl!(Info,  $($arg)*) }; }
#[macro_export]
macro_rules! warn  { ($($arg:tt)*) => { $crate::__log_impl!(Warn,  $($arg)*) }; }
#[macro_export]
macro_rules! error { ($($arg:tt)*) => { $crate::__log_impl!(Error, $($arg)*) }; }