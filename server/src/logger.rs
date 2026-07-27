use std::fs::OpenOptions;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn log_action(message: &str) {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    match OpenOptions::new()
        .create(true)
        .append(true)
        .open("operation_log.txt")
    {
        Ok(mut file) => {
            if let Err(e) = writeln!(file, "[{}] {}", now, message) {
                println!("Failed to write log: {}", e);
            }
        }
        Err(e) => {
            println!("Failed to open log file: {}", e);
        }
    }
}
