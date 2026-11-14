use abi_stable::std_types::RStr;
use carrion_api::core::logger::{LogLevel, LoggerApi};

pub struct LoggerApiImpl;

impl LoggerApi for LoggerApiImpl {
    fn _log(&self, level: LogLevel, target: RStr<'_>, message: RStr<'_>) {
        let (t, m) = (target.as_str(), message.as_str());

        let message = format!("\x1b[1D\x1b[2m{t}:\x1b[0m {m}");

        match level {
            LogLevel::Debug => tracing::debug!(target: "plugin:", "{message}"),
            LogLevel::Info  => tracing::info!( target: "plugin:", "{message}"),
            LogLevel::Warn  => tracing::warn!( target: "plugin:", "{message}"),
            LogLevel::Error => tracing::error!(target: "plugin:", "{message}"),
        }
    }
}