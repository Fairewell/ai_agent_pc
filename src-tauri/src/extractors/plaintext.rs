use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use super::ExtractionResult;

/// Set of extensions treated as plain text.
const PLAINTEXT_EXTENSIONS: &[&str] = &[
    "txt", "md", "csv", "json", "xml", "yaml", "yml", "toml",
    "py", "js", "ts", "html", "css", "rs", "go", "java",
    "c", "cpp", "h", "sh", "bat", "ps1", "cfg", "ini", "log",
];

/// Returns true if this extractor can handle the given extension.
pub fn can_handle(extension: &str) -> bool {
    let ext = extension.to_lowercase();
    PLAINTEXT_EXTENSIONS.contains(&ext.as_str()) || ext == "rtf"
}

/// Extracts text from a plaintext (or RTF) file.
pub fn extract(path: &Path) -> Result<ExtractionResult> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    let raw_bytes = fs::read(path)?;
    let raw_text = String::from_utf8_lossy(&raw_bytes).into_owned();

    let text = if ext == "rtf" {
        strip_rtf(&raw_text)
    } else {
        raw_text
    };

    let mut metadata = HashMap::new();
    metadata.insert("extractor".to_string(), "plaintext".to_string());
    metadata.insert("extension".to_string(), ext);

    Ok(ExtractionResult { text, metadata })
}

/// Very basic RTF stripping: removes control words and braces to extract
/// readable text. Good enough for MVP.
fn strip_rtf(input: &str) -> String {
    // Remove {\rtf1 header and closing brace are handled by brace removal.
    let mut result = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();
    let mut brace_depth: i32 = 0;
    // Track if we are inside a control word / group we should skip.
    // For simplicity we skip content inside {\*...} destinations.
    let mut skip_depth: Option<i32> = None;

    while let Some(ch) = chars.next() {
        match ch {
            '{' => {
                brace_depth += 1;
                // Detect {\* destinations to skip entirely.
                if chars.peek() == Some(&'\\') {
                    // Peek further for '*'
                    let mut lookahead = chars.clone();
                    lookahead.next(); // consume '\'
                    if lookahead.peek() == Some(&'*') {
                        if skip_depth.is_none() {
                            skip_depth = Some(brace_depth);
                        }
                    }
                }
            }
            '}' => {
                if let Some(sd) = skip_depth {
                    if brace_depth == sd {
                        skip_depth = None;
                    }
                }
                brace_depth -= 1;
            }
            '\\' if skip_depth.is_none() => {
                // Control word or symbol.
                if let Some(&next) = chars.peek() {
                    if next == '\'' {
                        // Hex char \'xx — skip two hex digits.
                        chars.next(); // consume '
                        chars.next(); // hex1
                        chars.next(); // hex2
                    } else if next.is_ascii_alphabetic() {
                        // Consume control word.
                        let mut word = String::new();
                        while let Some(&c) = chars.peek() {
                            if c.is_ascii_alphabetic() {
                                word.push(c);
                                chars.next();
                            } else {
                                break;
                            }
                        }
                        // Consume optional numeric parameter.
                        while let Some(&c) = chars.peek() {
                            if c.is_ascii_digit() || c == '-' {
                                chars.next();
                            } else {
                                break;
                            }
                        }
                        // Consume trailing space delimiter.
                        if chars.peek() == Some(&' ') {
                            chars.next();
                        }
                        // Translate common control words.
                        match word.as_str() {
                            "par" | "line" => result.push('\n'),
                            "tab" => result.push('\t'),
                            _ => {}
                        }
                    } else {
                        // Control symbol like \\ \{ \}
                        chars.next();
                        if next == '\\' || next == '{' || next == '}' {
                            result.push(next);
                        }
                    }
                }
            }
            _ if skip_depth.is_some() => {
                // Inside a skipped destination — ignore text.
            }
            '\r' | '\n' => {
                // RTF line breaks are decoration; real breaks come from \par.
            }
            _ => {
                result.push(ch);
            }
        }
    }

    result
}
