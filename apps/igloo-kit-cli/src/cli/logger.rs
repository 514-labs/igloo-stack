use serde::Deserialize;
use std::time::SystemTime;
use uuid::Uuid;

use super::settings::user_directory;

const LOG_FILE: &str = "cli.log";

#[derive(Deserialize, Debug, Clone)]
pub enum LoggerLevel {
    DEBUG,
    INFO,
    WARN,
    ERROR,
}

impl LoggerLevel {
    pub fn to_log_level(&self) -> log::LevelFilter {
        match self {
            LoggerLevel::DEBUG => log::LevelFilter::Debug,
            LoggerLevel::INFO => log::LevelFilter::Info,
            LoggerLevel::WARN => log::LevelFilter::Warn,
            LoggerLevel::ERROR => log::LevelFilter::Error,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct LoggerSettings {
    #[serde(default = "default_log_file")]
    pub log_file: String,
    #[serde(default = "default_log_level")]
    pub level: LoggerLevel,
}

fn default_log_file() -> String {
    let mut dir = user_directory();
    dir.push(LOG_FILE);
    dir.to_str().unwrap().to_string()
}

fn default_log_level() -> LoggerLevel {
    LoggerLevel::INFO
}

impl Default for LoggerSettings {
    fn default() -> Self {
        LoggerSettings {
            log_file: default_log_file(),
            level: default_log_level(),
        }
    }
}

// TODO ensure that the log file rotates after a certain size
pub fn setup_logging(settings: LoggerSettings) -> Result<(), fern::InitError> {
    let session_id = Uuid::new_v4().to_string();

    let base_config = fern::Dispatch::new().level(settings.level.to_log_level());

    let file_config = fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                &session_id,
                message
            ))
        })
        .chain(fern::log_file(settings.log_file)?);

    base_config.chain(file_config).apply()?;

    Ok(())
}
