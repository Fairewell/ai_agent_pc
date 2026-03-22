use std::path::Path;
use tauri::State;

use crate::db::models::FileRecord;
use crate::db::queries;
use crate::state::AppState;

/// Opens a file with the system's default application.
#[tauri::command]
pub fn open_file(path: String) -> Result<String, String> {
    let file_path = Path::new(&path);
    if !file_path.exists() {
        return Err(format!("File not found: {}", path));
    }

    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(["/C", "start", "", &path])
            .spawn()
            .map_err(|e| format!("Failed to open file: {}", e))?;
    }

    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("Failed to open file: {}", e))?;
    }

    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("Failed to open file: {}", e))?;
    }

    Ok(format!("Opened: {}", path))
}

/// Returns the full file record (metadata) from the SQLite database for the
/// given path.
#[tauri::command]
pub fn get_file_detail(path: String, state: State<'_, AppState>) -> Result<FileRecord, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let record = queries::get_file_by_path(&db, &path).map_err(|e| e.to_string())?;

    match record {
        Some(r) => Ok(r),
        None => Err(format!("No record found for path: {}", path)),
    }
}
