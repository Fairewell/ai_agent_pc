use anyhow::{Context, Result};
use rusqlite::Connection;
use std::collections::HashSet;
use std::path::Path;

use crate::db::models::FileRecord;
use crate::db::queries;
use crate::extractors;
use crate::search::tantivy_index::SearchIndex;
use crate::utils::supported_extensions;

use super::scanner::scan_directory;

/// Progress information returned after a scan completes.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ScanProgress {
    pub total_discovered: u64,
    pub indexed: u64,
    pub failed: u64,
}

/// The indexing pipeline that coordinates scanning, text extraction, and
/// insertion into both SQLite and Tantivy.
pub struct IndexPipeline<'a> {
    pub search_index: &'a mut SearchIndex,
    pub db: &'a Connection,
    pub extensions: HashSet<String>,
}

impl<'a> IndexPipeline<'a> {
    /// Creates a new pipeline with the default set of supported extensions.
    pub fn new(search_index: &'a mut SearchIndex, db: &'a Connection) -> Self {
        Self {
            search_index,
            db,
            extensions: supported_extensions(),
        }
    }

    /// Processes a single file: extracts text, computes hash, writes to SQLite
    /// and Tantivy. Returns `Ok(true)` on success, `Ok(false)` if the file was
    /// already indexed with the same hash.
    pub fn process_file(&mut self, path: &Path) -> Result<bool> {
        let metadata = std::fs::metadata(path)
            .with_context(|| format!("Failed to read metadata for {}", path.display()))?;

        let size_bytes = metadata.len() as i64;

        let modified_at = metadata
            .modified()
            .ok()
            .and_then(|t| {
                let dt: chrono::DateTime<chrono::Utc> = t.into();
                Some(dt.format("%Y-%m-%dT%H:%M:%SZ").to_string())
            })
            .unwrap_or_default();

        let created_at = metadata
            .created()
            .ok()
            .and_then(|t| {
                let dt: chrono::DateTime<chrono::Utc> = t.into();
                Some(dt.format("%Y-%m-%dT%H:%M:%SZ").to_string())
            })
            .unwrap_or_default();

        let filename = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();

        let extension = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();

        // Compute blake3 hash of file contents.
        let file_bytes = std::fs::read(path)
            .with_context(|| format!("Failed to read file: {}", path.display()))?;
        let hash = blake3::hash(&file_bytes).to_hex().to_string();

        // Check if already indexed with same hash.
        let path_str = path.to_string_lossy().to_string();
        if let Some(existing) = queries::get_file_by_path(self.db, &path_str)? {
            if existing.blake3_hash == hash && existing.index_status == "indexed" {
                return Ok(false); // Already up to date.
            }
        }

        // Determine drive (root component).
        let drive = path
            .components()
            .next()
            .map(|c| c.as_os_str().to_string_lossy().to_string())
            .unwrap_or_default();

        let record = FileRecord {
            id: None,
            path: path_str.clone(),
            filename: filename.clone(),
            extension: extension.clone(),
            size_bytes,
            modified_at: modified_at.clone(),
            created_at,
            blake3_hash: hash,
            index_status: "pending".to_string(),
            drive,
        };

        let file_id = queries::insert_file(self.db, &record)?;

        // Extract text content.
        match extractors::extract_file(path) {
            Ok(extraction) => {
                self.search_index.add_document(
                    &path_str,
                    &filename,
                    &extraction.text,
                    &extension,
                    &modified_at,
                )?;
                queries::update_index_status(self.db, file_id, "indexed")?;
            }
            Err(e) => {
                tracing::warn!("Extraction failed for {}: {}", path.display(), e);
                queries::update_index_status(self.db, file_id, "failed")?;
            }
        }

        Ok(true)
    }

    /// Runs a full scan of the given directory, processing each discovered file.
    pub fn run_scan(&mut self, dir: &Path) -> Result<ScanProgress> {
        let mut progress = ScanProgress {
            total_discovered: 0,
            indexed: 0,
            failed: 0,
        };

        let paths: Vec<_> = scan_directory(dir, &self.extensions).collect();
        progress.total_discovered = paths.len() as u64;

        for path in paths {
            match self.process_file(&path) {
                Ok(true) => progress.indexed += 1,
                Ok(false) => {
                    // Already indexed, still count as success.
                    progress.indexed += 1;
                }
                Err(e) => {
                    tracing::warn!("Failed to process {}: {}", path.display(), e);
                    progress.failed += 1;
                }
            }
        }

        // Commit Tantivy index after the scan.
        self.search_index.commit()?;

        Ok(progress)
    }
}
