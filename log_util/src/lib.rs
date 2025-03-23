use chrono::Local;

pub fn log_message(left: &str, right: &str) -> Option<String> {
    match left {
        "INFO" | "WARN" | "ERROR" => {
            let dt = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            Some(format!("[{}] {} - {}",left,dt,right).to_string())

        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_log_message_info() {
        let result = log_message("INFO", "Service started");
        assert!(result.unwrap().contains("[INFO]"));
    }

    #[test]
    fn it_log_message_warn() {
        let result = log_message("WARN", "Disk space is low");
        assert!(result.unwrap().contains("[WARN]"));
    }

    #[test]
    fn it_log_message_error() {
        let result = log_message("ERROR", "Failed to connect to DB");
        assert!(result.unwrap().contains("[ERROR]"));
    }

    #[test]
    fn it_log_message_debug() {
        let result = log_message("DEBUG", "This won't be logged");
        assert_eq!(result, None);
    }
}
