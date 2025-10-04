mod models;
use std::io::{self, BufRead};

use clap::Parser;
use models::{log_level::LogLevel, log_line::LogLine};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Maximum log-level
    #[arg(short, long, default_value = "debug")]
    log_level: LogLevel,
}

fn handle_line(content: String, max_log_level: &LogLevel) {
    match serde_json::from_str::<LogLine>(&content) {
        Ok(log_line) => {
            if log_line.level <= *max_log_level {
                println!("{}", log_line.to_clog())
            }
        }
        Err(e) => println!("Failed to parse: {}", e),
    }
}

fn main() {
    let args = Args::parse();
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        match line {
            Ok(content) => {
                handle_line(content, &args.log_level);
            }
            Err(_) => break,
        }
    }
}
