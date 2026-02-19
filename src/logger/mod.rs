use std::{
    sync::{
        Mutex,
        Arc,
        OnceLock,
    },
    fs::{
        File,
        OpenOptions,
    },
    io::Write,
};

use chrono::Utc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel{
    Error,
    Info,
    Debug,
}

impl LogLevel {
    fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Error => "ERROR",
            LogLevel::Info  => "INFO",
            LogLevel::Debug => "DEBUG",
        }
    }
}

pub struct Logger{
    log_level: LogLevel,
    log_file_path: Mutex<File>,
}

impl Logger {
    pub fn new(log_level: LogLevel, path: String) -> Result<Self, String> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .map_err(|e| format!("{}", e))?;
        Ok(Logger{log_level: log_level, log_file_path: Mutex::new(file)})
    }

    fn write(&self, level: LogLevel, record: String) {
        if level > self.log_level {
            return;
        }
        let line = format!(
            "[{} - {}]: {}\n",
            level.as_str(),
            Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
            record,
        );
        print!("{}", line);
        if let Ok(mut file) = self.log_file_path.lock() {
            file.write_all(line.as_bytes()).map_err(|e| panic!("Error at write to log file ({})", e));
            file.flush().map_err(|e| panic!("Error at flushing log buffer ({})", e));
        }
    }

    pub fn error(&self, record: String) { self.write(LogLevel::Error, record) }
    pub fn info(&self, record: String) { self.write(LogLevel::Info, record) }
    pub fn debug(&self, record: String) { self.write(LogLevel::Debug, record) }
}

static LOGGER: OnceLock<Arc<Logger>> = OnceLock::new();

pub fn init_logger(log_level: LogLevel, path: String) -> Result<(), String> {
    let logger = Arc::new(Logger::new(log_level, path)?);
    LOGGER.set(logger).map_err(|_| "Logger already initialized")?;
    Ok(())
}

pub fn logger() -> &'static Logger {
    LOGGER.get().expect("Logger not initialized")
}
