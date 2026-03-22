use anyhow::{bail, Result};
use rusqlite::Connection;
use serde::Serialize;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter};

use crate::db::models::ProviderConfig;
use crate::search::tantivy_index::SearchIndex;

use super::provider::{ChatCompletionMessage, LlmClient};
use super::streaming::EVENT_AGENT_STEP;
use super::tools;

/// Maximum number of tool-use iterations before forcing a final answer.
const MAX_AGENT_STEPS: usize = 10;

/// Payload emitted for each agent reasoning step.
#[derive(Clone, Serialize)]
pub struct AgentStepPayload {
    pub session_id: i64,
    pub step: usize,
    pub tool_name: String,
    pub tool_args: String,
    pub tool_result: String,
}

/// The agent executor runs an agentic loop: it sends a prompt to the LLM,
/// checks if the LLM wants to call a tool, executes the tool, feeds the
/// result back, and repeats until the LLM produces a final answer.
pub struct AgentExecutor;

impl AgentExecutor {
    pub async fn execute(
        search_index: &Arc<Mutex<SearchIndex>>,
        db: &Arc<Mutex<Connection>>,
        http_client: &reqwest::Client,
        config: &ProviderConfig,
        user_prompt: &str,
        app: &AppHandle,
        session_id: i64,
        message_id: i64,
    ) -> Result<String> {
        // Build the system prompt with tool descriptions.
        let system_prompt = format!(
            "You are an intelligent agent that can use tools to help answer questions about the user's documents.\n\n{}\n\
             Think step-by-step. Use tools to gather information before answering.\n\
             When you have gathered enough information, provide your final answer directly without any tool calls.",
            tools::tools_prompt()
        );

        let mut messages = vec![
            ChatCompletionMessage {
                role: "system".to_string(),
                content: system_prompt,
            },
            ChatCompletionMessage {
                role: "user".to_string(),
                content: user_prompt.to_string(),
            },
        ];

        for step in 0..MAX_AGENT_STEPS {
            // For intermediate steps, use non-streaming completion.
            let response = LlmClient::complete(http_client, config, &messages).await?;

            // Check if the response contains a tool call.
            if let Some((tool_name, tool_args)) = parse_tool_call(&response) {
                // Execute the tool.
                let tool_result = match tools::execute_tool(
                    &tool_name,
                    &tool_args,
                    search_index,
                    db,
                ) {
                    Ok(result) => result,
                    Err(e) => format!("Tool error: {}", e),
                };

                // Emit an agent step event.
                let _ = app.emit(
                    EVENT_AGENT_STEP,
                    AgentStepPayload {
                        session_id,
                        step: step + 1,
                        tool_name: tool_name.clone(),
                        tool_args: tool_args.to_string(),
                        tool_result: tool_result.clone(),
                    },
                );

                // Append the assistant's response (with the tool call) and the
                // tool result to the conversation.
                messages.push(ChatCompletionMessage {
                    role: "assistant".to_string(),
                    content: response,
                });
                messages.push(ChatCompletionMessage {
                    role: "tool".to_string(),
                    content: format!("[Tool: {}] Result:\n{}", tool_name, tool_result),
                });

                continue;
            }

            // No tool call — this is the final answer. Stream it to the user.
            // We re-send the messages with the final answer context to get a
            // streamed version.
            messages.push(ChatCompletionMessage {
                role: "assistant".to_string(),
                content: response.clone(),
            });

            // Stream the final response.
            let final_response = LlmClient::complete_streaming(
                http_client,
                config,
                &messages[..messages.len() - 1], // up to but not including the final
                app,
                session_id,
                message_id,
            )
            .await?;

            return Ok(final_response);
        }

        bail!("Agent exceeded maximum number of steps ({})", MAX_AGENT_STEPS)
    }
}

/// Parses a tool call from the LLM response.
/// Expected format: `<tool_call>{"name":"tool_name","args":{...}}</tool_call>`
fn parse_tool_call(response: &str) -> Option<(String, serde_json::Value)> {
    let start_tag = "<tool_call>";
    let end_tag = "</tool_call>";

    let start = response.find(start_tag)?;
    let end = response.find(end_tag)?;

    if end <= start {
        return None;
    }

    let json_str = &response[start + start_tag.len()..end].trim();
    let parsed: serde_json::Value = serde_json::from_str(json_str).ok()?;

    let name = parsed["name"].as_str()?.to_string();
    let args = parsed.get("args").cloned().unwrap_or(serde_json::Value::Object(Default::default()));

    Some((name, args))
}
