use chrono::Local;
use std::fs::{self, create_dir_all, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::OnceLock;

static LOG_PATH: OnceLock<PathBuf> = OnceLock::new();

/// Initialize the logger with the app data directory
pub fn init(app_data_dir: PathBuf) {
    let log_path = app_data_dir.join("log.txt");
    let _ = LOG_PATH.set(log_path);
}

/// Get the log file path
fn get_log_path() -> Option<&'static PathBuf> {
    LOG_PATH.get()
}

/// Write a log entry to the log file
fn write_log(level: &str, message: &str) {
    let Some(log_path) = get_log_path() else {
        eprintln!("[Logger] Not initialized: {}", message);
        return;
    };

    // Ensure parent directory exists
    if let Some(parent) = log_path.parent() {
        let _ = create_dir_all(parent);
    }

    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
    let log_entry = format!("[{}] [{}] {}\n", timestamp, level, message);

    // Append to log file
    match OpenOptions::new().create(true).append(true).open(log_path) {
        Ok(mut file) => {
            let _ = file.write_all(log_entry.as_bytes());
        }
        Err(e) => {
            eprintln!("[Logger] Failed to write log: {}", e);
        }
    }

    // Also print to stderr for debugging
    eprint!("{}", log_entry);
}

/// Log an error message
pub fn error(message: &str) {
    write_log("ERROR", message);
}

/// Log an error with context
pub fn error_with_context(context: &str, message: &str) {
    write_log("ERROR", &format!("[{}] {}", context, message));
}

/// Log a warning message
pub fn warn(message: &str) {
    write_log("WARN", message);
}

/// Log an info message
pub fn info(message: &str) {
    write_log("INFO", message);
}

/// Read the last N lines from the log file.
/// Caps read to the last 512KB to avoid OOM on large log files.
pub fn read_recent_logs(max_lines: usize) -> String {
    use std::io::{Read, Seek, SeekFrom};

    let Some(log_path) = get_log_path() else {
        return "Logger not initialized".to_string();
    };

    let mut file = match fs::File::open(log_path) {
        Ok(f) => f,
        Err(e) => return format!("Failed to read log file: {}", e),
    };

    let metadata = match file.metadata() {
        Ok(m) => m,
        Err(e) => return format!("Failed to read log file metadata: {}", e),
    };

    // Only read the tail of the file (max 512KB) to prevent OOM
    const MAX_READ_BYTES: u64 = 512 * 1024;
    let file_len = metadata.len();
    if file_len > MAX_READ_BYTES {
        let _ = file.seek(SeekFrom::End(-(MAX_READ_BYTES as i64)));
    }

    let mut content = String::new();
    if let Err(e) = file.read_to_string(&mut content) {
        return format!("Failed to read log file: {}", e);
    }

    let lines: Vec<&str> = content.lines().collect();
    let start = lines.len().saturating_sub(max_lines);
    lines[start..].join("\n")
}
