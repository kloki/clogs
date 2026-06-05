use std::io::{self, BufRead};

use clap::Parser;
use clogs::models::{log_level::LogLevel, log_line::LogLine};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Maximum log level
    #[arg(short, long, default_value = "trace")]
    log_level: LogLevel,
    /// exclude extra fields from the output
    #[arg(short, long, default_value_t = false)]
    exclude_fields: bool,
}

fn handle_line(content: String, max_log_level: &LogLevel, exclude_fields: bool) {
    match serde_json::from_str::<LogLine>(&content) {
        Ok(log_line) => {
            if log_line.level <= *max_log_level {
                if exclude_fields {
                    println!("{}", log_line.to_single_clog())
                } else {
                    println!("{}", log_line.to_multi_clog())
                }
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
                handle_line(content, &args.log_level, args.exclude_fields);
            }
            Err(_) => break,
        }
    }
}
