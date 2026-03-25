use anyhow::{bail, Result};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::{Arc, Mutex};

use crate::extractors;
use crate::search::tantivy_index::SearchIndex;

// ---------------------------------------------------------------------------
// Tool definitions
// ---------------------------------------------------------------------------

/// Describes a tool the agent can invoke.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDef {
    pub name: &'static str,
    pub description: &'static str,
    pub parameters_schema: &'static str,
}

/// Returns the list of tools available to the agent.
pub fn available_tools() -> Vec<ToolDef> {
    vec![
        ToolDef {
            name: "search_documents",
            description: "Search the indexed document library by keyword query. Returns matching file paths, relevance scores, and text snippets.",
            parameters_schema: r#"{"type":"object","properties":{"query":{"type":"string","description":"The search query"},"max_results":{"type":"integer","description":"Maximum number of results to return (default 5)"}},"required":["query"]}"#,
        },
        ToolDef {
            name: "read_file",
            description: "Read the text content of a file given its absolute path. Returns extracted text (truncated to 4000 characters).",
            parameters_schema: r#"{"type":"object","properties":{"path":{"type":"string","description":"Absolute path to the file"}},"required":["path"]}"#,
        },
        ToolDef {
            name: "list_files",
            description: "List files in the index, optionally filtered by extension and/or filename substring.",
            parameters_schema: r#"{"type":"object","properties":{"extension":{"type":"string","description":"Filter by file extension (e.g. 'pdf', 'txt')"},"name_contains":{"type":"string","description":"Filter filenames containing this substring"}}}"#,
        },
        ToolDef {
            name: "calculate",
            description: "Evaluate a simple arithmetic expression. Supports +, -, *, / and parentheses.",
            parameters_schema: r#"{"type":"object","properties":{"expression":{"type":"string","description":"The arithmetic expression to evaluate"}},"required":["expression"]}"#,
        },
    ]
}

/// Builds a formatted system prompt section describing the available tools.
pub fn tools_prompt() -> String {
    let tools = available_tools();
    let mut prompt = String::from(
        "You have the following tools available. To use a tool, include a tool call in your response using this exact XML format:\n\
         <tool_call>{\"name\": \"tool_name\", \"args\": {\"param\": \"value\"}}</tool_call>\n\n\
         Available tools:\n\n",
    );

    for tool in &tools {
        prompt.push_str(&format!(
            "- **{}**: {}\n  Parameters: {}\n\n",
            tool.name, tool.description, tool.parameters_schema
        ));
    }

    prompt.push_str(
        "When you have enough information to answer the user's question, respond directly without tool calls. \
         Only use tools when you need to look up information you don't already have.\n",
    );

    prompt
}

// ---------------------------------------------------------------------------
// Tool execution
// ---------------------------------------------------------------------------

/// Executes the named tool with the given JSON arguments.
pub fn execute_tool(
    name: &str,
    args: &serde_json::Value,
    search_index: &Arc<Mutex<SearchIndex>>,
    db: &Arc<Mutex<Connection>>,
) -> Result<String> {
    match name {
        "search_documents" => tool_search_documents(args, search_index),
        "read_file" => tool_read_file(args),
        "list_files" => tool_list_files(args, db),
        "calculate" => tool_calculate(args),
        _ => bail!("Unknown tool: {}", name),
    }
}

// ---------------------------------------------------------------------------
// Individual tool implementations
// ---------------------------------------------------------------------------

fn tool_search_documents(
    args: &serde_json::Value,
    search_index: &Arc<Mutex<SearchIndex>>,
) -> Result<String> {
    let query = args["query"]
        .as_str()
        .unwrap_or("")
        .to_string();

    if query.is_empty() {
        bail!("search_documents requires a non-empty 'query' argument");
    }

    let max_results = args["max_results"]
        .as_i64()
        .unwrap_or(5) as usize;

    let index = search_index
        .lock()
        .map_err(|e| anyhow::anyhow!("Failed to lock search index: {}", e))?;

    let hits = index.search(&query, max_results)?;

    if hits.is_empty() {
        return Ok("No documents found matching the query.".to_string());
    }

    let mut output = format!("Found {} result(s):\n\n", hits.len());
    for (i, hit) in hits.iter().enumerate() {
        output.push_str(&format!(
            "{}. **{}** (score: {:.2})\n   {}\n\n",
            i + 1,
            hit.path,
            hit.score,
            hit.snippet
        ));
    }

    Ok(output)
}

fn tool_read_file(args: &serde_json::Value) -> Result<String> {
    let path_str = args["path"]
        .as_str()
        .unwrap_or("");

    if path_str.is_empty() {
        bail!("read_file requires a non-empty 'path' argument");
    }

    let path = Path::new(path_str);
    if !path.exists() {
        bail!("File not found: {}", path_str);
    }

    let result = extractors::extract_file(path)?;
    let text = if result.text.len() > 4000 {
        format!("{}...\n[truncated, {} total characters]", &result.text[..4000], result.text.len())
    } else {
        result.text
    };

    Ok(text)
}

fn tool_list_files(
    args: &serde_json::Value,
    db: &Arc<Mutex<Connection>>,
) -> Result<String> {
    let extension = args["extension"].as_str().unwrap_or("");
    let name_contains = args["name_contains"].as_str().unwrap_or("");

    let conn = db
        .lock()
        .map_err(|e| anyhow::anyhow!("Failed to lock database: {}", e))?;

    let mut conditions = Vec::new();
    let mut param_values: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

    if !extension.is_empty() {
        conditions.push("extension = ?".to_string());
        param_values.push(Box::new(extension.to_string()));
    }

    if !name_contains.is_empty() {
        conditions.push("filename LIKE ?".to_string());
        param_values.push(Box::new(format!("%{}%", name_contains)));
    }

    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!(" WHERE {}", conditions.join(" AND "))
    };

    let sql = format!(
        "SELECT path, filename, extension, size_bytes FROM files{} ORDER BY filename LIMIT 50",
        where_clause
    );

    let mut stmt = conn.prepare(&sql)?;
    let param_refs: Vec<&dyn rusqlite::types::ToSql> =
        param_values.iter().map(|p| p.as_ref()).collect();

    let rows = stmt.query_map(param_refs.as_slice(), |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, i64>(3)?,
        ))
    })?;

    let mut output = String::new();
    let mut count = 0;
    for row in rows {
        let (path, filename, ext, size) = row?;
        output.push_str(&format!("- {} (.{}, {} bytes) — {}\n", filename, ext, size, path));
        count += 1;
    }

    if count == 0 {
        return Ok("No files found matching the criteria.".to_string());
    }

    Ok(format!("Found {} file(s):\n{}", count, output))
}

fn tool_calculate(args: &serde_json::Value) -> Result<String> {
    let expr = args["expression"]
        .as_str()
        .unwrap_or("");

    if expr.is_empty() {
        bail!("calculate requires a non-empty 'expression' argument");
    }

    match eval_arithmetic(expr) {
        Some(result) => Ok(format!("{} = {}", expr, result)),
        None => bail!("Failed to evaluate expression: {}", expr),
    }
}

// ---------------------------------------------------------------------------
// Simple arithmetic evaluator (recursive descent)
// ---------------------------------------------------------------------------

/// Evaluates a simple arithmetic expression supporting +, -, *, /, and parentheses.
fn eval_arithmetic(expr: &str) -> Option<f64> {
    let tokens: Vec<char> = expr.chars().filter(|c| !c.is_whitespace()).collect();
    let mut pos = 0;
    let result = parse_expr(&tokens, &mut pos)?;
    if pos == tokens.len() {
        Some(result)
    } else {
        None // unexpected trailing characters
    }
}

fn parse_expr(tokens: &[char], pos: &mut usize) -> Option<f64> {
    let mut left = parse_term(tokens, pos)?;
    while *pos < tokens.len() {
        match tokens[*pos] {
            '+' => {
                *pos += 1;
                left += parse_term(tokens, pos)?;
            }
            '-' => {
                *pos += 1;
                left -= parse_term(tokens, pos)?;
            }
            _ => break,
        }
    }
    Some(left)
}

fn parse_term(tokens: &[char], pos: &mut usize) -> Option<f64> {
    let mut left = parse_factor(tokens, pos)?;
    while *pos < tokens.len() {
        match tokens[*pos] {
            '*' => {
                *pos += 1;
                left *= parse_factor(tokens, pos)?;
            }
            '/' => {
                *pos += 1;
                let divisor = parse_factor(tokens, pos)?;
                if divisor == 0.0 {
                    return None;
                }
                left /= divisor;
            }
            _ => break,
        }
    }
    Some(left)
}

fn parse_factor(tokens: &[char], pos: &mut usize) -> Option<f64> {
    if *pos >= tokens.len() {
        return None;
    }

    // Handle unary minus
    if tokens[*pos] == '-' {
        *pos += 1;
        let val = parse_factor(tokens, pos)?;
        return Some(-val);
    }

    if tokens[*pos] == '(' {
        *pos += 1;
        let val = parse_expr(tokens, pos)?;
        if *pos < tokens.len() && tokens[*pos] == ')' {
            *pos += 1;
            return Some(val);
        }
        return None; // missing closing paren
    }

    // Parse number
    let start = *pos;
    while *pos < tokens.len() && (tokens[*pos].is_ascii_digit() || tokens[*pos] == '.') {
        *pos += 1;
    }
    if start == *pos {
        return None;
    }
    let num_str: String = tokens[start..*pos].iter().collect();
    num_str.parse::<f64>().ok()
}
