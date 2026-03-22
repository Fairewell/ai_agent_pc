use std::collections::HashSet;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Directories that should always be skipped during scanning.
const SKIP_DIRS: &[&str] = &[
    "$Recycle.Bin",
    "$RECYCLE.BIN",
    "node_modules",
    ".git",
    ".hg",
    ".svn",
    "Windows",
    "System32",
    "SysWOW64",
    "__pycache__",
    ".tox",
    ".venv",
    "venv",
    "target",
    "dist",
    "build",
    ".cache",
    ".npm",
    ".cargo",
];

/// Returns an iterator over file paths under `path` whose extensions are in
/// `extensions`. Hidden directories and common junk directories are skipped.
pub fn scan_directory<'a>(
    path: &Path,
    extensions: &'a HashSet<String>,
) -> impl Iterator<Item = PathBuf> + 'a {
    let walker = WalkDir::new(path)
        .follow_links(false)
        .into_iter()
        .filter_entry(|entry| {
            let name = entry.file_name().to_string_lossy();

            // Skip hidden directories (but not the root itself).
            if entry.depth() > 0 && name.starts_with('.') {
                return false;
            }

            // Skip well-known junk directories.
            if entry.file_type().is_dir() {
                if SKIP_DIRS.iter().any(|&skip| name == skip) {
                    return false;
                }
            }

            true
        });

    walker.filter_map(move |entry| {
        let entry = entry.ok()?;
        if !entry.file_type().is_file() {
            return None;
        }

        let ext = entry
            .path()
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_lowercase())
            .unwrap_or_default();

        if extensions.contains(&ext) {
            Some(entry.into_path())
        } else {
            None
        }
    })
}
