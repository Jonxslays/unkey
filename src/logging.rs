lazy_static::lazy_static! {
    /// An environment variable containing the log level env var.
    pub(crate) static ref UNKEY_LOG: Log = match option_env!("UNKEY_LOG") {
        None => Log::None,
        Some(level) => match level {
           "debug" | "DEBUG" => Log::Debug,
           "info" | "INFO" => Log::Info,
           "error" | "ERROR" => Log::Error,
           _ => {
               eprintln!("Invalid UNKEY_LOG level detected: {level}");
               Log::None
           }
        }
    };
}

/// The different logging levels supported by the crate.
#[derive(Eq, PartialEq, PartialOrd)]
pub(crate) enum Log {
    None,
    Error,
    Info,
    Debug,
}

impl From<&Log> for String {
    fn from(val: &Log) -> String {
        let message = match val {
            Log::Debug => "[DEBUG]",
            Log::Info => "[INFO] ",
            Log::Error => "[ERROR]",
            Log::None => "",
        };

        message.to_string()
    }
}

impl From<Log> for String {
    fn from(val: Log) -> String {
        (&val).into()
    }
}

impl std::fmt::Display for Log {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = self.into();
        write!(f, "{s}")
    }
}

/// Logs the given message at the given level.
macro_rules! log {
    ($level:expr, $message:expr) => {
        if *$crate::logging::UNKEY_LOG >= $level {
            match $level {
                $crate::logging::Log::None => (),
                $crate::logging::Log::Error => eprintln!("{} {}", $level, $message),
                $crate::logging::Log::Info | $crate::logging::Log::Debug => {
                    println!("{} {}", $level, $message)
                }
            }
        }
    };
}

/// Logs the given message at the debug level.
macro_rules! debug {
    ($message:expr) => {
        $crate::logging::log!($crate::logging::Log::Debug, $message)
    };
}

/// Logs the given message at the infomation level.
macro_rules! info {
    ($message:expr) => {
        $crate::logging::log!($crate::logging::Log::Info, $message)
    };
}

/// Logs the given message at the error level.
macro_rules! error {
    ($message:expr) => {
        $crate::logging::log!($crate::logging::Log::Error, $message)
    };
}

pub(crate) use debug;
pub(crate) use error;
pub(crate) use info;
pub(crate) use log;
