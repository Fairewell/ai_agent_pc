use anyhow::Result;
use rusqlite::Connection;
use std::path::Path;
use std::sync::{Arc, Mutex};
use tauri::AppHandle;

use crate::db::models::ProviderConfig;
use crate::extractors;
use crate::search::tantivy_index::SearchIndex;

use super::provider::{ChatCompletionMessage, LlmClient};

/// RAG (Retrieval-Augmented Generation) pipeline that searches indexed documents,
/// builds a context-enriched prompt, and streams the response.
pub struct RagPipeline;

impl RagPipeline {
    /// Executes a RAG query:
    /// 1. Searches the Tantivy index for relevant documents
    /// 2. Extracts text from top results
    /// 3. Builds a system prompt with document context
    /// 4. Streams the LLM response back to the frontend
    pub async fn query_with_context(
        search_index: &Arc<Mutex<SearchIndex>>,
        db: &Arc<Mutex<Connection>>,
        http_client: &reqwest::Client,
        config: &ProviderConfig,
        user_query: &str,
        app: &AppHandle,
        session_id: i64,
        message_id: i64,
    ) -> Result<String> {
        // Step 1: Search for relevant documents.
        let hits = {
            let index = search_index
                .lock()
                .map_err(|e| anyhow::anyhow!("Failed to lock search index: {}", e))?;
            index.search(user_query, 5)?
        };

        // Step 2: Extract text from each hit.
        let mut context_parts = Vec::new();
        for hit in &hits {
            let path = Path::new(&hit.path);
            match extractors::extract_file(path) {
                Ok(extraction) => {
                    let text = if extraction.text.len() > 2000 {
                        format!("{}...", &extraction.text[..2000])
                    } else {
                        extraction.text
                    };
                    context_parts.push(format!(
                        "--- Document: {} ---\n{}\n",
                        hit.path, text
                    ));
                }
                Err(e) => {
                    tracing::warn!("Failed to extract text from {}: {}", hit.path, e);
                    // Fall back to the snippet from the search result.
                    if !hit.snippet.is_empty() {
                        context_parts.push(format!(
                            "--- Document: {} ---\n{}\n",
                            hit.path, hit.snippet
                        ));
                    }
                }
            }
        }

        // Step 3: Build system prompt with context.
        let system_prompt = if context_parts.is_empty() {
            "You are a helpful assistant. The user asked a question but no relevant documents were found in their library. \
             Answer to the best of your ability and let them know that no matching documents were found."
                .to_string()
        } else {
            format!(
                "You are a helpful assistant with access to the user's document library. \
                 Use the following document excerpts to answer the user's question. \
                 Cite the document paths when referencing specific information.\n\n{}\n\
                 If the documents don't contain enough information to fully answer the question, \
                 say so and provide what you can based on the available context.",
                context_parts.join("\n")
            )
        };

        // Step 4: Build messages and stream response.
        let messages = vec![
            ChatCompletionMessage {
                role: "system".to_string(),
                content: system_prompt,
            },
            ChatCompletionMessage {
                role: "user".to_string(),
                content: user_query.to_string(),
            },
        ];

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
}
