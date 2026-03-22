use anyhow::Result;
use rusqlite::{params, Connection};

use super::models::FileRecord;

/// Inserts a file record and returns the new row id.
pub fn insert_file(conn: &Connection, file: &FileRecord) -> Result<i64> {
    conn.execute(
        "INSERT OR REPLACE INTO files (path, filename, extension, size_bytes, modified_at, created_at, blake3_hash, index_status, drive)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        params![
            file.path,
            file.filename,
            file.extension,
            file.size_bytes,
            file.modified_at,
            file.created_at,
            file.blake3_hash,
            file.index_status,
            file.drive,
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

/// Looks up a file record by its absolute path.
pub fn get_file_by_path(conn: &Connection, path: &str) -> Result<Option<FileRecord>> {
    let mut stmt = conn.prepare(
        "SELECT id, path, filename, extension, size_bytes, modified_at, created_at, blake3_hash, index_status, drive
         FROM files WHERE path = ?1",
    )?;

    let mut rows = stmt.query_map(params![path], |row| {
        Ok(FileRecord {
            id: Some(row.get(0)?),
            path: row.get(1)?,
            filename: row.get(2)?,
            extension: row.get(3)?,
            size_bytes: row.get(4)?,
            modified_at: row.get(5)?,
            created_at: row.get(6)?,
            blake3_hash: row.get(7)?,
            index_status: row.get(8)?,
            drive: row.get(9)?,
        })
    })?;

    match rows.next() {
        Some(Ok(record)) => Ok(Some(record)),
        Some(Err(e)) => Err(e.into()),
        None => Ok(None),
    }
}

/// Updates the index_status for a given file id.
pub fn update_index_status(conn: &Connection, id: i64, status: &str) -> Result<()> {
    conn.execute(
        "UPDATE files SET index_status = ?1 WHERE id = ?2",
        params![status, id],
    )?;
    Ok(())
}

/// Returns (total_files, indexed_files) counts.
pub fn get_index_stats(conn: &Connection) -> Result<(i64, i64)> {
    let total: i64 =
        conn.query_row("SELECT COUNT(*) FROM files", [], |row| row.get(0))?;
    let indexed: i64 = conn.query_row(
        "SELECT COUNT(*) FROM files WHERE index_status = 'indexed'",
        [],
        |row| row.get(0),
    )?;
    Ok((total, indexed))
}

/// Fetches file records for the given set of ids.
pub fn get_files_by_ids(conn: &Connection, ids: &[i64]) -> Result<Vec<FileRecord>> {
    if ids.is_empty() {
        return Ok(Vec::new());
    }

    // Build a parameterised IN clause.
    let placeholders: Vec<String> = ids.iter().map(|_| "?".to_string()).collect();
    let sql = format!(
        "SELECT id, path, filename, extension, size_bytes, modified_at, created_at, blake3_hash, index_status, drive
         FROM files WHERE id IN ({})",
        placeholders.join(", ")
    );

    let mut stmt = conn.prepare(&sql)?;
    let params: Vec<Box<dyn rusqlite::types::ToSql>> =
        ids.iter().map(|id| Box::new(*id) as Box<dyn rusqlite::types::ToSql>).collect();
    let param_refs: Vec<&dyn rusqlite::types::ToSql> = params.iter().map(|p| p.as_ref()).collect();

    let rows = stmt.query_map(param_refs.as_slice(), |row| {
        Ok(FileRecord {
            id: Some(row.get(0)?),
            path: row.get(1)?,
            filename: row.get(2)?,
            extension: row.get(3)?,
            size_bytes: row.get(4)?,
            modified_at: row.get(5)?,
            created_at: row.get(6)?,
            blake3_hash: row.get(7)?,
            index_status: row.get(8)?,
            drive: row.get(9)?,
        })
    })?;

    let mut results = Vec::new();
    for row in rows {
        results.push(row?);
    }
    Ok(results)
}
