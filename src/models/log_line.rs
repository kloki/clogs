use std::collections::HashMap;

use chrono::{DateTime, Utc};
use colorize::AnsiColor;
use serde::{Deserialize, Deserializer};
use serde_json::Value;

use super::log_level::LogLevel;

#[derive(Deserialize)]
pub struct LogLine {
    pub timestamp: DateTime<Utc>,
    pub level: LogLevel,
    #[serde(deserialize_with = "deserialize_fields")]
    pub fields: HashMap<String, String>,
}

impl LogLine {
    pub fn to_clog(&self) -> String {
        let clog_fields = self
            .fields
            .iter()
            .map(|(k, v)| format!("\n  {}:{}", format!("{:<12}", k).green(), v))
            .collect::<Vec<_>>()
            .join("");

        format!(
            "{}{}{}",
            self.timestamp
                .format("%Y-%m-%d %H:%M:%S")
                .to_string()
                .black()
                .greyb(),
            self.level.to_clog(),
            clog_fields,
        )
    }
}

fn deserialize_fields<'de, D>(deserializer: D) -> Result<HashMap<String, String>, D::Error>
where
    D: Deserializer<'de>,
{
    let raw_fields: HashMap<String, Value> = HashMap::deserialize(deserializer)?;
    let fields = raw_fields
        .into_iter()
        .map(|(k, v)| (k, v.to_string().trim_matches('"').to_string()))
        .collect();
    Ok(fields)
}
