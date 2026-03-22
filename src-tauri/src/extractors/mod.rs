pub mod plaintext;

use anyhow::{bail, Result};
use std::collections::HashMap;
use std::path::Path;

/// Result of extracting text from a file.
#[derive(Debug, Clone)]
pub struct ExtractionResult {
    pub text: String,
    pub metadata: HashMap<String, String>,
}

/// Trait that all text extractors implement.
pub trait TextExtractor {
    fn extract(&self, path: &Path) -> Result<ExtractionResult>;
}

/// Dispatches to the appropriate extractor based on file extension.
pub fn extract_file(path: &Path) -> Result<ExtractionResult> {
    let extension = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    if plaintext::can_handle(&extension) {
        return plaintext::extract(path);
    }

    bail!(
        "No extractor available for extension '{}': {}",
        extension,
        path.display()
    )
}
