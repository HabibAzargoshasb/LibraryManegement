use std::fs::OpenOption;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn log_action(message : &str) {
    let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
    let mut file = OpenOption::new()
        .create(true)
        .append(true)
        .open(" operation_log.txt")
        .unwrap();

     writeln!(file, "[{}] {}", now, message).unwrap();

}