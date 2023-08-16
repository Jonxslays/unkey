lazy_static::lazy_static! {
    /// An evenironment variable containing the log level env var.
    pub static ref UNKEY_LOG: Log = match option_env!("UNKEY_LOG") {
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

#[derive(Eq, PartialEq, PartialOrd)]
pub enum Log {
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

#[macro_export]
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
