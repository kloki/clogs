use std::collections::BTreeMap;

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
    pub fields: BTreeMap<String, String>,
}

impl LogLine {
    fn timestamp_clog(&self) -> String {
        format!("[{}]", self.timestamp.format("%H:%M:%S").to_string()).grey()
    }

    fn label_clog(&self) -> String {
        match self.fields.get("label") {
            Some(content) => format!("[{}] ", content.clone().cyan()),
            None => "".to_string(),
        }
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
            "{} {} {}{}",
            self.timestamp_clog(),
            self.level.to_clog(),
            self.label_clog(),
            self.fields.get("message").cloned().unwrap_or_default(),
        )
    }

    pub fn to_multi_clog(&self) -> String {
        format!("{}{}", self.to_single_clog(), self.fields_clog())
    }
}

fn deserialize_fields<'de, D>(deserializer: D) -> Result<BTreeMap<String, String>, D::Error>
where
    D: Deserializer<'de>,
{
    let raw_fields: BTreeMap<String, Value> = BTreeMap::deserialize(deserializer)?;
    let fields = raw_fields
        .into_iter()
        .map(|(k, v)| (k, v.to_string().trim_matches('"').to_string()))
        .collect();
    Ok(fields)
}
