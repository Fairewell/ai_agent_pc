use anyhow::Result;
use rusqlite::Connection;
use std::path::Path;

/// Opens (or creates) the SQLite database at the given path and ensures all
/// tables and indexes exist.
pub fn init_database(path: &Path) -> Result<Connection> {
    let conn = Connection::open(path)?;

    // Enable WAL mode for better concurrent read performance.
    conn.execute_batch("PRAGMA journal_mode=WAL;")?;
    conn.execute_batch("PRAGMA foreign_keys=ON;")?;

    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS files (
            id            INTEGER PRIMARY KEY AUTOINCREMENT,
            path          TEXT    NOT NULL UNIQUE,
            filename      TEXT    NOT NULL,
            extension     TEXT    NOT NULL DEFAULT '',
            size_bytes    INTEGER NOT NULL DEFAULT 0,
            modified_at   TEXT    NOT NULL DEFAULT '',
            created_at    TEXT    NOT NULL DEFAULT '',
            blake3_hash   TEXT    NOT NULL DEFAULT '',
            index_status  TEXT    NOT NULL DEFAULT 'pending',
            drive         TEXT    NOT NULL DEFAULT ''
        );

        CREATE INDEX IF NOT EXISTS idx_files_path       ON files(path);
        CREATE INDEX IF NOT EXISTS idx_files_extension   ON files(extension);
        CREATE INDEX IF NOT EXISTS idx_files_status      ON files(index_status);
        CREATE INDEX IF NOT EXISTS idx_files_drive       ON files(drive);

        CREATE TABLE IF NOT EXISTS file_metadata (
            id       INTEGER PRIMARY KEY AUTOINCREMENT,
            file_id  INTEGER NOT NULL,
            key      TEXT    NOT NULL,
            value    TEXT    NOT NULL DEFAULT '',
            FOREIGN KEY (file_id) REFERENCES files(id) ON DELETE CASCADE
        );

        CREATE INDEX IF NOT EXISTS idx_file_metadata_file_id ON file_metadata(file_id);

        CREATE TABLE IF NOT EXISTS drives (
            id            INTEGER PRIMARY KEY AUTOINCREMENT,
            path          TEXT    NOT NULL UNIQUE,
            label         TEXT    NOT NULL DEFAULT '',
            total_bytes   INTEGER NOT NULL DEFAULT 0,
            free_bytes    INTEGER NOT NULL DEFAULT 0,
            last_scanned  TEXT
        );

        CREATE TABLE IF NOT EXISTS settings (
            key   TEXT PRIMARY KEY,
            value TEXT NOT NULL DEFAULT ''
        );
        ",
    )?;

    Ok(conn)
}
