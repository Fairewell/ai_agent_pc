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
