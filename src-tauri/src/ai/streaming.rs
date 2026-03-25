/// Tauri event name for individual streaming tokens.
pub const EVENT_AI_TOKEN: &str = "ai:token";

/// Tauri event name emitted when streaming is complete.
pub const EVENT_AI_DONE: &str = "ai:done";

/// Tauri event name emitted on errors.
pub const EVENT_AI_ERROR: &str = "ai:error";

/// Tauri event name emitted for each agent reasoning step.
pub const EVENT_AGENT_STEP: &str = "ai:agent_step";

/// Parses a single SSE line and extracts the content delta token, if present.
///
/// Expected format: `data: {"choices":[{"delta":{"content":"token"}}]}`
pub fn parse_sse_line(line: &str) -> Option<String> {
    let data = line.strip_prefix("data: ")?;

    // Ignore non-JSON lines (e.g., comments, empty data).
    let json: serde_json::Value = serde_json::from_str(data).ok()?;

    let content = json
        .get("choices")?
        .get(0)?
        .get("delta")?
        .get("content")?
        .as_str()?;

    if content.is_empty() {
        return None;
    }

    Some(content.to_string())
}
