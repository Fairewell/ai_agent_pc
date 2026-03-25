use anyhow::{bail, Result};
use rusqlite::Connection;
use std::sync::{Arc, Mutex};
use tauri::AppHandle;

use crate::db::ai_queries;
use crate::db::models::{ChatMessage, ProviderConfig};

use super::provider::{ChatCompletionMessage, LlmClient};

/// Resolves the provider configuration for a chat session.
/// If the session has a provider_id, uses that; otherwise falls back to the
/// default provider. Returns an error if no provider is configured.
pub fn resolve_provider(
    db: &Arc<Mutex<Connection>>,
    provider_id: Option<i64>,
) -> Result<ProviderConfig> {
    let conn = db
        .lock()
        .map_err(|e| anyhow::anyhow!("Failed to lock database: {}", e))?;

    // Try session-specific provider first.
    if let Some(pid) = provider_id {
        if let Some(provider) = ai_queries::get_provider_by_id(&conn, pid)? {
            return Ok(provider);
        }
    }

    // Fall back to default provider.
    if let Some(provider) = ai_queries::get_default_provider(&conn)? {
        return Ok(provider);
    }

    // Fall back to any provider at all.
    let all = ai_queries::get_all_providers(&conn)?;
    if let Some(provider) = all.into_iter().next() {
        return Ok(provider);
    }

    bail!("No LLM provider configured. Please add a provider in Settings.")
}

/// Loads chat history from the database and converts it to the format expected
/// by the LLM API.
pub fn load_chat_history(
    db: &Arc<Mutex<Connection>>,
    session_id: i64,
) -> Result<Vec<ChatCompletionMessage>> {
    let conn = db
        .lock()
        .map_err(|e| anyhow::anyhow!("Failed to lock database: {}", e))?;

    let messages = ai_queries::get_messages_by_session_id(&conn, session_id)?;

    let api_messages: Vec<ChatCompletionMessage> = messages
        .iter()
        .filter(|m| m.role == "user" || m.role == "assistant" || m.role == "system")
        .map(|m| ChatCompletionMessage {
            role: m.role.clone(),
            content: m.content.clone(),
        })
        .collect();

    Ok(api_messages)
}

/// Saves a chat message to the database and returns the new message id.
pub fn save_message(
    db: &Arc<Mutex<Connection>>,
    session_id: i64,
    role: &str,
    content: &str,
    attached_files: &[String],
) -> Result<i64> {
    let conn = db
        .lock()
        .map_err(|e| anyhow::anyhow!("Failed to lock database: {}", e))?;

    let attached_json = serde_json::to_string(attached_files).unwrap_or_else(|_| "[]".to_string());

    let msg = ChatMessage {
        id: None,
        session_id,
        role: role.to_string(),
        content: content.to_string(),
        attached_files: attached_json,
        tool_calls: "[]".to_string(),
        created_at: String::new(), // DB will set via default
    };

    let id = ai_queries::insert_chat_message(&conn, &msg)?;
    Ok(id)
}

/// Runs a standard (non-RAG, non-agent) chat completion with streaming.
pub async fn run_chat_completion(
    http_client: &reqwest::Client,
    config: &ProviderConfig,
    history: &[ChatCompletionMessage],
    user_message: &str,
    app: &AppHandle,
    session_id: i64,
    message_id: i64,
) -> Result<String> {
    let mut messages = history.to_vec();
    messages.push(ChatCompletionMessage {
        role: "user".to_string(),
        content: user_message.to_string(),
    });

    let response = LlmClient::complete_streaming(
        http_client,
        config,
        &messages,
        app,
        session_id,
        message_id,
    )
    .await?;

    Ok(response)
}
