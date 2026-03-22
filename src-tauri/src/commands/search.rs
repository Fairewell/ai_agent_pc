use serde::{Deserialize, Serialize};
use tauri::State;

use crate::state::AppState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchFilters {
    pub extensions: Option<Vec<String>>,
    pub drive: Option<String>,
    pub max_results: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: i64,
    pub path: String,
    pub filename: String,
    pub extension: String,
    pub size_bytes: i64,
    pub modified_at: String,
    pub snippet: String,
}

/// Queries the Tantivy index and joins with SQLite metadata to produce full
/// search results.
#[tauri::command]
pub fn search_files(
    query: String,
    filters: SearchFilters,
    state: State<'_, AppState>,
) -> Result<Vec<SearchResult>, String> {
    let limit = filters.max_results.unwrap_or(50);

    // Search Tantivy for matching paths + snippets.
    let search_index = state.search_index.lock().map_err(|e| e.to_string())?;
    let hits = search_index
        .search(&query, limit)
        .map_err(|e| e.to_string())?;

    if hits.is_empty() {
        return Ok(Vec::new());
    }

    // Look up full metadata from SQLite for the matched paths.
    let db = state.db.lock().map_err(|e| e.to_string())?;

    let mut results = Vec::with_capacity(hits.len());
    for hit in &hits {
        let record = crate::db::queries::get_file_by_path(&db, &hit.path)
            .map_err(|e| e.to_string())?;

        if let Some(record) = record {
            // Apply extension filter.
            if let Some(ref exts) = filters.extensions {
                if !exts.is_empty() && !exts.contains(&record.extension) {
                    continue;
                }
            }
            // Apply drive filter.
            if let Some(ref drive) = filters.drive {
                if !drive.is_empty() && record.drive != *drive {
                    continue;
                }
            }

            results.push(SearchResult {
                id: record.id.unwrap_or(0),
                path: record.path,
                filename: record.filename,
                extension: record.extension,
                size_bytes: record.size_bytes,
                modified_at: record.modified_at,
                snippet: hit.snippet.clone(),
            });
        }
    }

    Ok(results)
}
