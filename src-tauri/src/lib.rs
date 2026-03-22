mod ai;
mod commands;
mod db;
mod extractors;
mod indexer;
mod search;
pub mod state;
mod utils;

use std::path::PathBuf;

use crate::commands::{
    add_provider, agent_execute, create_chat_session, delete_chat_session, delete_provider,
    get_chat_messages, get_file_detail, get_index_status, get_providers, list_chat_sessions,
    open_file, search_files, send_chat_message, start_scan, test_provider, transform_document,
    update_provider,
};
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

    // Build shared HTTP client for LLM API calls.
    let http_client = reqwest::Client::new();

    // Build shared state.
    let app_state = AppState::new(db, search_index, http_client);

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            search_files,
            get_index_status,
            start_scan,
            open_file,
            get_file_detail,
            // AI commands
            get_providers,
            add_provider,
            update_provider,
            delete_provider,
            test_provider,
            create_chat_session,
            list_chat_sessions,
            delete_chat_session,
            get_chat_messages,
            send_chat_message,
            transform_document,
            agent_execute,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
