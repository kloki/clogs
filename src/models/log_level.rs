use std::str::FromStr;

use colorize::AnsiColor;
use serde::{Deserialize, Deserializer};
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Default)]
pub enum LogLevel {
    Error,
    Info,
    Warning,
    #[default]
    Debug,
    Trace,
}

impl FromStr for LogLevel {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "error" => Ok(LogLevel::Error),
            "warning" => Ok(LogLevel::Warning),
            "warn" => Ok(LogLevel::Warning),
            "info" => Ok(LogLevel::Info),
            "debug" => Ok(LogLevel::Debug),
            "trace" => Ok(LogLevel::Trace),
            _ => Err("Invalid log level"),
        }
    }
}

impl<'de> Deserialize<'de> for LogLevel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        LogLevel::from_str(s).map_err(serde::de::Error::custom)
    }
}

impl LogLevel {
    pub fn to_clog(&self) -> String {
        match self {
            LogLevel::Error => " ERROR".redb().black(),
            LogLevel::Warning => "  WARN".yellowb().black(),
            LogLevel::Info => "  INFO".greenb().black(),
            LogLevel::Debug => " DEBUG".blueb().black(),
            LogLevel::Trace => " TRACE".magentab().black(),
        }
    }
}
