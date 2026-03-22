use std::sync::{Arc, Mutex};

use rusqlite::Connection;

use crate::search::tantivy_index::SearchIndex;

/// Shared application state managed by Tauri. All fields are wrapped in
/// `Arc<Mutex<..>>` so they can be safely shared across command invocations
/// and background threads.
pub struct AppState {
    pub db: Arc<Mutex<Connection>>,
    pub search_index: Arc<Mutex<SearchIndex>>,
    pub is_scanning: Arc<Mutex<bool>>,
    pub http_client: reqwest::Client,
}

impl AppState {
    pub fn new(db: Connection, search_index: SearchIndex, http_client: reqwest::Client) -> Self {
        Self {
            db: Arc::new(Mutex::new(db)),
            search_index: Arc::new(Mutex::new(search_index)),
            is_scanning: Arc::new(Mutex::new(false)),
            http_client,
        }
    }

    /// Returns a clone of the database Arc (for passing to background threads).
    pub fn db_arc(&self) -> Arc<Mutex<Connection>> {
        Arc::clone(&self.db)
    }

    /// Returns a clone of the search index Arc.
    pub fn search_arc(&self) -> Arc<Mutex<SearchIndex>> {
        Arc::clone(&self.search_index)
    }

    /// Returns a clone of the scanning flag Arc.
    pub fn scanning_arc(&self) -> Arc<Mutex<bool>> {
        Arc::clone(&self.is_scanning)
    }

    /// Returns a reference to the shared HTTP client.
    pub fn http_client(&self) -> &reqwest::Client {
        &self.http_client
    }
}
