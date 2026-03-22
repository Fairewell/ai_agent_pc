use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::State;

use crate::db::queries;
use crate::indexer::pipeline::IndexPipeline;
use crate::state::AppState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexStatus {
    pub total_files: i64,
    pub indexed_files: i64,
    pub is_scanning: bool,
}

/// Returns the current indexing status: total files, indexed files, and whether
/// a scan is currently running.
#[tauri::command]
pub fn get_index_status(state: State<'_, AppState>) -> Result<IndexStatus, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let (total, indexed) = queries::get_index_stats(&db).map_err(|e| e.to_string())?;
    let is_scanning = *state.is_scanning.lock().map_err(|e| e.to_string())?;

    Ok(IndexStatus {
        total_files: total,
        indexed_files: indexed,
        is_scanning,
    })
}

/// Spawns a background indexing task for the given directory path.
/// Returns immediately; use `get_index_status` to poll progress.
#[tauri::command]
pub fn start_scan(path: String, state: State<'_, AppState>) -> Result<String, String> {
    // Prevent concurrent scans.
    {
        let scanning = state.is_scanning.lock().map_err(|e| e.to_string())?;
        if *scanning {
            return Err("A scan is already in progress".to_string());
        }
    }

    let scan_path = PathBuf::from(&path);
    if !scan_path.exists() || !scan_path.is_dir() {
        return Err(format!("Path does not exist or is not a directory: {}", path));
    }

    // Clone the Arc-wrapped state fields for the background thread.
    let db_arc = state.db_arc();
    let search_arc = state.search_arc();
    let scanning_arc = state.scanning_arc();

    std::thread::spawn(move || {
        // Mark scanning as active.
        if let Ok(mut scanning) = scanning_arc.lock() {
            *scanning = true;
        }

        let result = {
            let db = db_arc.lock();
            let mut search_index = search_arc.lock();
            match (db, search_index) {
                (Ok(db), Ok(ref mut search_index)) => {
                    let mut pipeline = IndexPipeline::new(search_index, &db);
                    pipeline.run_scan(&scan_path)
                }
                _ => {
                    tracing::error!("Failed to acquire locks for scan");
                    return;
                }
            }
        };

        match result {
            Ok(progress) => {
                tracing::info!(
                    "Scan complete: {} discovered, {} indexed, {} failed",
                    progress.total_discovered,
                    progress.indexed,
                    progress.failed,
                );
            }
            Err(e) => {
                tracing::error!("Scan failed: {}", e);
            }
        }

        // Mark scanning as finished.
        if let Ok(mut scanning) = scanning_arc.lock() {
            *scanning = false;
        }
    });

    Ok(format!("Scan started for: {}", path))
}
