use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileRecord {
    pub id: Option<i64>,
    pub path: String,
    pub filename: String,
    pub extension: String,
    pub size_bytes: i64,
    pub modified_at: String,
    pub created_at: String,
    pub blake3_hash: String,
    pub index_status: String, // "pending", "indexed", "failed"
    pub drive: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetadata {
    pub id: Option<i64>,
    pub file_id: i64,
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriveRecord {
    pub id: Option<i64>,
    pub path: String,
    pub label: String,
    pub total_bytes: i64,
    pub free_bytes: i64,
    pub last_scanned: Option<String>,
}

// ---------------------------------------------------------------------------
// AI-related models
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub id: Option<i64>,
    pub name: String,
    pub base_url: String,
    pub api_key: String,
    pub model_name: String,
    pub max_tokens: i32,
    pub temperature: f64,
    pub is_default: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatSession {
    pub id: Option<i64>,
    pub title: String,
    pub provider_id: Option<i64>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: Option<i64>,
    pub session_id: i64,
    pub role: String,
    pub content: String,
    pub attached_files: String, // JSON array
    pub tool_calls: String,    // JSON array
    pub created_at: String,
}
