use anyhow::{bail, Context, Result};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};

use crate::db::models::ProviderConfig;

use super::streaming::{parse_sse_line, EVENT_AI_DONE, EVENT_AI_ERROR, EVENT_AI_TOKEN};

// ---------------------------------------------------------------------------
// Message types shared with the LLM API
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionMessage {
    pub role: String,
    pub content: String,
}

// ---------------------------------------------------------------------------
// Event payloads emitted to the frontend
// ---------------------------------------------------------------------------

#[derive(Clone, Serialize, Deserialize)]
pub struct TokenPayload {
    pub session_id: i64,
    pub message_id: i64,
    pub token: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DonePayload {
    pub session_id: i64,
    pub message_id: i64,
    pub full_content: String,
    pub sources: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ErrorPayload {
    pub session_id: i64,
    pub error: String,
}

// ---------------------------------------------------------------------------
// LLM client
// ---------------------------------------------------------------------------

pub struct LlmClient;

impl LlmClient {
    /// Non-streaming chat completion. Returns the assistant message content.
    pub async fn complete(
        client: &reqwest::Client,
        config: &ProviderConfig,
        messages: &[ChatCompletionMessage],
    ) -> Result<String> {
        let url = format!("{}/v1/chat/completions", config.base_url.trim_end_matches('/'));

        let mut body = serde_json::json!({
            "model": config.model_name,
            "messages": messages,
            "max_tokens": config.max_tokens,
            "temperature": config.temperature,
            "stream": false,
        });

        let mut request = client.post(&url);

        if !config.api_key.is_empty() {
            request = request.header("Authorization", format!("Bearer {}", config.api_key));
        }

        request = request.header("Content-Type", "application/json");

        let response = request
            .json(&body)
            .send()
            .await
            .context("Failed to send request to LLM API")?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            bail!("LLM API returned {}: {}", status, text);
        }

        let json: serde_json::Value = response
            .json()
            .await
            .context("Failed to parse LLM API response")?;

        let content = json["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("")
            .to_string();

        Ok(content)
    }

    /// Streaming chat completion. Emits Tauri events for each token and returns
    /// the fully assembled response text.
    pub async fn complete_streaming(
        client: &reqwest::Client,
        config: &ProviderConfig,
        messages: &[ChatCompletionMessage],
        app: &AppHandle,
        session_id: i64,
        message_id: i64,
    ) -> Result<String> {
        let url = format!("{}/v1/chat/completions", config.base_url.trim_end_matches('/'));

        let body = serde_json::json!({
            "model": config.model_name,
            "messages": messages,
            "max_tokens": config.max_tokens,
            "temperature": config.temperature,
            "stream": true,
        });

        let mut request = client.post(&url);

        if !config.api_key.is_empty() {
            request = request.header("Authorization", format!("Bearer {}", config.api_key));
        }

        request = request.header("Content-Type", "application/json");

        let response = request
            .json(&body)
            .send()
            .await
            .context("Failed to send streaming request to LLM API")?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            let err_msg = format!("LLM API returned {}: {}", status, text);
            let _ = app.emit(
                EVENT_AI_ERROR,
                ErrorPayload {
                    session_id,
                    error: err_msg.clone(),
                },
            );
            bail!("{}", err_msg);
        }

        let mut full_content = String::new();
        let mut stream = response.bytes_stream();
        let mut buffer = String::new();

        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result.context("Error reading stream chunk")?;
            let text = String::from_utf8_lossy(&chunk);
            buffer.push_str(&text);

            // Process complete lines from the buffer.
            while let Some(newline_pos) = buffer.find('\n') {
                let line = buffer[..newline_pos].trim().to_string();
                buffer = buffer[newline_pos + 1..].to_string();

                if line.is_empty() {
                    continue;
                }

                if line == "data: [DONE]" {
                    let _ = app.emit(
                        EVENT_AI_DONE,
                        DonePayload {
                            session_id,
                            message_id,
                            full_content: full_content.clone(),
                            sources: vec![],
                        },
                    );
                    return Ok(full_content);
                }

                if let Some(token) = parse_sse_line(&line) {
                    full_content.push_str(&token);
                    let _ = app.emit(
                        EVENT_AI_TOKEN,
                        TokenPayload {
                            session_id,
                            message_id,
                            token,
                        },
                    );
                }
            }
        }

        // Stream ended without [DONE] — emit done anyway.
        let _ = app.emit(
            EVENT_AI_DONE,
            DonePayload {
                session_id,
                message_id,
                full_content: full_content.clone(),
                sources: vec![],
            },
        );

        Ok(full_content)
    }
}
