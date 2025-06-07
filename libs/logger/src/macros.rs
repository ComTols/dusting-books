#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::log($crate::LogLevel::Debug, "DEBUG", format!($($arg)*));
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::log($crate::LogLevel::Info, "INFO", format!($($arg)*));
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::log($crate::LogLevel::Warn, "WARN", format!($($arg)*));
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::log($crate::LogLevel::Error, "ERROR", format!($($arg)*));
    };
}

#[macro_export]
macro_rules! panic_log {
    ($($arg:tt)*) => {
        $crate::log($crate::LogLevel::Panic, "PANIC", format!($($arg)*));
    };
}
