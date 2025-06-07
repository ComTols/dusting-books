use std::env;
use std::fs::{OpenOptions};
use std::io::Write;
use std::path::Path;
use std::sync::Mutex;

use chrono::Local;
use colored::Colorize;
use once_cell::sync::Lazy;

#[derive(PartialEq, PartialOrd, Debug)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
    Panic,
}

pub fn log(level: LogLevel, label: &str, message: String) {
    LOGGER.log(level, label, message);
}

static LOGGER: Lazy<Logger> = Lazy::new(|| Logger::new());

struct Logger {
    file: Mutex<std::fs::File>,
    level: LogLevel,
}

impl Logger {
    fn new() -> Self {
        let log_path = Path::new("app.log");
        if let Some(parent) = log_path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }

        let log_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_path)
            .expect("unable to open log file");

        let level = match env::var("DUSTY_LOG_LEVEL").unwrap_or_default().to_lowercase().as_str() {
            "debug" => LogLevel::Debug,
            "info" => LogLevel::Info,
            "warn" => LogLevel::Warn,
            "error" => LogLevel::Error,
            "panic" => LogLevel::Panic,
            "" => {
                if cfg!(debug_assertions) {
                    LogLevel::Debug
                } else {
                    LogLevel::Warn
                }
            }
            _ => LogLevel::Warn,
        };

        Logger { file: Mutex::new(log_file), level }
    }

    fn log(&self, level: LogLevel, label: &str, message: String) {
        if level < self.level {
            return;
        }

        let now = Local::now().format("%Y-%m-%d %H:%M:%S");
        let formatted = format!("[{}] {:<5} {}", now, label, message);

        if let Ok(mut file) = self.file.lock() {
            let _ = writeln!(file, "{}", formatted);
        }

        match level {
            LogLevel::Debug => println!("{}", formatted.green()),
            LogLevel::Info => println!("{}", formatted),
            LogLevel::Warn => println!("{}", formatted.yellow()),
            LogLevel::Error => println!("{}", formatted.red()),
            LogLevel::Panic => println!("{}", formatted.black().on_red()),
        }

        if level == LogLevel::Panic {
            panic!("{}", message);
        }
    }
}
