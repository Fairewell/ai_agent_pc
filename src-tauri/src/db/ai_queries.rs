use anyhow::Result;
use rusqlite::{params, Connection};

use super::models::{ChatMessage, ChatSession, ProviderConfig};

// ---------------------------------------------------------------------------
// Provider CRUD
// ---------------------------------------------------------------------------

/// Inserts a new provider and returns its row id.
pub fn insert_provider(conn: &Connection, p: &ProviderConfig) -> Result<i64> {
    conn.execute(
        "INSERT INTO providers (name, base_url, api_key, model_name, max_tokens, temperature, is_default)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            p.name,
            p.base_url,
            p.api_key,
            p.model_name,
            p.max_tokens,
            p.temperature,
            p.is_default as i32,
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

/// Returns all provider configurations.
pub fn get_all_providers(conn: &Connection) -> Result<Vec<ProviderConfig>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, base_url, api_key, model_name, max_tokens, temperature, is_default
         FROM providers ORDER BY id",
    )?;
    let rows = stmt.query_map([], |row| {
        Ok(ProviderConfig {
            id: Some(row.get(0)?),
            name: row.get(1)?,
            base_url: row.get(2)?,
            api_key: row.get(3)?,
            model_name: row.get(4)?,
            max_tokens: row.get(5)?,
            temperature: row.get(6)?,
            is_default: row.get::<_, i32>(7)? != 0,
        })
    })?;
    let mut results = Vec::new();
    for row in rows {
        results.push(row?);
    }
    Ok(results)
}

/// Returns a provider by id.
pub fn get_provider_by_id(conn: &Connection, id: i64) -> Result<Option<ProviderConfig>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, base_url, api_key, model_name, max_tokens, temperature, is_default
         FROM providers WHERE id = ?1",
    )?;
    let mut rows = stmt.query_map(params![id], |row| {
        Ok(ProviderConfig {
            id: Some(row.get(0)?),
            name: row.get(1)?,
            base_url: row.get(2)?,
            api_key: row.get(3)?,
            model_name: row.get(4)?,
            max_tokens: row.get(5)?,
            temperature: row.get(6)?,
            is_default: row.get::<_, i32>(7)? != 0,
        })
    })?;
    match rows.next() {
        Some(Ok(record)) => Ok(Some(record)),
        Some(Err(e)) => Err(e.into()),
        None => Ok(None),
    }
}

/// Returns the default provider (is_default = 1), if any.
pub fn get_default_provider(conn: &Connection) -> Result<Option<ProviderConfig>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, base_url, api_key, model_name, max_tokens, temperature, is_default
         FROM providers WHERE is_default = 1 LIMIT 1",
    )?;
    let mut rows = stmt.query_map([], |row| {
        Ok(ProviderConfig {
            id: Some(row.get(0)?),
            name: row.get(1)?,
            base_url: row.get(2)?,
            api_key: row.get(3)?,
            model_name: row.get(4)?,
            max_tokens: row.get(5)?,
            temperature: row.get(6)?,
            is_default: row.get::<_, i32>(7)? != 0,
        })
    })?;
    match rows.next() {
        Some(Ok(record)) => Ok(Some(record)),
        Some(Err(e)) => Err(e.into()),
        None => Ok(None),
    }
}

/// Updates an existing provider.
pub fn update_provider(conn: &Connection, p: &ProviderConfig) -> Result<()> {
    let id = p.id.ok_or_else(|| anyhow::anyhow!("Provider id is required for update"))?;
    conn.execute(
        "UPDATE providers SET name = ?1, base_url = ?2, api_key = ?3, model_name = ?4,
         max_tokens = ?5, temperature = ?6, is_default = ?7, updated_at = datetime('now')
         WHERE id = ?8",
        params![
            p.name,
            p.base_url,
            p.api_key,
            p.model_name,
            p.max_tokens,
            p.temperature,
            p.is_default as i32,
            id,
        ],
    )?;
    Ok(())
}

/// Deletes a provider by id.
pub fn delete_provider(conn: &Connection, id: i64) -> Result<()> {
    conn.execute("DELETE FROM providers WHERE id = ?1", params![id])?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Chat Session CRUD
// ---------------------------------------------------------------------------

/// Creates a new chat session and returns its id.
pub fn create_chat_session(conn: &Connection, provider_id: Option<i64>) -> Result<i64> {
    conn.execute(
        "INSERT INTO chat_sessions (title, provider_id) VALUES ('New Chat', ?1)",
        params![provider_id],
    )?;
    Ok(conn.last_insert_rowid())
}

/// Lists all chat sessions ordered by most recent first.
pub fn list_all_chat_sessions(conn: &Connection) -> Result<Vec<ChatSession>> {
    let mut stmt = conn.prepare(
        "SELECT id, title, provider_id, created_at, updated_at
         FROM chat_sessions ORDER BY updated_at DESC",
    )?;
    let rows = stmt.query_map([], |row| {
        Ok(ChatSession {
            id: Some(row.get(0)?),
            title: row.get(1)?,
            provider_id: row.get(2)?,
            created_at: row.get(3)?,
            updated_at: row.get(4)?,
        })
    })?;
    let mut results = Vec::new();
    for row in rows {
        results.push(row?);
    }
    Ok(results)
}

/// Gets a single chat session by id.
pub fn get_chat_session_by_id(conn: &Connection, id: i64) -> Result<Option<ChatSession>> {
    let mut stmt = conn.prepare(
        "SELECT id, title, provider_id, created_at, updated_at
         FROM chat_sessions WHERE id = ?1",
    )?;
    let mut rows = stmt.query_map(params![id], |row| {
        Ok(ChatSession {
            id: Some(row.get(0)?),
            title: row.get(1)?,
            provider_id: row.get(2)?,
            created_at: row.get(3)?,
            updated_at: row.get(4)?,
        })
    })?;
    match rows.next() {
        Some(Ok(record)) => Ok(Some(record)),
        Some(Err(e)) => Err(e.into()),
        None => Ok(None),
    }
}

/// Updates the title of a chat session.
pub fn update_chat_session_title(conn: &Connection, id: i64, title: &str) -> Result<()> {
    conn.execute(
        "UPDATE chat_sessions SET title = ?1, updated_at = datetime('now') WHERE id = ?2",
        params![title, id],
    )?;
    Ok(())
}

/// Deletes a chat session and its messages (cascade).
pub fn delete_chat_session(conn: &Connection, id: i64) -> Result<()> {
    conn.execute("DELETE FROM chat_sessions WHERE id = ?1", params![id])?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Chat Message CRUD
// ---------------------------------------------------------------------------

/// Inserts a chat message and returns its id.
pub fn insert_chat_message(conn: &Connection, msg: &ChatMessage) -> Result<i64> {
    conn.execute(
        "INSERT INTO chat_messages (session_id, role, content, attached_files, tool_calls)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            msg.session_id,
            msg.role,
            msg.content,
            msg.attached_files,
            msg.tool_calls,
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

/// Returns all messages for a given session, ordered chronologically.
pub fn get_messages_by_session_id(conn: &Connection, session_id: i64) -> Result<Vec<ChatMessage>> {
    let mut stmt = conn.prepare(
        "SELECT id, session_id, role, content, attached_files, tool_calls, created_at
         FROM chat_messages WHERE session_id = ?1 ORDER BY id ASC",
    )?;
    let rows = stmt.query_map(params![session_id], |row| {
        Ok(ChatMessage {
            id: Some(row.get(0)?),
            session_id: row.get(1)?,
            role: row.get(2)?,
            content: row.get(3)?,
            attached_files: row.get(4)?,
            tool_calls: row.get(5)?,
            created_at: row.get(6)?,
        })
    })?;
    let mut results = Vec::new();
    for row in rows {
        results.push(row?);
    }
    Ok(results)
}

/// Deletes all messages in a session.
pub fn delete_messages_by_session_id(conn: &Connection, session_id: i64) -> Result<()> {
    conn.execute(
        "DELETE FROM chat_messages WHERE session_id = ?1",
        params![session_id],
    )?;
    Ok(())
}
