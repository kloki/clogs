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
    fn decide_header(&self) -> String {
        for key in ["message", "label"] {
            if let Some(content) = self.fields.get(key) {
                return content.clone();
            }
        }
        "".to_string()
    }

    fn timestamp_clog(&self) -> String {
        format!("[{}]", self.timestamp.format("%H:%M:%S").to_string()).grey()
    }
    fn fields_clog(&self) -> String {
        self.fields
            .iter()
            .filter(|(k, _)| k != &"message" && k != &"label")
            .map(|(k, v)| {
                format!(
                    "\n           {}:{}",
                    format!("{:<12}", k).cyan(),
                    v.to_string().grey()
                )
            })
            .collect::<Vec<_>>()
            .join("")
    }
    pub fn to_single_clog(&self) -> String {
        format!(
            "{} {} {}",
            self.timestamp_clog(),
            self.level.to_clog(),
            self.decide_header(),
        )
    }

    pub fn to_multi_clog(&self) -> String {
        format!("{}{}", self.to_single_clog(), self.fields_clog())
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
