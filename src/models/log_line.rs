use std::{
    collections::{BTreeMap, hash_map::DefaultHasher},
    hash::{Hash, Hasher},
};

use colorize::AnsiColor;
use serde::{Deserialize, Deserializer};
use serde_json::Value;

use super::log_level::LogLevel;

const THREAD_WIDTH: usize = 10;

#[derive(Deserialize)]
pub struct LogLine {
    #[serde(deserialize_with = "trimmed_string")]
    pub timestamp: String,
    pub level: LogLevel,
    #[serde(rename = "threadName", default)]
    pub thread_name: String,
    #[serde(default)]
    pub target: String,
    #[serde(deserialize_with = "deserialize_fields")]
    pub fields: BTreeMap<String, String>,
}

impl LogLine {
    fn timestamp_clog(&self) -> String {
        format!("[{}]", self.formatted_timestamp()).grey()
    }

    fn formatted_timestamp(&self) -> String {
        // Extract HH:MM:SS from an ISO 8601 timestamp like 2026-06-05T12:00:00.002918956Z
        match self.timestamp.split_once('T') {
            Some((_, time)) => time.chars().take(8).collect(),
            None => self.timestamp.clone(),
        }
    }

    fn thread_name_clog(&self) -> String {
        if self.thread_name.is_empty() {
            return " ".repeat(THREAD_WIDTH + 2);
        }
        let name: String = self.thread_name.chars().take(THREAD_WIDTH).collect();
        let pad = " ".repeat(THREAD_WIDTH - name.chars().count());
        format!("[{}]{}", color_thread(name, &self.thread_name), pad)
    }

    fn target_clog(&self) -> String {
        self.target.clone().b_black()
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
            "{}{} {} {}{} {}",
            self.timestamp_clog(),
            self.thread_name_clog(),
            self.level.to_clog(),
            self.label_clog(),
            self.fields.get("message").cloned().unwrap_or_default(),
            self.target_clog(),
        )
    }

    pub fn to_multi_clog(&self) -> String {
        format!("{}{}", self.to_single_clog(), self.fields_clog())
    }
}

fn color_thread(s: String, name: &str) -> String {
    let mut h = DefaultHasher::new();
    name.hash(&mut h);
    // match h.finish() % 12 {
    //     0 => s.red(),
    //     1 => s.green(),
    //     2 => s.yellow(),
    //     3 => s.blue(),
    //     4 => s.magenta(),
    //     5 => s.cyan(),
    //     6 => s.b_red(),
    //     7 => s.b_green(),
    //     8 => s.b_yellow(),
    //     9 => s.b_blue(),
    //     10 => s.b_magenta(),
    //     _ => s.b_cyan(),
    // }
    match h.finish() % 6 {
        0 => s.green(),
        1 => s.red(),
        2 => s.yellow(),
        3 => s.magenta(),
        4 => s.blue(),
        _ => s.cyan(),
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

fn trimmed_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    Ok(s.trim().to_string())
}
