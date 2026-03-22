mod commands;
mod db;
mod extractors;
mod indexer;
mod search;
pub mod state;
mod utils;

use std::path::PathBuf;

use crate::commands::{get_file_detail, get_index_status, open_file, search_files, start_scan};
use crate::db::schema::init_database;
use crate::search::tantivy_index::SearchIndex;
use crate::state::AppState;

/// Returns the application data directory: `~/.ai-agent-pc/`.
fn data_dir() -> PathBuf {
    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    home.join(".ai-agent-pc")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialise logging.
    tracing_subscriber::fmt::init();

    // Ensure data directory exists.
    let data = data_dir();
    std::fs::create_dir_all(&data).expect("Failed to create data directory");

    // Initialise SQLite database.
    let db_path = data.join("index.db");
    let db = init_database(&db_path).expect("Failed to initialise SQLite database");

    // Initialise Tantivy search index.
    let tantivy_path = data.join("tantivy");
    let search_index =
        SearchIndex::new(&tantivy_path).expect("Failed to initialise Tantivy index");

    // Build shared state.
    let app_state = AppState::new(db, search_index);

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            search_files,
            get_index_status,
            start_scan,
            open_file,
            get_file_detail,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
