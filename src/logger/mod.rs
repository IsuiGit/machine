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

// Create enumerate for logging leveling
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel{
    Error,
    Info,
    Debug,
}

// Impl string compare from enum to static str
impl LogLevel {
    fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Error => "ERROR",
            LogLevel::Info  => "INFO",
            LogLevel::Debug => "DEBUG",
        }
    }
}

// Main logger instance
pub struct Logger{
    log_level: LogLevel,
    log_file_path: Mutex<File>,
}

// Impl for Logger
impl Logger {
    // Create new instance of logger
    pub fn new(log_level: LogLevel, path: String) -> Result<Self, String> {
        // Create OpenOptions object for logger file IO
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .map_err(|e| format!("{}", e))?;
        // Return Logger instance
        // - log_level (minimal logging level)
        // - log_file_path (application logging file)
        Ok(Logger{log_level: log_level, log_file_path: Mutex::new(file)})
    }

    // Write function
    fn write(&self, level: LogLevel, record: String) {
        // Check if current level less than minimal
        if level > self.log_level {
            return;
        }
        // Create log string
        let line = format!(
            "{} | {} | {}\n",
            Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
            level.as_str(),
            record,
        );
        // Print created line
        print!("{}", line);
        // Check if log_file_path exists
        // - if exists lock file IO for futher time
        if let Ok(mut file) = self.log_file_path.lock() {
            // Write to file
            match file.write_all(line.as_bytes()) {
                Ok(()) => {},
                Err(e) => eprintln!("Error on write to logfile ({})", e),
            }
            match file.flush() {
                Ok(()) => {},
                Err(e) => eprintln!("Error at flushing IO buffer ({})", e),
            }
        }
    }

    // Public interface for call error, info, debug logger info
    pub fn error(&self, record: String) { self.write(LogLevel::Error, record) }
    pub fn info(&self, record: String) { self.write(LogLevel::Info, record) }
    pub fn debug(&self, record: String) { self.write(LogLevel::Debug, record) }
}

// Create global singleton instance
static LOGGER: OnceLock<Arc<Logger>> = OnceLock::new();

// Create logger instance and set global singleton
pub fn init_logger(log_level: LogLevel, path: String) -> Result<(), String> {
    let logger = Arc::new(Logger::new(log_level, path)?);
    LOGGER.set(logger).map_err(|_| "Logger already initialized")?;
    Ok(())
}

// Get global logger instance
pub fn logger() -> &'static Logger {
    LOGGER.get().expect("Logger not initialized")
}
