use std::collections::HashSet;
use std::path::Path;

/// Returns all file extensions supported for text extraction in Phase 1 (plaintext).
pub fn supported_extensions() -> HashSet<String> {
    let exts = [
        "txt", "md", "csv", "json", "xml", "yaml", "yml", "toml",
        "py", "js", "ts", "html", "css", "rs", "go", "java",
        "c", "cpp", "h", "sh", "bat", "ps1", "cfg", "ini", "log",
        "rtf",
    ];
    exts.iter().map(|e| e.to_string()).collect()
}

/// Checks whether a path component is hidden (starts with a dot).
pub fn is_hidden(path: &Path) -> bool {
    path.file_name()
        .and_then(|n| n.to_str())
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

/// Formats a byte count into a human-readable string.
pub fn format_file_size(bytes: i64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;
    const TB: f64 = GB * 1024.0;

    let bytes_f = bytes as f64;
    if bytes_f >= TB {
        format!("{:.2} TB", bytes_f / TB)
    } else if bytes_f >= GB {
        format!("{:.2} GB", bytes_f / GB)
    } else if bytes_f >= MB {
        format!("{:.2} MB", bytes_f / MB)
    } else if bytes_f >= KB {
        format!("{:.2} KB", bytes_f / KB)
    } else {
        format!("{} B", bytes)
    }
}
