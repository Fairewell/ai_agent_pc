use tauri::State;

use crate::ai::chat;
use crate::ai::agent::AgentExecutor;
use crate::ai::provider::{ChatCompletionMessage, LlmClient};
use crate::ai::rag::RagPipeline;
use crate::db::ai_queries;
use crate::db::models::{ChatMessage, ChatSession, ProviderConfig};
use crate::extractors;
use crate::state::AppState;

// ---------------------------------------------------------------------------
// Provider CRUD
// ---------------------------------------------------------------------------

#[tauri::command]
pub fn get_providers(state: State<'_, AppState>) -> Result<Vec<ProviderConfig>, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    ai_queries::get_all_providers(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn add_provider(state: State<'_, AppState>, provider: ProviderConfig) -> Result<i64, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    ai_queries::insert_provider(&conn, &provider).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_provider(state: State<'_, AppState>, provider: ProviderConfig) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    ai_queries::update_provider(&conn, &provider).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_provider(state: State<'_, AppState>, id: i64) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    ai_queries::delete_provider(&conn, id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn test_provider(state: State<'_, AppState>, id: i64) -> Result<String, String> {
    let config = {
        let conn = state.db.lock().map_err(|e| e.to_string())?;
        ai_queries::get_provider_by_id(&conn, id)
            .map_err(|e| e.to_string())?
            .ok_or_else(|| format!("Provider with id {} not found", id))?
    };

    let messages = vec![ChatCompletionMessage {
        role: "user".to_string(),
        content: "Say 'Hello! Connection successful.' in exactly those words.".to_string(),
    }];

    let client = state.http_client();
    let response = LlmClient::complete(client, &config, &messages)
        .await
        .map_err(|e| e.to_string())?;

    Ok(response)
}

// ---------------------------------------------------------------------------
// Chat Session CRUD
// ---------------------------------------------------------------------------

#[tauri::command]
pub fn create_chat_session(
    state: State<'_, AppState>,
    provider_id: Option<i64>,
) -> Result<i64, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    ai_queries::create_chat_session(&conn, provider_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn list_chat_sessions(state: State<'_, AppState>) -> Result<Vec<ChatSession>, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    ai_queries::list_all_chat_sessions(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_chat_session(state: State<'_, AppState>, id: i64) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    ai_queries::delete_chat_session(&conn, id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_chat_messages(
    state: State<'_, AppState>,
    session_id: i64,
) -> Result<Vec<ChatMessage>, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    ai_queries::get_messages_by_session_id(&conn, session_id).map_err(|e| e.to_string())
}

// ---------------------------------------------------------------------------
// Chat messaging (with streaming via events)
// ---------------------------------------------------------------------------

#[tauri::command]
pub async fn send_chat_message(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    session_id: i64,
    message: String,
    attached_files: Vec<String>,
    use_rag: bool,
) -> Result<(), String> {
    // 1. Save user message to DB.
    let user_msg_id = chat::save_message(
        &state.db_arc(),
        session_id,
        "user",
        &message,
        &attached_files,
    )
    .map_err(|e| e.to_string())?;

    // 2. Get provider config.
    let session_provider_id = {
        let conn = state.db.lock().map_err(|e| e.to_string())?;
        ai_queries::get_chat_session_by_id(&conn, session_id)
            .map_err(|e| e.to_string())?
            .and_then(|s| s.provider_id)
    };

    let config = chat::resolve_provider(&state.db_arc(), session_provider_id)
        .map_err(|e| e.to_string())?;

    // 3. Create a placeholder assistant message to get its id for streaming events.
    let assistant_msg_id = chat::save_message(
        &state.db_arc(),
        session_id,
        "assistant",
        "",
        &[],
    )
    .map_err(|e| e.to_string())?;

    let client = state.http_client().clone();
    let search_index = state.search_arc();
    let db = state.db_arc();

    // 4. Run completion.
    let response = if use_rag {
        RagPipeline::query_with_context(
            &search_index,
            &db,
            &client,
            &config,
            &message,
            &app,
            session_id,
            assistant_msg_id,
        )
        .await
        .map_err(|e| e.to_string())?
    } else {
        // Load chat history and run standard completion.
        let history = chat::load_chat_history(&db, session_id)
            .map_err(|e| e.to_string())?;

        // Remove the last two messages (the user message and empty assistant
        // message we just inserted) since we'll add them ourselves.
        let history: Vec<_> = history
            .into_iter()
            .filter(|m| {
                // Keep all messages except the empty assistant placeholder
                !(m.role == "assistant" && m.content.is_empty())
            })
            .collect();

        // Remove the user message we just saved (it's the last one).
        let history_without_current: Vec<_> = if history.last().map(|m| m.content.as_str()) == Some(&message) {
            history[..history.len() - 1].to_vec()
        } else {
            history
        };

        chat::run_chat_completion(
            &client,
            &config,
            &history_without_current,
            &message,
            &app,
            session_id,
            assistant_msg_id,
        )
        .await
        .map_err(|e| e.to_string())?
    };

    // 5. Update assistant message with actual content.
    {
        let conn = db.lock().map_err(|e| e.to_string())?;
        conn.execute(
            "UPDATE chat_messages SET content = ?1 WHERE id = ?2",
            rusqlite::params![response, assistant_msg_id],
        )
        .map_err(|e| e.to_string())?;
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Document transformation
// ---------------------------------------------------------------------------

#[tauri::command]
pub async fn transform_document(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    path: String,
    prompt: String,
) -> Result<String, String> {
    // Extract text from the document.
    let extraction = extractors::extract_file(std::path::Path::new(&path))
        .map_err(|e| e.to_string())?;

    let text = if extraction.text.len() > 8000 {
        format!("{}...\n[truncated]", &extraction.text[..8000])
    } else {
        extraction.text
    };

    // Get the default provider.
    let config = chat::resolve_provider(&state.db_arc(), None)
        .map_err(|e| e.to_string())?;

    let messages = vec![
        ChatCompletionMessage {
            role: "system".to_string(),
            content: format!(
                "You are a document transformation assistant. The user will provide a prompt describing \
                 how to transform the following document. Apply the transformation and return the result.\n\n\
                 --- Document: {} ---\n{}",
                path, text
            ),
        },
        ChatCompletionMessage {
            role: "user".to_string(),
            content: prompt,
        },
    ];

    let client = state.http_client();
    let response = LlmClient::complete(client, &config, &messages)
        .await
        .map_err(|e| e.to_string())?;

    Ok(response)
}

// ---------------------------------------------------------------------------
// Agent execution
// ---------------------------------------------------------------------------

#[tauri::command]
pub async fn agent_execute(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    prompt: String,
    session_id: i64,
) -> Result<(), String> {
    // Save user message.
    let _user_msg_id = chat::save_message(
        &state.db_arc(),
        session_id,
        "user",
        &prompt,
        &[],
    )
    .map_err(|e| e.to_string())?;

    // Create assistant placeholder.
    let assistant_msg_id = chat::save_message(
        &state.db_arc(),
        session_id,
        "assistant",
        "",
        &[],
    )
    .map_err(|e| e.to_string())?;

    // Resolve provider.
    let session_provider_id = {
        let conn = state.db.lock().map_err(|e| e.to_string())?;
        ai_queries::get_chat_session_by_id(&conn, session_id)
            .map_err(|e| e.to_string())?
            .and_then(|s| s.provider_id)
    };

    let config = chat::resolve_provider(&state.db_arc(), session_provider_id)
        .map_err(|e| e.to_string())?;

    let client = state.http_client().clone();
    let search_index = state.search_arc();
    let db = state.db_arc();

    let response = AgentExecutor::execute(
        &search_index,
        &db,
        &client,
        &config,
        &prompt,
        &app,
        session_id,
        assistant_msg_id,
    )
    .await
    .map_err(|e| e.to_string())?;

    // Update assistant message with final content.
    {
        let conn = db.lock().map_err(|e| e.to_string())?;
        conn.execute(
            "UPDATE chat_messages SET content = ?1 WHERE id = ?2",
            rusqlite::params![response, assistant_msg_id],
        )
        .map_err(|e| e.to_string())?;
    }

    Ok(())
}
